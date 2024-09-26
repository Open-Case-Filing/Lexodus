use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::domain::models::mdl::MDLProceeding;
        use anyhow::Result;
        use spin_sdk::pg::{Connection, ParameterValue};
        use async_trait::async_trait;

        pub struct PgMDLRepository {
            conn: Connection,
        }

        impl PgMDLRepository {
            pub fn new(db_url: &str) -> Result<Self> {
                let conn = Connection::open(db_url)?;
                Ok(Self { conn })
            }
        }

        #[async_trait]
        impl MDLRepository for PgMDLRepository {
            async fn save(&self, mdl: &MDLProceeding) -> Result<i64> {
                let sql = "INSERT INTO mdl_proceedings (mdl_number, title) VALUES ($1, $2) RETURNING id";
                let result = self.conn.query(sql, &[
                    ParameterValue::Str(mdl.mdl_number.clone()),
                    ParameterValue::Str(mdl.title.clone()),
                ])?;
                if let Some(row) = result.get(0) {
                    Ok(row.get(0).parse()?)
                } else {
                    Err(anyhow::anyhow!("Failed to insert MDL proceeding"))
                }
            }

            async fn find_by_id(&self, id: i64) -> Result<Option<MDLProceeding>> {
                let sql = "SELECT * FROM mdl_proceedings WHERE id = $1";
                let result = self.conn.query(sql, &[ParameterValue::Int64(id)])?;
                if let Some(row) = result.get(0) {
                    Ok(Some(MDLProceeding {
                        id: row.get(0).parse()?,
                        mdl_number: row.get(1),
                        title: row.get(2),
                    }))
                } else {
                    Ok(None)
                }
            }
        }
    }
}
