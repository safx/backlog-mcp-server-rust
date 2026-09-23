use super::{bridge, request::*};
use crate::access_control::AccessControl;
use backlog_api_client::client::BacklogApiClient;
use backlog_core::identifier::Identifier;
use serde_json::{Value, json};
use std::sync::Arc;
use tokio::sync::Mutex;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path, query_param},
};

const ID: &str = "01939983409c79d5a06a49859789e38f";

fn client(server: &MockServer) -> Arc<Mutex<BacklogApiClient>> {
    Arc::new(Mutex::new(
        BacklogApiClient::new(&server.uri())
            .unwrap()
            .with_api_key("dummy"),
    ))
}

fn access(keys: Option<Vec<&str>>) -> AccessControl {
    AccessControl::for_test(
        keys.map(|keys| keys.into_iter().map(|key| key.parse().unwrap()).collect()),
    )
}

fn document(project: u32) -> Value {
    let mut value: Value = serde_json::from_str(include_str!(
        "../../../crates/backlog-document/tests/fixtures/document.json"
    ))
    .unwrap();
    value["projectId"] = json!(project);
    value
}

fn project(id: u32, key: &str) -> Value {
    json!({"id":id,"projectKey":key,"name":key,"chartEnabled":false,"subtaskingEnabled":false,
        "projectLeaderCanEditProjectLeader":false,"useWiki":false,"useFileSharing":false,
        "useWikiTreeView":false,"useOriginalImageSizeAtWiki":false,"textFormattingRule":"markdown",
        "archived":false,"displayOrder":0,"useDevAttributes":false})
}

async fn project_response(server: &MockServer, lookup: &str, id: u32, key: &str) {
    Mock::given(method("GET"))
        .and(path(format!("/api/v2/projects/{lookup}")))
        .respond_with(ResponseTemplate::new(200).set_body_json(project(id, key)))
        .expect(1)
        .mount(server)
        .await;
}

#[tokio::test]
async fn list_without_restrictions_preserves_requested_scope() {
    for input in [json!({}), json!({"project_ids":[1,2]})] {
        let server = MockServer::start().await;
        Mock::given(path("/api/v2/documents"))
            .and(query_param("offset", "0"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(json!([document(1), document(2)])),
            )
            .expect(1)
            .mount(&server)
            .await;
        bridge::list_documents_bridge(
            client(&server),
            serde_json::from_value(input.clone()).unwrap(),
            &access(None),
        )
        .await
        .unwrap();
        let requests = server.received_requests().await.unwrap();
        assert_eq!(
            requests.len(),
            1,
            "Unrestricted lists must not fetch projects"
        );
        let ids: Vec<_> = requests[0]
            .url
            .query_pairs()
            .filter(|(key, _)| key == "projectId[]")
            .map(|(_, value)| value.parse::<u32>().unwrap())
            .collect();
        assert_eq!(
            ids,
            if input.get("project_ids").is_some() {
                vec![1, 2]
            } else {
                vec![]
            }
        );
    }
}

#[tokio::test]
async fn list_omitted_projects_are_scoped_to_allowed_keys() {
    let server = MockServer::start().await;
    project_response(&server, "TEST", 1, "TEST").await;
    project_response(&server, "SECOND", 2, "SECOND").await;
    Mock::given(path("/api/v2/documents"))
        .and(query_param("projectId[]", "1"))
        .and(query_param("projectId[]", "2"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            document(1),
            document(1),
            document(2),
            document(2)
        ])))
        .expect(2)
        .mount(&server)
        .await;
    let access = access(Some(vec!["TEST", "TEST", "SECOND", "TEST"]));
    for _ in 0..2 {
        let result = bridge::list_documents_bridge(
            client(&server),
            serde_json::from_value(json!({})).unwrap(),
            &access,
        )
        .await
        .unwrap();
        assert_eq!(
            result
                .iter()
                .map(|doc| doc.project_id.value())
                .collect::<Vec<_>>(),
            vec![1, 1, 2, 2]
        );
    }
    let requests = server.received_requests().await.unwrap();
    for request in requests
        .iter()
        .filter(|req| req.url.path() == "/api/v2/documents")
    {
        let ids: Vec<_> = request
            .url
            .query_pairs()
            .filter(|(key, _)| key == "projectId[]")
            .map(|(_, value)| value.into_owned())
            .collect();
        assert_eq!(ids, ["1", "2"]);
    }
}

