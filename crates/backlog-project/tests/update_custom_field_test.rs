#[cfg(feature = "writable")]
mod update_custom_field_tests {
    use backlog_api_core::Error as ApiError;
    use backlog_core::{
        Date, ProjectKey,
        identifier::{CustomFieldId, IssueTypeId, ProjectId},
    };
    use backlog_domain_models::{CustomFieldSettings, InitialDate};
    use backlog_project::api::{ProjectApi, UpdateCustomFieldParams};
    use chrono::NaiveDate;
    use client::test_utils::setup_client;
    use std::str::FromStr;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_update_custom_field_basic_fields() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let expected_response = serde_json::json!({
            "id": 12345,
            "projectId": 123,
            "typeId": 1,
            "name": "Updated Field Name",
            "description": "Updated description",
            "required": true,
            "useIssueType": true,
            "applicableIssueTypes": [1, 2],
            "displayOrder": 1
        });

        Mock::given(method("PATCH"))
            .and(path("/api/v2/projects/TEST_PROJECT/customFields/12345"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_response))
            .mount(&mock_server)
            .await;

        let params = UpdateCustomFieldParams::new(
            ProjectKey::from_str("TEST_PROJECT").unwrap(),
            CustomFieldId::new(12345),
        )
        .with_name("Updated Field Name")
        .with_description("Updated description")
        .with_required(true)
        .with_applicable_issue_types(vec![IssueTypeId::new(1), IssueTypeId::new(2)]);

        let result = project_api.update_custom_field(params).await;

        if let Err(e) = &result {
            eprintln!("Error in test_update_custom_field_basic_fields: {e:?}");
        }

