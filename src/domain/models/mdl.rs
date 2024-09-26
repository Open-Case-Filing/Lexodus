use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MDLCase {
    pub mdl_id: i64,
    pub case_id: i64,
    pub date_added: String,
    pub date_remanded: Option<String>,
}



#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MDLProceeding {
    pub id: i64,
    pub mdl_number: String,
    pub title: String,
}
