mod common;

#[cfg(test)]
mod user_recent_updates_tests {
    use super::common::setup_user_api;
    use backlog_api_core::IntoRequest;
    use backlog_core::identifier::{ActivityTypeId, Identifier, UserId};
    use backlog_user::GetUserRecentUpdatesParams;
    use serde_json::json;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_get_user_recent_updates_minimal() {
        let mock_server = MockServer::start().await;
        let user_id = UserId::from(12345);

        Mock::given(method("GET"))
            .and(path("/api/v2/users/12345/activities"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!([
                {
                    "id": 123,
                    "project": {
                        "id": 101,
                        "projectKey": "TEST",
                        "name": "Test Project",
                        "chartEnabled": false,
                        "subtaskingEnabled": false,
                        "projectLeaderCanEditProjectLeader": false,
                        "useWikiTreeView": false,
                        "textFormattingRule": "backlog",
                        "archived": false,
                        "displayOrder": 0,
                        "useDevAttributes": true,
                        "useWiki": true,
                        "useFileSharing": true,
                        "useOriginalImageSizeAtWiki": false
                    },
                    "type": 1,
                    "content": {
                        "id": 456,
                        "key_id": 789,
                        "summary": "Test issue",
                        "description": "Test description"
                    },
                    "created": "2024-01-01T10:00:00Z",
                    "notifications": [],
                    "createdUser": {
                        "id": 12345,
                        "userId": "testuser",
                        "name": "Test User",
                        "roleType": 2,
                        "lang": "ja",
                        "mailAddress": "test@example.com",
                        "nulabAccount": {
                            "nulabId": "nulabtest",
                            "name": "Test Nulab User",
                            "uniqueId": "unique123"
                        },
                        "keyword": "test keyword"
                    }
                }
            ])))
            .mount(&mock_server)
            .await;

        let api = setup_user_api(&mock_server).await;

        let params = GetUserRecentUpdatesParams {
            user_id,
            activity_type_ids: None,
            min_id: None,
            max_id: None,
            count: None,
            order: None,
        };

        let result = api.get_user_recent_updates(params).await;
        assert!(
            result.is_ok(),
            "Failed to get user recent updates: {result:?}"
        );

        let activities = result.expect("get_user_recent_updates should succeed");
        assert_eq!(activities.len(), 1);
        assert_eq!(activities[0].id.value(), 123);
        // Use helper method to access project id
        assert_eq!(activities[0].project_id(), Some(101));
    }

    #[tokio::test]
    async fn test_get_user_recent_updates_with_all_params() {
        let mock_server = MockServer::start().await;
        let user_id = UserId::from(12345);

        Mock::given(method("GET"))
            .and(path("/api/v2/users/12345/activities"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!([])))
            .mount(&mock_server)
            .await;

        let api = setup_user_api(&mock_server).await;

        let params = GetUserRecentUpdatesParams {
            user_id,
            activity_type_ids: Some(vec![ActivityTypeId::from(1), ActivityTypeId::from(2)]),
            min_id: Some(100),
            max_id: Some(200),
            count: Some(50),
            order: Some("asc".to_string()),
        };

        let result = api.get_user_recent_updates(params).await;
        if let Err(e) = &result {
            eprintln!("Error in test_get_user_recent_updates_with_all_params: {e:?}");
        }
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_user_recent_updates_path() {
        let user_id = UserId::from(12345);
        let params = GetUserRecentUpdatesParams {
            user_id,
            activity_type_ids: None,
            min_id: None,
            max_id: None,
            count: None,
            order: None,
        };

        assert_eq!(params.path(), "/api/v2/users/12345/activities");
    }

    #[tokio::test]
    async fn test_get_user_recent_updates_error() {
        let mock_server = MockServer::start().await;
        let user_id = UserId::from(99999);

        Mock::given(method("GET"))
            .and(path("/api/v2/users/99999/activities"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({
                "errors": [{
                    "message": "No such user",
                    "code": 6,
                    "moreInfo": ""
                }]
            })))
            .mount(&mock_server)
            .await;

        let api = setup_user_api(&mock_server).await;

        let params = GetUserRecentUpdatesParams {
            user_id,
            activity_type_ids: None,
            min_id: None,
            max_id: None,
            count: None,
            order: None,
        };

        let result = api.get_user_recent_updates(params).await;
        assert!(result.is_err());

        if let Err(e) = result {
            assert!(e.to_string().contains("No such user"));
        }
    }
}
