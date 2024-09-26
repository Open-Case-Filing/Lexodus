// In src/models/case.rs
// use cfg_if::cfg_if;
// cfg_if! {

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Case {
    pub id: i64,
    pub case_number: String,
    pub title: String,
    pub status: String,
    pub filed_date: DateTime<Utc>,
    pub closed_date: Option<DateTime<Utc>>,
    pub court_id: i64,
    pub current_court_id: i64,
    pub judge_id: Option<i64>,
}
