use super::{Language, Role};
use crate::identifier::UserId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: UserId,
    pub user_id: Option<String>,
    pub name: String,
    pub role_type: Role,
    pub lang: Option<Language>,
    pub mail_address: String,
    pub last_login_time: Option<DateTime<Utc>>,
}

#[cfg(test)]
mod tests {
    use crate::identifier::Identifier;
    use serde_json::Value;

    #[test]
    fn deserializable() {
        let json_str = r#"
        {
            "id": 1,
            "userId": "admin",
            "name": "John Doe",
            "roleType": 1,
            "lang": "ja",
            "mailAddress": "johndoe@example.com",
            "lastLoginTime": "2022-09-01T06:35:39Z"
        }"#;
        let user: super::User = serde_json::from_str(json_str).unwrap();
        assert_eq!(user.id.value(), 1);
        assert_eq!(user.user_id, Some("admin".to_string()));
        assert_eq!(user.name, "John Doe");
        assert_eq!(user.role_type, super::Role::Admin);
        assert_eq!(user.lang, Some(super::Language::Japanese));
        assert_eq!(user.mail_address, "johndoe@example.com".to_string());
        let expected_dt = Some("2022-09-01T06:35:39Z".parse().unwrap());
        assert_eq!(user.last_login_time, expected_dt);
        assert_eq!(user.role_type, super::Role::Admin);
    }

    #[test]
    fn serializable() {
        let user = super::User {
            id: super::UserId::new(1),
            user_id: Some("admin".to_string()),
            name: "John Doe".to_string(),
            role_type: super::Role::Admin,
            lang: Some(super::Language::Japanese),
            mail_address: "johndoe@example.com".to_string(),
            last_login_time: Some("2022-09-01T06:35:39Z".parse().unwrap()),
        };

        let json = serde_json::to_string(&user).unwrap();
        let actual: Value = serde_json::from_str(&json).unwrap();
        let expected: Value = serde_json::json!({
            "id": 1,
            "userId": "admin",
            "name": "John Doe",
            "roleType": 1,
            "lang": "ja",
            "mailAddress": "johndoe@example.com",
            "lastLoginTime": "2022-09-01T06:35:39Z"
        });
        assert_eq!(actual, expected);
    }

    #[test]
    fn deserializable_null_fields() {
        let json_str = r#"
        {
            "id": 2,
            "userId": null,
            "name": "Jane Doe",
            "roleType": 2,
            "lang": null,
            "mailAddress": "null@example.com",
            "lastLoginTime": null
        }"#;
        let user: super::User = serde_json::from_str(json_str).unwrap();
        assert_eq!(user.id.value(), 2);
        assert_eq!(user.user_id, None);
        assert_eq!(user.name, "Jane Doe");
        assert_eq!(user.role_type, super::Role::User); // Assuming Role::User is 2
        assert_eq!(user.lang, None);
        assert_eq!(user.mail_address, "null@example.com");
        assert_eq!(user.last_login_time, None);
    }
}
