use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct RecentCaseActivity {
    pub case_name: String,
    pub case_number: String,
    pub activity: String,
    pub date: String,
    pub involved_parties: String,
    pub status: String,
}