#[tokio::test]
async fn project_scope_resolves_unique_keys_concurrently_with_a_limit() {
    use std::{collections::HashSet, time::Duration};
    use tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        net::TcpListener,
        sync::{Semaphore, mpsc},
        task::JoinSet,
        time::timeout,
    };

    // Hold every response until eight distinct lookups arrive. This rejects both
    // serial resolution and unbounded fan-out without a wall-clock speed assertion.
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let client = BacklogApiClient::new(&format!("http://{}", listener.local_addr().unwrap()))
        .unwrap()
        .with_api_key("dummy");
    let gate = Arc::new(Semaphore::new(0));
    let server_gate = gate.clone();
    let (sent, mut received) = mpsc::unbounded_channel();
    let mut server_tasks = JoinSet::new();
    server_tasks.spawn(async move {
        let mut connections = JoinSet::new();
        loop {
            let (mut socket, _) = listener.accept().await.unwrap();
            let gate = server_gate.clone();
            let sent = sent.clone();
            connections.spawn(async move {
                let mut request = Vec::new();
                let mut buf = [0; 1024];
                while !request.windows(4).any(|end| end == b"\r\n\r\n") {
                    let size = socket.read(&mut buf).await.unwrap();
                    assert!(size > 0);
                    request.extend_from_slice(&buf[..size]);
                }
                let text = std::str::from_utf8(&request).unwrap();
                let path = text.split_whitespace().nth(1).unwrap().split('?').next().unwrap();
                let key = path.strip_prefix("/api/v2/projects/").unwrap();
                let id = key.strip_prefix("TEST_").unwrap().parse::<u32>().unwrap();
                sent.send(id).unwrap();
                let _permit = gate.acquire().await.unwrap();
                let body = project(id, key).to_string();
                let response = format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len());
                socket.write_all(response.as_bytes()).await.unwrap();
            });
        }
    });
    let keys: Vec<_> = (1..=17)
        .flat_map(|id| [format!("TEST_{id}"), format!("TEST_{id}")])
        .collect();
    let access = access(Some(keys.iter().map(String::as_str).collect()));
    let (result, first_batch) = timeout(Duration::from_secs(10), async {
        tokio::join!(access.scope_document_projects(None, &client), async {
            let mut ids = HashSet::new();
            for _ in 0..8 {
                assert!(
                    ids.insert(received.recv().await.unwrap()),
                    "Duplicate lookup"
                );
            }
            assert!(
                timeout(Duration::from_millis(100), received.recv())
                    .await
                    .is_err(),
                "More than eight lookups in flight"
            );
            gate.add_permits(17);
            ids
        })
    })
    .await
    .expect("Lookups did not run concurrently or failed to complete");
    assert_eq!(
        result
            .unwrap()
            .unwrap()
            .iter()
            .map(|id| id.value())
            .collect::<Vec<_>>(),
        (1..=17).collect::<Vec<_>>()
    );
    let mut all_ids = first_batch;
    while let Ok(id) = received.try_recv() {
        assert!(all_ids.insert(id), "Duplicate lookup");
    }
    assert_eq!(all_ids.len(), 17);
    server_tasks.abort_all();
    while server_tasks.join_next().await.is_some() {}
}

#[tokio::test]
async fn list_explicit_allowed_project_is_checked_before_query() {
    let server = MockServer::start().await;
    project_response(&server, "1", 1, "TEST").await;
    Mock::given(path("/api/v2/documents"))
        .and(query_param("projectId[]", "1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([document(1)])))
        .expect(1)
        .mount(&server)
        .await;
    bridge::list_documents_bridge(
        client(&server),
        serde_json::from_value(json!({"project_ids":[1]})).unwrap(),
        &access(Some(vec!["TEST"])),
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn list_rejects_mixed_projects_without_querying_documents() {
    let server = MockServer::start().await;
    project_response(&server, "1", 1, "TEST").await;
    project_response(&server, "2", 2, "OTHER").await;
    Mock::given(path("/api/v2/documents"))
        .respond_with(ResponseTemplate::new(200))
        .expect(0)
        .mount(&server)
        .await;
    let result = bridge::list_documents_bridge(
        client(&server),
        serde_json::from_value(json!({"project_ids":[1,2]})).unwrap(),
        &access(Some(vec!["TEST"])),
    )
    .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn list_never_falls_back_to_unrestricted_search() {
    for keys in [vec!["MISSING"], vec!["TEST", "MISSING"], vec![]] {
        let server = MockServer::start().await;
        if keys.contains(&"TEST") {
            project_response(&server, "TEST", 1, "TEST").await;
        }
        Mock::given(path("/api/v2/documents"))
            .respond_with(ResponseTemplate::new(200))
            .expect(0)
            .mount(&server)
            .await;
        let result = bridge::list_documents_bridge(
            client(&server),
            serde_json::from_value(json!({})).unwrap(),
            &access(Some(keys)),
        )
        .await;
        assert!(result.is_err());
    }
}

#[tokio::test]
async fn list_rejects_unexpected_forbidden_documents_after_one_query() {
    let server = MockServer::start().await;
    project_response(&server, "TEST", 1, "TEST").await;
    project_response(&server, "2", 2, "OTHER").await;
    Mock::given(path("/api/v2/documents"))
        .and(query_param("projectId[]", "1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            document(1),
            document(1),
            document(2)
        ])))
        .expect(1)
        .mount(&server)
        .await;
    assert!(
        bridge::list_documents_bridge(
            client(&server),
            serde_json::from_value(json!({})).unwrap(),
            &access(Some(vec!["TEST"]))
        )
        .await
        .is_err()
    );
}

#[tokio::test]
async fn invalid_list_and_count_inputs_are_invalid_params_without_http() {
    let server = MockServer::start().await;
    for input in [
        json!({"project_ids":[]}),
        json!({"count":0}),
        json!({"count":101}),
    ] {
        let err = bridge::list_documents_bridge(
            client(&server),
            serde_json::from_value(input).unwrap(),
            &access(Some(vec!["TEST"])),
        )
        .await
        .unwrap_err();
        assert!(matches!(err, crate::error::Error::Parameter(_)));
    }
    let err = bridge::get_document_count_bridge(
        client(&server),
        GetDocumentCountRequest {
            project_id_or_key: " ".into(),
        },
        &access(None),
    )
    .await
    .unwrap_err();
    assert!(matches!(err, crate::error::Error::Parameter(_)));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn count_checks_key_and_numeric_id_before_calling_api() {
    for (input, allowed) in [("TEST", true), ("OTHER", false), ("1", true), ("2", false)] {
        let server = MockServer::start().await;
        if let Ok(id) = input.parse::<u32>() {
            project_response(&server, input, id, if allowed { "TEST" } else { "OTHER" }).await;
        }
        Mock::given(path("/api/v2/documents/count"))
            .and(query_param("projectIdOrKey", input))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"count":11})))
            .expect(if allowed { 1 } else { 0 })
            .mount(&server)
            .await;
        let result = bridge::get_document_count_bridge(
            client(&server),
            GetDocumentCountRequest {
                project_id_or_key: input.into(),
            },
            &access(Some(vec!["TEST"])),
        )
        .await;
        assert_eq!(result.is_ok(), allowed);
    }
}

