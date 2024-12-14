use chrono::{DateTime, Utc};
use notifbot_macros::notifbot_enum;

notifbot_enum!(Series {
    F1,
    F2,
    F3,
    F1Academy
});

notifbot_enum!(SessionNotifySettings { Notify, Ignore });

notifbot_enum!(SessionKind {
    Race,
    SprintRace,
    Qualifying,
    SprintQualifying,
    Practice,
    Custom
});

notifbot_enum!(SessionStatus {
    Open,
    Finished,
    Delayed,
    Cancelled
});

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Session {
    id: i64,
    weekend: i64,
    start_date: DateTime<Utc>,
    title: String,
    kind: SessionKind,
    duration: i32,
    notify: SessionNotifySettings,
    status: SessionStatus,
}

