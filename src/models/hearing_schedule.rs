use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Clone)]
pub struct HearingSchedule {
    pub case_name: String,
    pub  case_number: String,
    pub date: String,
    pub time: String,
    pub  courtroom: String,
    pub  status: String,
}
