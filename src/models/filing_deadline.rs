use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct FilingDeadline {
    pub case_name: String,
    pub case_number: String,
    pub deadline: String,
    pub description: String,
    pub status: String,
}
