#![cfg(feature = "document")]

use serde_json::{Value, json};
use std::{
    process::{Output, Stdio},
    time::Duration,
};
use tokio::{io::AsyncReadExt, process::Command, time::timeout};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path},
};

const ID: &str = "01939983409c79d5a06a49859789e38f";

async fn run(server: &MockServer, args: &[&str]) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_blg"))
        .args(args)
        .env("BACKLOG_BASE_URL", server.uri())
        .env("BACKLOG_API_KEY", "dummy")
        .env_remove("BACKLOG_PROJECTS")
        .env_remove("BACKLOG_PREFIX")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .unwrap();
    let mut stdout = child.stdout.take().unwrap();
    let mut stderr = child.stderr.take().unwrap();
    let outcome = timeout(Duration::from_secs(30), async {
        let mut out = Vec::new();
        let mut err = Vec::new();
        let (status, _, _) = tokio::try_join!(
            child.wait(),
            stdout.read_to_end(&mut out),
            stderr.read_to_end(&mut err)
        )?;
        Ok::<_, std::io::Error>(Output {
            status,
            stdout: out,
            stderr: err,
        })
    })
    .await;
    if !matches!(&outcome, Ok(Ok(_))) {
        let _ = child.kill().await;
        let _ = child.wait().await;
    }
    outcome.expect("CLI timeout").expect("CLI I/O")
}

fn success_json(output: Output) -> Value {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).expect("stdout must contain only JSON")
}

#[tokio::test]
async fn list_supports_existing_multiple_and_omitted_ids() {
    let document: Value = serde_json::from_str(include_str!(
        "../../crates/backlog-document/tests/fixtures/document.json"
    ))
    .unwrap();
    for ids in [vec!["1"], vec!["1", "2"], vec![]] {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v2/documents"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!([document])))
            .expect(1)
            .mount(&server)
            .await;
        let mut args = vec![
            "document",
            "list",
            "--keyword",
            "設計",
            "--sort",
            "updated",
            "--order",
            "asc",
            "--json",
        ];
        for id in &ids {
            args.extend(["--project-id", id]);
        }
        let body = success_json(run(&server, &args).await);
        assert_eq!(body[0]["json"], document["json"]);
        assert_eq!(body[0]["attachments"], json!([]));
        assert_eq!(body[0]["tags"], document["tags"]);
        let requests = server.received_requests().await.unwrap();
        let query: Vec<_> = requests[0].url.query_pairs().collect();
        assert_eq!(
            query
                .iter()
                .filter(|(key, _)| key == "projectId[]")
                .map(|(_, v)| v.as_ref())
                .collect::<Vec<_>>(),
            ids
        );
        for (key, value) in [
            ("keyword", "設計"),
            ("sort", "updated"),
            ("order", "asc"),
            ("offset", "0"),
        ] {
            assert!(query.iter().any(|(k, v)| k == key && v == value));
        }
    }
}

#[tokio::test]
async fn list_text_shows_complete_ids_and_page_count() {
    let server = MockServer::start().await;
    let document: Value = serde_json::from_str(include_str!(
        "../../crates/backlog-document/tests/fixtures/document.json"
    ))
    .unwrap();
    Mock::given(path("/api/v2/documents"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([document])))
        .mount(&server)
        .await;
    let output = run(&server, &["document", "list", "-p", "1"]).await;
    assert!(output.status.success());
    let text = String::from_utf8(output.stdout).unwrap();
    assert!(text.contains(ID));
    assert!(text.contains("Project ID: 1"));
    assert!(!text.contains("total"));
}

#[tokio::test]
async fn count_supports_id_and_key_and_api_failure() {
    for project in ["123", "TEST"] {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v2/documents/count"))
            .and(wiremock::matchers::query_param("projectIdOrKey", project))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"count":0})))
            .expect(1)
            .mount(&server)
            .await;
        assert_eq!(
            success_json(
                run(
                    &server,
                    &["document", "count", "--project-id", project, "--json"]
                )
                .await
            ),
            json!({"count":0})
        );
    }
    let server = MockServer::start().await;
    Mock::given(path("/api/v2/documents/count"))
        .respond_with(
            ResponseTemplate::new(403)
                .set_body_json(json!({"errors":[{"message":"Denied","code":5}]})),
        )
        .mount(&server)
        .await;
    let output = run(&server, &["document", "count", "-p", "TEST", "--json"]).await;
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
}

#[tokio::test]
async fn invalid_inputs_are_rejected_before_http() {
    let server = MockServer::start().await;
    for args in [
        vec!["document", "list", "--sort", "title"],
        vec!["document", "list", "--order", "wrong"],
        vec!["document", "list", "--count", "0"],
        vec!["document", "list", "--count", "101"],
        vec!["document", "list", "--offset", "-1"],
        vec!["document", "list", "-p", "TEST"],
        vec!["document", "count"],
    ] {
        assert!(!run(&server, &args).await.status.success(), "{args:?}");
    }
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn tag_commands_follow_feature_flag() {
    let server = MockServer::start().await;
    let output = run(&server, &["document", "--help"]).await;
    assert!(output.status.success());
    let help = String::from_utf8(output.stdout).unwrap();
    assert!(help.contains("count"));
    for name in ["tag-add", "tag-remove"] {
        assert_eq!(help.contains(name), cfg!(feature = "document_writable"));
        let output = run(&server, &["document", name, ID, "--tag-name", " "]).await;
        assert!(!output.status.success());
    }
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[cfg(feature = "document_writable")]
#[tokio::test]
async fn tags_preserve_repeated_names_and_remove_returns_success_json() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(format!("/api/v2/documents/{ID}/tags")))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!([{"id":1,"name":"設計,仕様"}])),
        )
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("DELETE"))
        .and(path(format!("/api/v2/documents/{ID}/tags")))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;
    for command in ["tag-add", "tag-remove"] {
        let result = success_json(
            run(
                &server,
                &[
                    "document",
                    command,
                    ID,
                    "--tag-name",
                    "設計,仕様",
                    "--tag-name",
                    "要確認",
                    "--json",
                ],
            )
            .await,
        );
        assert_eq!(
            result,
            if command == "tag-add" {
                json!([{"id":1,"name":"設計,仕様"}])
            } else {
                json!({"success":true})
            }
        );
    }
    let requests = server.received_requests().await.unwrap();
    for req in requests {
        assert_eq!(
            String::from_utf8(req.body)
                .unwrap()
                .matches("tagNames%5B%5D=")
                .count(),
            2
        );
    }
}
