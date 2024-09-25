
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MDLProceeding {
    pub id: i64,
    pub mdl_number: String,
    pub title: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MDLCase {
    pub mdl_id: i64,
    pub case_id: i64,
    pub date_added: String,
    pub date_remanded: Option<String>,
}


// src/application/commands/create_mdl_proceeding.rs

use crate::domain::models::MDLProceeding;
use crate::domain::repositories::MDLRepository;
use crate::domain::services::generate_mdl_number;
use crate::infrastructure::repositories::pg_mdl_repository::PgMDLRepository;
use anyhow::Result;
use spin_sdk::variables;

pub async fn create_mdl_proceeding(title: String) -> Result<i64> {
    let db_url = variables::get("db_url").expect("db_url must be set");
    let repo = PgMDLRepository::new(&db_url)?;

    let mdl_number = generate_mdl_number::generate()?;

    let mdl = MDLProceeding {
        id: 0, // This will be set by the database
        mdl_number,
        title,
    };

    repo.save(&mdl).await
}
