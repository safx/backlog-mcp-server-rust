use serde_json::{Value, json};
use std::{process::Stdio, time::Duration};
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    process::Command,
    time::timeout,
};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path, query_param},
};

const ID: &str = "01939983409c79d5a06a49859789e38f";

async fn exchange(server: &MockServer, prefix: &str, requests: Vec<Value>) -> Vec<Value> {
    let mut child = Command::new(env!("CARGO_BIN_EXE_mcp-backlog-server"))
        .env("BACKLOG_BASE_URL", server.uri())
        .env("BACKLOG_API_KEY", "dummy")
        .env("BACKLOG_PREFIX", prefix)
        .env_remove("BACKLOG_PROJECTS")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .unwrap();
    let mut stdin = child.stdin.take().unwrap();
    let mut lines = BufReader::new(child.stdout.take().unwrap()).lines();
    let mut stderr = child.stderr.take().unwrap();
    let stderr_task = tokio::spawn(async move {
        let mut bytes = Vec::new();
        stderr.read_to_end(&mut bytes).await.map(|_| bytes)
    });
    let expected = requests.len() + 1;
    let outcome = timeout(Duration::from_secs(30), async {
        let mut messages = vec![
            json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"document-test","version":"1"}}}),
            json!({"jsonrpc":"2.0","method":"notifications/initialized"}),
        ];
        messages.extend(requests);
        for message in messages { stdin.write_all(format!("{message}\n").as_bytes()).await?; }
        let mut replies = Vec::new();
        while replies.len() < expected {
            let line = lines.next_line().await?.ok_or_else(|| std::io::Error::other("MCP closed stdout before replying"))?;
            let reply: Value = serde_json::from_str(&line)?;
            if reply.get("id").is_some() { replies.push(reply); }
        }
        replies.sort_by_key(|reply| reply["id"].as_u64());
        Ok::<_, std::io::Error>(replies)
    }).await;
    drop(stdin);
    let _ = child.kill().await;
    let _ = child.wait().await;
    let stderr = timeout(Duration::from_secs(5), stderr_task)
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    outcome
        .expect("MCP timeout")
        .unwrap_or_else(|error| panic!("{error}: {}", String::from_utf8_lossy(&stderr)))
}

fn call(id: u32, name: &str, arguments: Value) -> Value {
    json!({"jsonrpc":"2.0","id":id,"method":"tools/call","params":{"name":name,"arguments":arguments}})
}

fn content(reply: &Value) -> Value {
    assert!(reply.get("error").is_none(), "{reply}");
    assert_ne!(reply["result"]["isError"], json!(true), "{reply}");
    serde_json::from_str(reply["result"]["content"][0]["text"].as_str().unwrap()).unwrap()
}

#[tokio::test]
async fn tool_registration_schema_and_feature_flags() {
    let server = MockServer::start().await;
    for prefix in ["test_", ""] {
        let response = exchange(
            &server,
            prefix,
            vec![json!({"jsonrpc":"2.0","id":2,"method":"tools/list"})],
        )
        .await;
        let tools = response[1]["result"]["tools"].as_array().unwrap();
        let lookup = |name: &str| {
            tools
                .iter()
                .find(|tool| tool["name"] == format!("{prefix}{name}"))
        };
        for name in ["document_list_get", "document_count_get"] {
            assert!(lookup(name).is_some());
        }
        for name in [
            "document_tag_add",
            "document_tag_remove",
            "document_add",
            "document_delete",
        ] {
            assert_eq!(
                lookup(name).is_some(),
                cfg!(feature = "document_writable"),
                "{name}"
            );
        }
        let list_schema = &lookup("document_list_get").unwrap()["inputSchema"];
        assert_eq!(list_schema["properties"]["count"]["minimum"], json!(1));
        assert_eq!(list_schema["properties"]["count"]["maximum"], json!(100));
        assert_eq!(
            list_schema["properties"]["project_ids"]["minItems"],
            json!(1)
        );
        let schema_text = serde_json::to_string(list_schema).unwrap();
        for value in ["created", "updated", "asc", "desc"] {
            assert!(schema_text.contains(&format!("\"{value}\"")));
        }
        assert_eq!(
            lookup("document_count_get").unwrap()["inputSchema"]["required"],
            json!(["project_id_or_key"])
        );
        #[cfg(feature = "document_writable")]
        {
            let schema = &lookup("document_tag_add").unwrap()["inputSchema"];
            assert_eq!(schema["properties"]["tag_names"]["minItems"], json!(1));
            let required = schema["required"].as_array().unwrap();
            assert!(
                required.contains(&json!("document_id")) && required.contains(&json!("tag_names"))
            );
        }
    }
}

