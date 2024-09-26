use leptos::*;
use crate::domain::models::mdl::MDLCase;
#[server(CreateMDLProceeding, "/api")]
pub async fn create_mdl_proceeding(title: String) -> Result<String, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    // Generate MDL number
    let mdl_number = generate_mdl_number(&conn)?;

    let sql = "INSERT INTO mdl_proceedings (mdl_number, title) VALUES ($1, $2) RETURNING id";
    let result = conn.query(sql, &[
        ParameterValue::Str(mdl_number.clone()),
        ParameterValue::Str(title),
    ])?;

    if let Some(row) = result.get(0) {
        let id: i64 = row.get(0).parse().map_err(|e| ServerFnError::ServerError(e.to_string()))?;
        Ok(format!("MDL proceeding created with number: {} and ID: {}", mdl_number, id))
    } else {
        Err(ServerFnError::ServerError("Failed to create MDL proceeding".to_string()))
    }
}

#[server(AddCaseToMDL, "/api")]
pub async fn add_case_to_mdl(mdl_id: i64, case_id: i64) -> Result<String, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    // Add case to MDL
    let sql = "INSERT INTO mdl_cases (mdl_id, case_id, date_added) VALUES ($1, $2, CURRENT_TIMESTAMP)";
    conn.execute(sql, &[
        ParameterValue::Int64(mdl_id),
        ParameterValue::Int64(case_id),
    ])?;

    // Update case status
    let update_sql = "UPDATE cases SET is_mdl = TRUE, mdl_status = 'CONSOLIDATED' WHERE id = $1";
    conn.execute(update_sql, &[ParameterValue::Int64(case_id)])?;

    Ok(format!("Case {} added to MDL {}", case_id, mdl_id))
}

#[server(RemandCaseFromMDL, "/api")]
pub async fn remand_case_from_mdl(mdl_id: i64, case_id: i64) -> Result<String, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    // Update MDL case
    let sql = "UPDATE mdl_cases SET date_remanded = CURRENT_TIMESTAMP WHERE mdl_id = $1 AND case_id = $2";
    conn.execute(sql, &[
        ParameterValue::Int64(mdl_id),
        ParameterValue::Int64(case_id),
    ])?;

    // Update case status
    let update_sql = "UPDATE cases SET mdl_status = 'REMANDED' WHERE id = $1";
    conn.execute(update_sql, &[ParameterValue::Int64(case_id)])?;

    Ok(format!("Case {} remanded from MDL {}", case_id, mdl_id))
}

#[server(GetMDLCases, "/api")]
pub async fn get_mdl_cases(mdl_id: i64) -> Result<Vec<MDLCase>, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "SELECT mdl_id, case_id, date_added, date_remanded FROM mdl_cases WHERE mdl_id = $1";
    let result = conn.query(sql, &[ParameterValue::Int64(mdl_id)])?;

    let mdl_cases: Vec<MDLCase> = result
        .iter()
        .map(|row| MDLCase {
            mdl_id: row.get(0).parse().unwrap(),
            case_id: row.get(1).parse().unwrap(),
            date_added: row.get(2),
            date_remanded: row.get(3),
        })
        .collect();

    Ok(mdl_cases)
}
