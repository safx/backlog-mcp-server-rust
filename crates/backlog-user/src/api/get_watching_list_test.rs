#[cfg(test)]
mod get_watching_list_tests {

    use backlog_api_core::IntoRequest;
    use backlog_core::identifier::{IssueId, UserId};

    use crate::api::get_watching_list::{
        GetWatchingListParams, GetWatchingListRequest, Order, WatchingSort,
    };

    #[test]
    fn test_default_params_form_serialization() {
        let params = GetWatchingListParams::default();
        let form_params: Vec<(String, String)> = (&params).into();
        assert_eq!(form_params.len(), 0);
    }

    #[test]
    fn test_full_params_serialization() {
        let params = GetWatchingListParams::builder()
            .order(Order::Asc)
            .sort(WatchingSort::Created)
            .count(50)
            .offset(100)
            .resource_already_read(true)
            .issue_ids(vec![IssueId::from(123), IssueId::from(456)])
            .build()
            .expect("builder should succeed with all fields");

        let form_params: Vec<(String, String)> = (&params).into();

        // Check all parameters are properly serialized
        assert!(form_params.contains(&("order".to_string(), "asc".to_string())));
        assert!(form_params.contains(&("sort".to_string(), "created".to_string())));
        assert!(form_params.contains(&("count".to_string(), "50".to_string())));
        assert!(form_params.contains(&("offset".to_string(), "100".to_string())));
        assert!(form_params.contains(&("resourceAlreadyRead".to_string(), "true".to_string())));
        assert!(form_params.contains(&("issueId[]".to_string(), "123".to_string())));
        assert!(form_params.contains(&("issueId[]".to_string(), "456".to_string())));
    }

    #[test]
    fn test_order_serialization() {
        assert_eq!(
            serde_json::to_string(&Order::Asc).expect("Order::Asc should serialize"),
            r#""asc""#
        );
        assert_eq!(
            serde_json::to_string(&Order::Desc).expect("Order::Desc should serialize"),
            r#""desc""#
        );
    }

    #[test]
    fn test_watching_sort_serialization() {
        assert_eq!(
            serde_json::to_string(&WatchingSort::Created)
                .expect("WatchingSort::Created should serialize"),
            r#""created""#
        );
        assert_eq!(
            serde_json::to_string(&WatchingSort::Updated)
                .expect("WatchingSort::Updated should serialize"),
            r#""updated""#
        );
        assert_eq!(
            serde_json::to_string(&WatchingSort::IssueUpdated)
                .expect("WatchingSort::IssueUpdated should serialize"),
            r#""issueUpdated""#
        );
    }

    #[test]
    fn test_request_path() {
        let request = GetWatchingListRequest {
            user_id: UserId::from(123),
            params: GetWatchingListParams::default(),
        };

        assert_eq!(request.path(), "/api/v2/users/123/watchings");
    }

    #[test]
    fn test_partial_params() {
        let params = GetWatchingListParams::builder()
            .count(10)
            .build()
            .expect("builder should succeed with partial fields");

        let form_params: Vec<(String, String)> = (&params).into();
        assert_eq!(form_params.len(), 1);
        assert!(form_params.contains(&("count".to_string(), "10".to_string())));
    }

    #[test]
    fn test_issue_ids_array_format() {
        let params = GetWatchingListParams::builder()
            .issue_ids(vec![IssueId::from(1), IssueId::from(2), IssueId::from(3)])
            .build()
            .expect("builder should succeed with issue_ids");

        let form_params: Vec<(String, String)> = (&params).into();

        let issue_id_params: Vec<_> = form_params
            .iter()
            .filter(|(key, _)| key == "issueId[]")
            .collect();

        assert_eq!(issue_id_params.len(), 3);
        assert!(form_params.contains(&("issueId[]".to_string(), "1".to_string())));
        assert!(form_params.contains(&("issueId[]".to_string(), "2".to_string())));
        assert!(form_params.contains(&("issueId[]".to_string(), "3".to_string())));
    }
}