#[tokio::test]
async fn read_tools_call_the_api_and_return_json() {
    let server = MockServer::start().await;
    let document: Value = serde_json::from_str(include_str!(
        "../../crates/backlog-document/tests/fixtures/document.json"
    ))
    .unwrap();
    Mock::given(method("GET"))
        .and(path("/api/v2/documents"))
        .and(query_param("offset", "0"))
        .and(query_param("keyword", "設計"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([document])))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/api/v2/documents/count"))
        .and(query_param("projectIdOrKey", "123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"count":11})))
        .expect(1)
        .mount(&server)
        .await;
    let responses = exchange(
        &server,
        "test_",
        vec![
            call(2, "test_document_list_get", json!({"keyword":"設計"})),
            call(
                3,
                "test_document_count_get",
                json!({"project_id_or_key":"123"}),
            ),
            call(4, "test_document_list_get", json!({"count":0})),
        ],
    )
    .await;
    assert_eq!(content(&responses[1])[0]["id"], json!(ID));
    assert_eq!(content(&responses[1])[0]["json"], document["json"]);
    assert_eq!(content(&responses[2]), json!({"count":11}));
    assert_eq!(responses[3]["error"]["code"], json!(-32602));
}

#[cfg(feature = "document_writable")]
#[tokio::test]
async fn tag_tools_handle_tag_arrays_and_empty_204() {
    let server = MockServer::start().await;
    let document: Value = serde_json::from_str(include_str!(
        "../../crates/backlog-document/tests/fixtures/document.json"
    ))
    .unwrap();
    Mock::given(method("GET"))
        .and(path(format!("/api/v2/documents/{ID}")))
        .respond_with(ResponseTemplate::new(200).set_body_json(document))
        .expect(0)
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path(format!("/api/v2/documents/{ID}/tags")))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([{"id":11,"name":"設計"}])))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("DELETE"))
        .and(path(format!("/api/v2/documents/{ID}/tags")))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;
    let args = json!({"document_id":ID,"tag_names":["設計","要確認"]});
    let replies = exchange(
        &server,
        "test_",
        vec![
            call(2, "test_document_tag_add", args.clone()),
            call(3, "test_document_tag_remove", args),
        ],
    )
    .await;
    assert_eq!(content(&replies[1]), json!([{"id":11,"name":"設計"}]));
    assert_eq!(content(&replies[2]), json!({"success":true}));
    let requests = server.received_requests().await.unwrap();
    for req in requests
        .iter()
        .filter(|req| req.url.path().ends_with("/tags"))
    {
        assert_eq!(
            String::from_utf8_lossy(&req.body)
                .matches("tagNames%5B%5D=")
                .count(),
            2
        );
    }
}

#[cfg(not(feature = "document_writable"))]
#[tokio::test]
async fn disabled_tag_tools_cannot_be_called() {
    let server = MockServer::start().await;
    let replies = exchange(
        &server,
        "test_",
        vec![
            call(
                2,
                "test_document_tag_add",
                json!({"document_id":ID,"tag_names":["tag"]}),
            ),
            call(
                3,
                "test_document_tag_remove",
                json!({"document_id":ID,"tag_names":["tag"]}),
            ),
        ],
    )
    .await;
    for reply in &replies[1..] {
        assert!(reply.get("error").is_some(), "{reply}");
    }
    assert!(server.received_requests().await.unwrap().is_empty());
}
