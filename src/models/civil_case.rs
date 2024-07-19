use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate, Default)]
pub struct CivilCase {
    #[validate(length(min = 1, max = 100))]
    pub case_type: String,
    #[validate(nested)]
    pub plaintiff: Party,
    #[validate(nested)]
    pub defendant: Party,
    #[validate(length(min = 10, max = 1000))]
    pub cause_of_action: String,
    #[validate(length(min = 10, max = 1000))]
    pub relief_sought: String,
    #[validate(length(min = 10, max = 10))] // Assuming YYYY-MM-DD format
    pub filing_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, Default)]
pub struct Party {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    #[validate(length(min = 5, max = 200))]
    pub address: String,
    #[validate(length(min = 10, max = 20))] // Basic length check for phone
    pub phone: String,
    #[validate(email)]
    pub email: String,
}