// In src/models/case.rs
// use cfg_if::cfg_if;
// cfg_if! {

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Case {
    pub id: String,
    pub case_name: String,
    pub case_number: String,
    pub court: String,
    pub date_filed: DateTime<Utc>,
    pub date_last_filing: Option<DateTime<Utc>>,
    pub nature_of_suit: Option<String>,
    pub jurisdiction_type: String,
    pub assigned_to: Option<String>,
    pub cause: Option<String>,
    pub docket_number: String,
    pub status: String,
}