#[cfg(feature = "document_writable")]
#[tokio::test]
async fn tags_skip_document_fetch_when_access_control_is_disabled() {
    for remove in [false, true] {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(format!("/api/v2/documents/{ID}")))
            .respond_with(ResponseTemplate::new(404))
            .expect(0)
            .mount(&server)
            .await;
        Mock::given(method(if remove { "DELETE" } else { "POST" }))
            .and(path(format!("/api/v2/documents/{ID}/tags")))
            .respond_with(if remove {
                ResponseTemplate::new(204)
            } else {
                ResponseTemplate::new(200).set_body_json(json!([{"id":11,"name":"tag"}]))
            })
            .expect(1)
            .mount(&server)
            .await;
        let req = DocumentTagsRequest {
            document_id: ID.into(),
            tag_names: vec!["tag".into()],
        };
        if remove {
            bridge::remove_document_tag_bridge(client(&server), req, &access(None))
                .await
                .unwrap();
        } else {
            bridge::add_document_tag_bridge(client(&server), req, &access(None))
                .await
                .unwrap();
        }
        assert_eq!(server.received_requests().await.unwrap().len(), 1);
    }
}

#[cfg(feature = "document_writable")]
#[tokio::test]
async fn tags_validate_input_before_getting_document() {
    let server = MockServer::start().await;
    for (id, names) in [("invalid", vec!["tag"]), (ID, vec![]), (ID, vec![" "])] {
        for remove in [false, true] {
            let req = DocumentTagsRequest {
                document_id: id.into(),
                tag_names: names.iter().map(|v| (*v).into()).collect(),
            };
            let result = if remove {
                bridge::remove_document_tag_bridge(client(&server), req, &access(None)).await
            } else {
                bridge::add_document_tag_bridge(client(&server), req, &access(None))
                    .await
                    .map(|_| ())
            };
            assert!(matches!(result, Err(crate::error::Error::Parameter(_))));
        }
    }
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[cfg(feature = "document_writable")]
#[tokio::test]
async fn tags_require_successful_detail_and_access_check_before_mutation() {
    for remove in [false, true] {
        for (detail_status, allowed) in [(200, true), (200, false), (404, false)] {
            let server = MockServer::start().await;
            Mock::given(method("GET"))
                .and(path(format!("/api/v2/documents/{ID}")))
                .respond_with(ResponseTemplate::new(detail_status).set_body_json(document(1)))
                .expect(1)
                .mount(&server)
                .await;
            if detail_status == 200 {
                project_response(&server, "1", 1, if allowed { "TEST" } else { "OTHER" }).await;
            }
            Mock::given(method(if remove { "DELETE" } else { "POST" }))
                .and(path(format!("/api/v2/documents/{ID}/tags")))
                .respond_with(if remove {
                    ResponseTemplate::new(204)
                } else {
                    ResponseTemplate::new(200).set_body_json(json!([{"id":11,"name":"tag"}]))
                })
                .expect(if allowed { 1 } else { 0 })
                .mount(&server)
                .await;
            let req = DocumentTagsRequest {
                document_id: format!(" {ID} "),
                tag_names: vec!["tag".into()],
            };
            let result = if remove {
                bridge::remove_document_tag_bridge(
                    client(&server),
                    req,
                    &access(Some(vec!["TEST"])),
                )
                .await
            } else {
                bridge::add_document_tag_bridge(client(&server), req, &access(Some(vec!["TEST"])))
                    .await
                    .map(|_| ())
            };
            assert_eq!(result.is_ok(), allowed);
        }
    }
}
