use chrono::{DateTime, Utc};
use notifbot_macros::notifbot_enum;

use super::Series;

#[derive(serde::Serialize, serde::Deserialize)]
struct Weekend {
    id: u64,
    name: String,
    year: i32,
    start_date: DateTime<Utc>,
    icon: String,
    series: Series,
    status: WeekendStatus,
}

notifbot_enum!(WeekendStatus { Open, Done });

