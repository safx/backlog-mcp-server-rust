mod common;

use backlog_document::{GetDocumentCountParams, ListDocumentsParamsBuilder};
use common::setup_document_api;
use serde_json::json;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn count_accepts_keys_and_numeric_strings_including_zero_results() {
    for (project, count) in [("TEST", 11), ("123", 0)] {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v2/documents/count"))
            .and(query_param("projectIdOrKey", project))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"count": count})))
            .expect(1)
            .mount(&server)
            .await;
        let params =
            GetDocumentCountParams::new(project.parse::<backlog_core::ProjectIdOrKey>().unwrap());
        let result = setup_document_api(&server)
            .await
            .get_document_count(params)
            .await
            .unwrap();
        assert_eq!(result.count, count);
    }
}

#[tokio::test]
async fn count_propagates_api_errors() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v2/documents/count"))
        .respond_with(
            ResponseTemplate::new(403)
                .set_body_json(json!({"errors":[{"message":"Denied", "code":5}]})),
        )
        .mount(&server)
        .await;
    let result = setup_document_api(&server)
        .await
        .get_document_count(GetDocumentCountParams::new(
            "TEST".parse::<backlog_core::ProjectIdOrKey>().unwrap(),
        ))
        .await;
    assert!(matches!(
        result,
        Err(backlog_api_core::Error::HttpStatus { status: 403, .. })
    ));
}

#[tokio::test]
async fn list_supplies_offset_and_accepts_count_boundaries() {
    for count in [None, Some(1), Some(100)] {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v2/documents"))
            .and(query_param("offset", "0"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!([])))
            .expect(1)
            .mount(&server)
            .await;
        let mut params = ListDocumentsParamsBuilder::default().build().unwrap();
        params.offset = None;
        params.count = count;
        assert!(
            setup_document_api(&server)
                .await
                .list_documents(params)
                .await
                .unwrap()
                .is_empty()
        );
        let requests = server.received_requests().await.unwrap();
        let query: Vec<_> = requests[0].url.query_pairs().collect();
        assert!(!query.iter().any(|(key, _)| key == "projectId[]"));
        assert_eq!(
            query
                .iter()
                .find(|(key, _)| key == "count")
                .map(|(_, value)| value.parse::<u32>().unwrap()),
            count
        );
    }
}

