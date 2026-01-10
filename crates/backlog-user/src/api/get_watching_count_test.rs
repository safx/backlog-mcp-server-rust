#[cfg(test)]
mod get_watching_count_tests {
    use backlog_api_core::IntoRequest;
    use backlog_core::identifier::{Identifier, UserId};

    use crate::api::get_watching_count::GetWatchingCountParams;

    #[test]
    fn test_default_params() {
        let params = GetWatchingCountParams::new(UserId::from(123));

        assert_eq!(params.user_id.value(), 123);
        assert_eq!(params.resource_already_read, None);
        assert_eq!(params.already_read, None);
    }

    #[test]
    fn test_params_with_filters() {
        let params = GetWatchingCountParams::new(UserId::from(456))
            .with_resource_already_read(true)
            .with_already_read(false);

        assert_eq!(params.user_id.value(), 456);
        assert_eq!(params.resource_already_read, Some(true));
        assert_eq!(params.already_read, Some(false));
    }

    #[test]
    fn test_path_generation() {
        let params = GetWatchingCountParams::new(UserId::from(789));
        assert_eq!(params.path(), "/api/v2/users/789/watchings/count");
    }

    #[test]
    fn test_query_serialization_empty() {
        let params = GetWatchingCountParams::new(UserId::from(123));
        let query =
            serde_json::to_value(params.to_query()).expect("params should serialize to JSON");

        assert!(
            query
                .as_object()
                .expect("query should be an object")
                .is_empty()
        );
    }

    #[test]
    fn test_query_serialization_with_filters() {
        let params = GetWatchingCountParams::new(UserId::from(123))
            .with_resource_already_read(true)
            .with_already_read(false);

        let query =
            serde_json::to_value(params.to_query()).expect("params should serialize to JSON");
        let query_obj = query.as_object().expect("query should be an object");

        assert_eq!(
            query_obj
                .get("resourceAlreadyRead")
                .expect("resourceAlreadyRead should exist"),
            &serde_json::Value::Bool(true)
        );
        assert_eq!(
            query_obj
                .get("alreadyRead")
                .expect("alreadyRead should exist"),
            &serde_json::Value::Bool(false)
        );
    }

    #[test]
    fn test_partial_filters() {
        let params =
            GetWatchingCountParams::new(UserId::from(123)).with_resource_already_read(false);

        let query =
            serde_json::to_value(params.to_query()).expect("params should serialize to JSON");
        let query_obj = query.as_object().expect("query should be an object");

        assert_eq!(query_obj.len(), 1);
        assert_eq!(
            query_obj
                .get("resourceAlreadyRead")
                .expect("resourceAlreadyRead should exist"),
            &serde_json::Value::Bool(false)
        );
        assert!(!query_obj.contains_key("alreadyRead"));
    }
}
