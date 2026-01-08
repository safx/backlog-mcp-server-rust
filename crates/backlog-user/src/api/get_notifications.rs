use backlog_api_core::IntoRequest;
use backlog_core::identifier::UserId;
use serde::Serialize;

use crate::models::Notification;

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum NotificationOrder {
    Asc,
    #[default]
    Desc,
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetNotificationsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<NotificationOrder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_id: Option<UserId>,
}

impl GetNotificationsParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_min_id(mut self, min_id: u64) -> Self {
        self.min_id = Some(min_id);
        self
    }

    pub fn with_max_id(mut self, max_id: u64) -> Self {
        self.max_id = Some(max_id);
        self
    }

    pub fn with_count(mut self, count: u8) -> Self {
        self.count = Some(count.clamp(1, 100));
        self
    }

    pub fn with_order(mut self, order: NotificationOrder) -> Self {
        self.order = Some(order);
        self
    }

    pub fn with_sender_id(mut self, sender_id: UserId) -> Self {
        self.sender_id = Some(sender_id);
        self
    }
}

impl IntoRequest for GetNotificationsParams {
    fn path(&self) -> String {
        "/api/v2/notifications".to_string()
    }

    fn to_query(&self) -> impl Serialize {
        self
    }
}

pub type GetNotificationsResponse = Vec<Notification>;