#[tokio::test]
async fn invalid_list_filters_do_not_send_requests() {
    let server = MockServer::start().await;
    let api = setup_document_api(&server).await;
    for count in [0, 101] {
        let params = ListDocumentsParamsBuilder::default()
            .count(count)
            .build()
            .unwrap();
        assert!(api.list_documents(params).await.is_err());
    }
    let params = ListDocumentsParamsBuilder::default()
        .project_ids(vec![])
        .build()
        .unwrap();
    assert!(api.list_documents(params).await.is_err());
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[test]
fn list_model_preserves_json_and_attachments_and_accepts_older_responses() {
    let user = json!({"id":1,"userId":"test","name":"Test","roleType":1,"lang":"ja","mailAddress":"test@example.com","nulabAccount":null,"keyword":null,"lastLoginTime":null});
    let mut body = json!({
        "id":"01939983409c79d5a06a49859789e38f", "projectId":1, "title":"Document", "plain":"text", "statusId":1,
        "createdUser":user, "updatedUser":user, "created":"2026-01-01T00:00:00Z", "updated":"2026-01-01T00:00:00Z",
        "json":"{\"type\":\"doc\"}", "tags":[{"id":11,"name":"設計"}],
        "attachments":[{"id":2,"name":"test.txt","size":4,"createdUser":user,"created":"2026-01-01T00:00:00Z"}]
    });
    let doc: backlog_document::Document = serde_json::from_value(body.clone()).unwrap();
    let serialized = serde_json::to_value(doc).unwrap();
    for field in ["json", "tags"] {
        assert_eq!(serialized[field], body[field]);
    }
    assert_eq!(serialized["attachments"].as_array().unwrap().len(), 1);
    for field in ["id", "name", "size", "created"] {
        assert_eq!(
            serialized["attachments"][0][field],
            body["attachments"][0][field]
        );
    }
    assert_eq!(serialized["attachments"][0]["createdUser"]["id"], json!(1));
    body.as_object_mut().unwrap().remove("json");
    body.as_object_mut().unwrap().remove("attachments");
    let doc: backlog_document::Document = serde_json::from_value(body).unwrap();
    assert!(doc.json.is_none());
    assert!(doc.attachments.is_empty());
}

#[cfg(feature = "writable")]
mod tags {
    use super::*;
    use backlog_document::{AddDocumentTagParams, RemoveDocumentTagParams};
    use wiremock::matchers::header;

    const ID: &str = "01939983409c79d5a06a49859789e38f";

    #[tokio::test]
    async fn tags_use_repeated_form_fields_and_remove_accepts_empty_204() {
        let server = MockServer::start().await;
        let names = vec![
            " 設計 + & , ".to_string(),
            "要確認".to_string(),
            "要確認".to_string(),
        ];
        let response = json!([{"id":11,"name":names[0]},{"id":12,"name":names[1]}]);
        for (verb, template) in [
            (
                "POST",
                ResponseTemplate::new(200).set_body_json(response.clone()),
            ),
            ("DELETE", ResponseTemplate::new(204)),
        ] {
            Mock::given(method(verb))
                .and(path(format!("/api/v2/documents/{ID}/tags")))
                .and(header("content-type", "application/x-www-form-urlencoded"))
                .respond_with(template)
                .expect(1)
                .mount(&server)
                .await;
        }
        let api = setup_document_api(&server).await;
        let result = api
            .add_document_tag(AddDocumentTagParams::new(
                ID.parse().unwrap(),
                names.clone(),
            ))
            .await
            .unwrap();
        assert_eq!(serde_json::to_value(result).unwrap(), response);
        api.remove_document_tag(RemoveDocumentTagParams::new(
            ID.parse().unwrap(),
            names.clone(),
        ))
        .await
        .unwrap();
        for request in server.received_requests().await.unwrap() {
            let form: Vec<_> = url::form_urlencoded::parse(&request.body)
                .into_owned()
                .collect();
            assert_eq!(
                form,
                names
                    .iter()
                    .map(|name| ("tagNames[]".to_string(), name.clone()))
                    .collect::<Vec<_>>()
            );
            assert!(
                !request
                    .url
                    .query_pairs()
                    .any(|(key, _)| key == "tagNames[]")
            );
        }
    }

    #[tokio::test]
    async fn invalid_tags_do_not_send_requests() {
        let server = MockServer::start().await;
        let api = setup_document_api(&server).await;
        for names in [
            vec![],
            vec![String::new()],
            vec![" \t\n".to_string()],
            vec!["ok".to_string(), String::new()],
        ] {
            assert!(
                api.add_document_tag(AddDocumentTagParams::new(
                    ID.parse().unwrap(),
                    names.clone()
                ))
                .await
                .is_err()
            );
            assert!(
                api.remove_document_tag(RemoveDocumentTagParams::new(ID.parse().unwrap(), names))
                    .await
                    .is_err()
            );
        }
        assert!(server.received_requests().await.unwrap().is_empty());
    }

    #[tokio::test]
    async fn tag_errors_and_unexpected_delete_status_are_not_success() {
        for status in [400, 401, 403, 404, 500] {
            let server = MockServer::start().await;
            Mock::given(path(format!("/api/v2/documents/{ID}/tags")))
                .respond_with(
                    ResponseTemplate::new(status)
                        .set_body_json(json!({"errors":[{"message":"Error", "code":1}]})),
                )
                .expect(2)
                .mount(&server)
                .await;
            let api = setup_document_api(&server).await;
            assert!(
                api.add_document_tag(AddDocumentTagParams::new(
                    ID.parse().unwrap(),
                    vec!["tag".into()]
                ))
                .await
                .is_err()
            );
            assert!(
                api.remove_document_tag(RemoveDocumentTagParams::new(
                    ID.parse().unwrap(),
                    vec!["tag".into()]
                ))
                .await
                .is_err()
            );
        }
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!([])))
            .mount(&server)
            .await;
        let result = setup_document_api(&server)
            .await
            .remove_document_tag(RemoveDocumentTagParams::new(
                ID.parse().unwrap(),
                vec!["tag".into()],
            ))
            .await;
        assert!(matches!(
            result,
            Err(backlog_api_core::Error::UnexpectedStatus { status: 200, .. })
        ));
    }
}
