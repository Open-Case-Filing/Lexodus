use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NewCase {
    pub title: String,
    pub status: String,
    pub court_id: i64,
    pub case_type: String,  // e.g., "CV" for civil, "CR" for criminal
    pub judge_id: Option<i64>,
}
