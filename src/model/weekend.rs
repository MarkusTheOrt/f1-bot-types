use chrono::{DateTime, Utc};
use notifbot_macros::notifbot_enum;

use super::Series;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Weekend {
    pub id: u64,
    pub name: String,
    pub year: i32,
    pub start_date: DateTime<Utc>,
    pub icon: String,
    pub series: Series,
    pub status: WeekendStatus,
}

notifbot_enum!(WeekendStatus { Open, Done });