        assert!(result.is_ok());
        let custom_field = result.unwrap();
        assert_eq!(custom_field.name, "Updated Field Name");
        assert_eq!(custom_field.description, "Updated description");
        assert!(custom_field.required);
        assert_eq!(
            custom_field.applicable_issue_types,
            Some(vec![IssueTypeId::new(1), IssueTypeId::new(2)])
        );
        assert!(matches!(custom_field.settings, CustomFieldSettings::Text));
    }

    #[tokio::test]
    async fn test_update_custom_field_numeric_settings() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let expected_response = serde_json::json!({
            "id": 12345,
            "projectId": 123,
            "typeId": 3,
            "name": "Score Field",
            "description": "",
            "required": false,
            "useIssueType": false,
            "applicableIssueTypes": [],
            "displayOrder": 2,
            "min": 0.0,
            "max": 100.0,
            "initialValue": 50.0,
            "unit": "points"
        });

        Mock::given(method("PATCH"))
            .and(path("/api/v2/projects/123/customFields/12345"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_response))
            .mount(&mock_server)
            .await;

        let params = UpdateCustomFieldParams::new(ProjectId::new(123), CustomFieldId::new(12345))
            .with_numeric_settings(
                Some(0.0),
                Some(100.0),
                Some(50.0),
                Some("points".to_string()),
            );

        let result = project_api.update_custom_field(params).await;
        assert!(result.is_ok());
        let custom_field = result.unwrap();
        if let CustomFieldSettings::Numeric(settings) = custom_field.settings {
            assert_eq!(settings.min, Some(0.0));
            assert_eq!(settings.max, Some(100.0));
            assert_eq!(settings.initial_value, Some(50.0));
            assert_eq!(settings.unit, Some("points".to_string()));
        } else {
            panic!("Expected numeric settings");
        }
    }

    #[tokio::test]
    async fn test_update_custom_field_date_settings() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let min_date = Date::from(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap());
        let max_date = Date::from(NaiveDate::from_ymd_opt(2025, 12, 31).unwrap());
        let initial_date = Date::from(NaiveDate::from_ymd_opt(2025, 6, 15).unwrap());

        let expected_response = serde_json::json!({
            "id": 12345,
            "projectId": 123,
            "typeId": 4,
            "name": "Deadline Field",
            "description": "",
            "required": false,
            "useIssueType": false,
            "applicableIssueTypes": [],
            "displayOrder": 3,
            "min": "2025-01-01T00:00:00Z",
            "max": "2025-12-31T00:00:00Z",
            "initialDate": {
                "id": 1,
                "date": "2025-06-15T00:00:00Z"
            }
        });

        Mock::given(method("PATCH"))
            .and(path("/api/v2/projects/123/customFields/12345"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_response))
            .mount(&mock_server)
            .await;

        let params = UpdateCustomFieldParams::new(ProjectId::new(123), CustomFieldId::new(12345))
            .with_date_settings(
                Some(min_date),
                Some(max_date),
                Some(1),
                Some(initial_date),
                Some(7),
            );

        let result = project_api.update_custom_field(params).await;
        assert!(result.is_ok());
        let custom_field = result.unwrap();
        if let CustomFieldSettings::Date(settings) = custom_field.settings {
            assert_eq!(
                settings.min,
                Some(Date::from(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()))
            );
            assert_eq!(
                settings.max,
                Some(Date::from(NaiveDate::from_ymd_opt(2025, 12, 31).unwrap()))
            );
            assert_eq!(settings.initial_value_type, Some(InitialDate::Today));
            assert_eq!(settings.initial_shift, None);
            assert_eq!(
                settings.initial_date,
                Some(Date::from(NaiveDate::from_ymd_opt(2025, 6, 15).unwrap()))
            );
        } else {
            panic!("Expected date settings");
        }
    }

    #[tokio::test]
    async fn test_update_custom_field_date_settings_shifted_initial_date() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let expected_response = serde_json::json!({
            "id": 12345,
            "projectId": 123,
            "typeId": 4,
            "name": "Deadline Field",
            "description": "",
            "required": false,
            "useIssueType": false,
            "applicableIssueTypes": [],
            "displayOrder": 3,
            "min": null,
            "max": null,
            "initialDate": {
                "id": 2,
                "shift": 7,
                "date": null
            }
        });

        Mock::given(method("PATCH"))
            .and(path("/api/v2/projects/123/customFields/12345"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_response))
            .mount(&mock_server)
            .await;

        let params = UpdateCustomFieldParams::new(ProjectId::new(123), CustomFieldId::new(12345))
            .with_date_settings(None, None, Some(2), None, Some(7));

        let custom_field = project_api
            .update_custom_field(params)
            .await
            .expect("update_custom_field should succeed");
        let CustomFieldSettings::Date(settings) = custom_field.settings else {
            panic!("Expected date settings");
        };
        assert_eq!(settings.initial_value_type, Some(InitialDate::Shifted));
        assert_eq!(settings.initial_shift, Some(7));
        assert_eq!(settings.initial_date, None);
    }

    #[tokio::test]
    async fn test_update_custom_field_list_settings() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let expected_response = serde_json::json!({
            "id": 12345,
            "projectId": 123,
            "typeId": 5,
            "name": "Priority Field",
            "description": "",
            "required": false,
            "useIssueType": false,
            "applicableIssueTypes": [],
            "displayOrder": 4,
            "items": [
                { "id": 1, "name": "Low", "displayOrder": 0 },
                { "id": 2, "name": "Medium", "displayOrder": 1 },
                { "id": 3, "name": "High", "displayOrder": 2 }
            ],
            "allowAddItem": true,
            "allowInput": false
        });

        Mock::given(method("PATCH"))
            .and(path("/api/v2/projects/123/customFields/12345"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_response))
            .mount(&mock_server)
            .await;

        let params = UpdateCustomFieldParams::new(ProjectId::new(123), CustomFieldId::new(12345))
            .with_list_settings(
                Some(vec![
                    "Low".to_string(),
                    "Medium".to_string(),
                    "High".to_string(),
                ]),
                Some(false),
                Some(true),
            );

        let result = project_api.update_custom_field(params).await;
        assert!(result.is_ok());
        let custom_field = result.unwrap();
        if let CustomFieldSettings::SingleList(settings) = custom_field.settings {
            assert_eq!(settings.items.len(), 3);
            assert_eq!(settings.allow_add_item, Some(true));
            assert_eq!(settings.allow_input, Some(false));
        } else {
            panic!("Expected single list settings");
        }
    }

    #[tokio::test]
    async fn test_update_custom_field_partial_update() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let expected_response = serde_json::json!({
            "id": 12345,
            "projectId": 123,
            "typeId": 1,
            "name": "Updated Name Only",
            "description": "Original description",
            "required": false,
            "useIssueType": false,
            "applicableIssueTypes": [],
            "displayOrder": 1
        });

        Mock::given(method("PATCH"))
            .and(path("/api/v2/projects/123/customFields/12345"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_response))
            .mount(&mock_server)
            .await;

        let params = UpdateCustomFieldParams::new(ProjectId::new(123), CustomFieldId::new(12345))
            .with_name("Updated Name Only");

        let result = project_api.update_custom_field(params).await;
        assert!(result.is_ok());
        let custom_field = result.unwrap();
        assert_eq!(custom_field.name, "Updated Name Only");
        assert_eq!(custom_field.description, "Original description");
    }

    #[tokio::test]
    async fn test_update_custom_field_not_found() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "Custom field not found",
                    "code": 2
                }
            ]
        });

        Mock::given(method("PATCH"))
            .and(path("/api/v2/projects/123/customFields/99999"))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_response))
            .mount(&mock_server)
            .await;

        let params = UpdateCustomFieldParams::new(ProjectId::new(123), CustomFieldId::new(99999))
            .with_name("This will fail");

        let result = project_api.update_custom_field(params).await;
        assert!(result.is_err());
        match result {
            Err(ApiError::HttpStatus { status, .. }) => {
                assert_eq!(status, 404);
            }
            _ => panic!("Expected HttpStatus error"),
        }
    }

    #[tokio::test]
    async fn test_update_custom_field_permission_denied() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let error_response = serde_json::json!({
            "errors": [
                {
                    "message": "No project admin role",
                    "code": 5
                }
            ]
        });

        Mock::given(method("PATCH"))
            .and(path("/api/v2/projects/123/customFields/12345"))
            .respond_with(ResponseTemplate::new(403).set_body_json(error_response))
            .mount(&mock_server)
            .await;

        let params = UpdateCustomFieldParams::new(ProjectId::new(123), CustomFieldId::new(12345))
            .with_name("Permission test");

        let result = project_api.update_custom_field(params).await;
        assert!(result.is_err());
        match result {
            Err(ApiError::HttpStatus { status, .. }) => {
                assert_eq!(status, 403);
            }
            _ => panic!("Expected HttpStatus error"),
        }
    }

    #[tokio::test]
    async fn test_update_custom_field_with_project_key() {
        let mock_server = MockServer::start().await;
        let client = setup_client(&mock_server).await;
        let project_api = ProjectApi::new(client);

        let expected_response = serde_json::json!({
            "id": 12345,
            "projectId": 123,
            "typeId": 1,
            "name": "Updated via Key",
            "description": "",
            "required": false,
            "useIssueType": false,
            "applicableIssueTypes": [],
            "displayOrder": 1
        });

        Mock::given(method("PATCH"))
            .and(path("/api/v2/projects/PRJKEY/customFields/12345"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_response))
            .mount(&mock_server)
            .await;

        let params = UpdateCustomFieldParams::new(
            ProjectKey::from_str("PRJKEY").unwrap(),
            CustomFieldId::new(12345),
        )
        .with_name("Updated via Key");

        let result = project_api.update_custom_field(params).await;
        assert!(result.is_ok());
        let custom_field = result.unwrap();
        assert_eq!(custom_field.name, "Updated via Key");
    }
}
