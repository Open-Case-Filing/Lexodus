use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use spin_sdk::variables;
        use crate::domain::models::mdl::MDLProceeding;
        use crate::infrastructure::repositories::pg_mdl_repository::PgMDLRepository;
        use crate::domain::services::generate_mdl_number;
        use anyhow::Result;

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
    }
}
