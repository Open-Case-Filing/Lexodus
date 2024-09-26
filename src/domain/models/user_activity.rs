use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct UserActivity {
    pub user_id: i64,
    pub activity: String,
    pub frequency: i64
}
