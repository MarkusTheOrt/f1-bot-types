use chrono::{DateTime, Utc};
use notifbot_macros::notifbot_enum;

use super::Series;

notifbot_enum!(MessageKind {
    Weekend,
    Notification,
    Calendar,
    Custom
});

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Message {
    pub id: u64,
    pub channel: String,
    pub message: String,
    pub kind: MessageKind,
    pub posted: DateTime<Utc>,
    pub hash: Option<String>,
    pub series: Series,
}
