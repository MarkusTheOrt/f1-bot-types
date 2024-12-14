use chrono::{DateTime, Utc};
use notifbot_macros::notifbot_enum;


notifbot_enum!(SessionNotifySettings { Notify, Ignore });

notifbot_enum!(SessionKind {
    Racing,
    Other
});

notifbot_enum!(SessionStatus {
    Open,
    Finished,
    Delayed,
    Cancelled
});

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Session {
    pub id: i64,
    pub weekend: i64,
    pub start_date: DateTime<Utc>,
    pub title: String,
    pub kind: SessionKind,
    pub duration: i32,
    pub notify: SessionNotifySettings,
    pub status: SessionStatus,
}

