
use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Clone)]
pub struct CaseNotification {
    pub  notification_id: String,
    pub case_name: String,
    pub date: String,
    pub message: String,
    pub  status: String,
}
