use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct CourtOrder {
    pub order_number: String,
    pub case_name: String,
    pub date: String,
    pub order_details: String,
    pub status: String,
}
