use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Case {
    pub id: i64,
    pub case_number: String,
    pub title: String,
    pub case_type: String,
    pub nature_of_suit: String,
    pub filing_type: String,
    pub status: String,
    pub filed_date: String,
    pub closed_date: Option<String>,
    pub reopened_date: Option<String>,
    pub court_id: i64,
    pub division_id: Option<i64>,
    pub assigned_judge_id: Option<i64>,
    pub security_level: String,
    pub created_by: i64,
    pub updated_by: Option<i64>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaseCreate {
    pub title: String,
    pub case_type: String,
    pub nature_of_suit: String,
    pub filing_type: String,
    pub court_id: i64,
    pub division_id: Option<i64>,
    pub assigned_judge_id: Option<i64>,
    pub security_level: String,
    pub created_by: i64
}
