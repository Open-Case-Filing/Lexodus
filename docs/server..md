1. Case creation and management
2. Document handling
3. Docket management
4. Hearing scheduling
5. Motion and pleading filing
6. Discovery requests
7. Judgment and appeal handling
8. Financial transactions
9. Case transfers and assignments
10. Case tagging and flagging
11. Related case linking
12. Case searching and filtering
13. Reporting
14. Notification preferences
15. User activity logging
16. Case timeline generation
17. Case archiving and retrieval
18. Access control and permissions
19. Electronic signatures
----------------------------------------
1. User Activities:
   - Logging user activities
   - Retrieving user activity history

2. Chatrooms:
   - Creating chat rooms
   - Sending and retrieving chat messages
   - Joining and leaving chat rooms

3. Behavior Tracking:
   - Tracking user actions
   - Getting frequent user actions
   - Updating and retrieving last user actions
   - Managing user presence (online status)
----------------------------------------

use leptos::*;
use serde::{Deserialize, Serialize};
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use spin_sdk::pg::{Connection, ParameterValue};
        use spin_sdk::variables;
        use spin_sdk::pg::DbValue;
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Case {
    // ... (keep existing fields)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Party {
    id: i64,
    name: String,
    role: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Document {
    id: i64,
    title: String,
    file_path: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DocketEntry {
    id: i64,
    entry_date: String,
    description: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Hearing {
    id: i64,
    hearing_date: String,
    location: String,
    description: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Motion {
    id: i64,
    motion_type: String,
    filed_date: String,
    status: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pleading {
    id: i64,
    pleading_type: String,
    filed_date: String,
    status: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DiscoveryRequest {
    id: i64,
    request_type: String,
    request_date: String,
    status: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Judgment {
    id: i64,
    judgment_date: String,
    description: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Appeal {
    id: i64,
    appeal_date: String,
    appellate_court: String,
    status: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FinancialTransaction {
    id: i64,
    amount: f64,
    transaction_date: String,
    description: String,
}

// Case Creation and Basic Management

#[server(CreateCase, "/api")]
pub async fn create_case(case: Case) -> Result<String, ServerFnError> {
    // ... (keep existing implementation)
}

#[server(GetCaseDetails, "/api")]
pub async fn get_case_details(case_id: i64) -> Result<Case, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "SELECT * FROM cases WHERE id = $1";
    let row = conn.query_row(sql, &[ParameterValue::Int64(case_id)])?;

    // Convert row to Case struct
    Ok(Case { /* populate fields from row */ })
}

#[server(UpdateCaseDetails, "/api")]
pub async fn update_case_details(case: Case) -> Result<String, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "UPDATE cases SET title = $2, status = $3, ... WHERE id = $1";
    conn.execute(sql, &[/* parameters */])?;

    Ok("Case updated successfully".to_string())
}

#[server(DeleteCase, "/api")]
pub async fn delete_case(case_id: i64) -> Result<String, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "DELETE FROM cases WHERE id = $1";
    conn.execute(sql, &[ParameterValue::Int64(case_id)])?;

    Ok("Case deleted successfully".to_string())
}

// Case Status and History

#[server(UpdateCaseStatus, "/api")]
pub async fn update_case_status(case_id: i64, new_status: String) -> Result<String, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "UPDATE cases SET status = $2 WHERE id = $1";
    conn.execute(sql, &[ParameterValue::Int64(case_id), ParameterValue::Str(new_status)])?;

    Ok("Case status updated successfully".to_string())
}

#[server(GetCaseStatusHistory, "/api")]
pub async fn get_case_status_history(case_id: i64) -> Result<Vec<(String, String)>, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "SELECT status, effective_date FROM case_status_history WHERE case_id = $1 ORDER BY effective_date DESC";
    let rows = conn.query(sql, &[ParameterValue::Int64(case_id)])?;

    let history: Vec<(String, String)> = rows.iter().map(|row| {
        (
            row[0].as_string().unwrap_or_default().to_string(),
            row[1].as_string().unwrap_or_default().to_string(),
        )
    }).collect();

    Ok(history)
}

#[server(AddCaseNote, "/api")]
pub async fn add_case_note(case_id: i64, note: String) -> Result<String, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "INSERT INTO case_notes (case_id, note, created_at) VALUES ($1, $2, CURRENT_TIMESTAMP)";
    conn.execute(sql, &[ParameterValue::Int64(case_id), ParameterValue::Str(note)])?;

    Ok("Case note added successfully".to_string())
}

// Party Management

#[server(AddPartyToCase, "/api")]
pub async fn add_party_to_case(case_id: i64, party: Party) -> Result<String, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "INSERT INTO case_parties (case_id, party_id, role) VALUES ($1, $2, $3)";
    conn.execute(sql, &[ParameterValue::Int64(case_id), ParameterValue::Int64(party.id), ParameterValue::Str(party.role)])?;

    Ok("Party added to case successfully".to_string())
}

#[server(RemovePartyFromCase, "/api")]
pub async fn remove_party_from_case(case_id: i64, party_id: i64) -> Result<String, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "DELETE FROM case_parties WHERE case_id = $1 AND party_id = $2";
    conn.execute(sql, &[ParameterValue::Int64(case_id), ParameterValue::Int64(party_id)])?;

    Ok("Party removed from case successfully".to_string())
}

#[server(GetCaseParties, "/api")]
pub async fn get_case_parties(case_id: i64) -> Result<Vec<Party>, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "SELECT p.id, p.name, cp.role FROM parties p JOIN case_parties cp ON p.id = cp.party_id WHERE cp.case_id = $1";
    let rows = conn.query(sql, &[ParameterValue::Int64(case_id)])?;

    let parties: Vec<Party> = rows.iter().map(|row| {
        Party {
            id: row[0].as_int64().unwrap_or_default(),
            name: row[1].as_string().unwrap_or_default().to_string(),
            role: row[2].as_string().unwrap_or_default().to_string(),
        }
    }).collect();

    Ok(parties)
}

// Document Management

#[server(UploadDocument, "/api")]
pub async fn upload_document(case_id: i64, document: Document) -> Result<String, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "INSERT INTO documents (case_id, title, file_path) VALUES ($1, $2, $3)";
    conn.execute(sql, &[ParameterValue::Int64(case_id), ParameterValue::Str(document.title), ParameterValue::Str(document.file_path)])?;

    Ok("Document uploaded successfully".to_string())
}

#[server(GetCaseDocuments, "/api")]
pub async fn get_case_documents(case_id: i64) -> Result<Vec<Document>, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "SELECT id, title, file_path FROM documents WHERE case_id = $1";
    let rows = conn.query(sql, &[ParameterValue::Int64(case_id)])?;

    let documents: Vec<Document> = rows.iter().map(|row| {
        Document {
            id: row[0].as_int64().unwrap_or_default(),
            title: row[1].as_string().unwrap_or_default().to_string(),
            file_path: row[2].as_string().unwrap_or_default().to_string(),
        }
    }).collect();

    Ok(documents)
}

#[server(SealDocument, "/api")]
pub async fn seal_document(document_id: i64) -> Result<String, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "UPDATE documents SET is_sealed = TRUE WHERE id = $1";
    conn.execute(sql, &[ParameterValue::Int64(document_id)])?;

    Ok("Document sealed successfully".to_string())
}

#[server(UnsealDocument, "/api")]
pub async fn unseal_document(document_id: i64) -> Result<String, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "UPDATE documents SET is_sealed = FALSE WHERE id = $1";
    conn.execute(sql, &[ParameterValue::Int64(document_id)])?;

    Ok("Document unsealed successfully".to_string())
}

// Docket Management

#[server(AddDocketEntry, "/api")]
pub async fn add_docket_entry(case_id: i64, entry: DocketEntry) -> Result<String, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "INSERT INTO dockets (case_id, entry_date, description) VALUES ($1, $2, $3)";
    conn.execute(sql, &[ParameterValue::Int64(case_id), ParameterValue::Str(entry.entry_date), ParameterValue::Str(entry.description)])?;

    Ok("Docket entry added successfully".to_string())
}

#[server(GetCaseDocket, "/api")]
pub async fn get_case_docket(case_id: i64) -> Result<Vec<DocketEntry>, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "SELECT id, entry_date, description FROM dockets WHERE case_id = $1 ORDER BY entry_date DESC";
    let rows = conn.query(sql, &[ParameterValue::Int64(case_id)])?;

    let docket_entries: Vec<DocketEntry> = rows.iter().map(|row| {
        DocketEntry {
            id: row[0].as_int64().unwrap_or_default(),
            entry_date: row[1].as_string().unwrap_or_default().to_string(),
            description: row[2].as_string().unwrap_or_default().to_string(),
        }
    }).collect();

    Ok(docket_entries)
}

// Hearing and Schedule Management

#[server(ScheduleHearing, "/api")]
pub async fn schedule_hearing(case_id: i64, hearing: Hearing) -> Result<String, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "INSERT INTO hearings (case_id, hearing_date, location, description) VALUES ($1, $2, $3, $4)";
    conn.execute(sql, &[ParameterValue::Int64(case_id), ParameterValue::Str(hearing.hearing_date), ParameterValue::Str(hearing.location), ParameterValue::Str(hearing.description)])?;

    Ok("Hearing scheduled successfully".to_string())
}

#[server(GetCaseHearings, "/api")]
pub async fn get_case_hearings(case_id: i64) -> Result<Vec<Hearing>, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "SELECT id, hearing_date, location, description FROM hearings WHERE case_id = $1 ORDER BY hearing_date";
    let rows = conn.query(sql, &[ParameterValue::Int64(case_id)])?;

    let hearings: Vec<Hearing> = rows.iter().map(|row| {
        Hearing {
            id: row[0].as_int64().unwrap_or_default(),
            hearing_date: row[1].as_string().unwrap_or_default().to_string(),
            location: row[2].as_string().unwrap_or_default().to_string(),
            description: row[3].as_string().unwrap_or_default().to_string(),
        }
    }).collect();

    Ok(hearings)
}

#[server(CancelHearing, "/api")]
pub async fn cancel_hearing(hearing_id: i64) -> Result<String, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "DELETE FROM hearings WHERE id = $1";
    conn.execute(sql, &[ParameterValue::Int64(hearing_id)])?;

    Ok("Hearing cancelled successfully".to_string())
}

// Motion and Pleading Management

#[server(FileMotion, "/api")]
pub async fn file_motion(case_id: i64, motion: Motion) -> Result<String, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "INSERT INTO motions (case_id, motion_type, filed_date, status) VALUES ($1, $2, $3, $4)";
    conn.execute(sql, &[ParameterValue::Int64(case_id), ParameterValue::Str(motion.motion_type), ParameterValue::Str(motion.filed_date), ParameterValue::Str(motion.status)])?;

    Ok("Motion filed successfully".to_string())
}

#[server(GetCaseMotions, "/api")]
pub async fn get_case_motions(case_id: i64) -> Result<Vec<Motion>, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "SELECT id, motion_type, filed_date, status FROM motions WHERE case_id = $1 ORDER BY filed_date DESC";
    let rows = conn.query(sql, &[ParameterValue::Int64(case_id)])?;

    let motions: Vec<Motion> = rows.iter().map(|row| {
        Motion {
            id: row[0].as_int64().unwrap_or_default(),
            motion_type: row[1].as_string().unwrap_or_default().to_string(),
            filed_date: row[2].as_string().unwrap_or_default().to_string(),
            status: row[3].as_string().unwrap_or_default().to_string(),
        }
    }).collect();

    Ok(motions)
}

#[server(FilePleading, "/api")]
pub async fn file_pleading(case_id: i64, pleading: Pleading) -> Result<String, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "INSERT INTO pleadings (case_id, pleading_type, filed_date, status) VALUES ($1, $2, $3, $4)";
    conn.execute(sql, &[ParameterValue::Int64(case_id), ParameterValue::Str(pleading.pleading_type), ParameterValue::Str(pleading.filed_date), ParameterValue::Str(pleading.status)])?;

    Ok("Pleading filed successfully".to_string())
}

#[server(GetCasePleadings, "/api")]
pub async fn get_case_pleadings(case_id: i64) -> Result<Vec<Pleading>, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "SELECT id, pleading_type, filed_date, status FROM pleadings WHERE case_id = $1 ORDER BY filed_date DESC";
    let rows = conn.query(sql, &[ParameterValue::Int64(case_id)])?;

    let pleadings: Vec<Pleading> = rows.iter().map(|row| {
        Pleading {
            id: row[0].as_int64().unwrap_or_default(),
            pleading_type: row[1].as_string().unwrap_or_default().to_string(),
            filed_date: row[2].as_string().unwrap_or_default().to_string(),
            status: row[3].as_string().unwrap_or_default().to_string(),
        }
    }).collect();

    Ok(pleadings)
}

// Discovery Management

#[server(CreateDiscoveryRequest, "/api")]
pub async fn create_discovery_request(case_id: i64, request: DiscoveryRequest) -> Result<String, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "INSERT INTO discovery (case_id, request_type, request_date, status) VALUES ($1, $2, $3, $4)";
    conn.execute(sql, &[ParameterValue::Int64(case_id), ParameterValue::Str(request.request_type), ParameterValue::Str(request.request_date), ParameterValue::Str(request.status)])?;

    Ok("Discovery request created successfully".to_string())
}

#[server(GetCaseDiscovery, "/api")]
pub async fn get_case_discovery(case_id: i64) -> Result<Vec<DiscoveryRequest>, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "SELECT id, request_type, request_date, status FROM discovery WHERE case_id = $1 ORDER BY request_date DESC";
    let rows = conn.query(sql, &[ParameterValue::Int64(case_id)])?;

    let discovery_requests: Vec<DiscoveryRequest> = rows.iter().map(|row| {
        DiscoveryRequest {
            id: row[0].as_int64().unwrap_or_default(),
            request_type: row[1].as_string().unwrap_or_default().to_string(),
            request_date: row[2].as_string().unwrap_or_default().to_string(),
            status: row[3].as_string().unwrap_or_default().to_string(),
        }
    }).collect();

    Ok(discovery_requests)
}

// Judgment and Appeal Management

#[server(RecordJudgment, "/api")]
pub async fn record_judgment(case_id: i64, judgment: Judgment) -> Result<String, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "INSERT INTO judgments (case_id, judgment_date, description) VALUES ($1, $2, $3)";
    conn.execute(sql, &[ParameterValue::Int64(case_id), ParameterValue::Str(judgment.judgment_date), ParameterValue::Str(judgment.description)])?;

    Ok("Judgment recorded successfully".to_string())
}

#[server(GetCaseJudgments, "/api")]
pub async fn get_case_judgments(case_id: i64) -> Result<Vec<Judgment>, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "SELECT id, judgment_date, description FROM judgments WHERE case_id = $1 ORDER BY judgment_date DESC";
    let rows = conn.query(sql, &[ParameterValue::Int64(case_id)])?;

    let judgments: Vec<Judgment> = rows.iter().map(|row| {
        Judgment {
            id: row[0].as_int64().unwrap_or_default(),
            judgment_date: row[1].as_string().unwrap_or_default().to_string(),
            description: row[2].as_string().unwrap_or_default().to_string(),
        }
    }).collect();

    Ok(judgments)
}

#[server(FileAppeal, "/api")]
pub async fn file_appeal(case_id: i64, appeal: Appeal) -> Result<String, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "INSERT INTO appeals (case_id, appeal_date, appellate_court, status) VALUES ($1, $2, $3, $4)";
    conn.execute(sql, &[ParameterValue::Int64(case_id), ParameterValue::Str(appeal.appeal_date), ParameterValue::Str(appeal.appellate_court), ParameterValue::Str(appeal.status)])?;

    Ok("Appeal filed successfully".to_string())
}

#[server(GetCaseAppeals, "/api")]
pub async fn get_case_appeals(case_id: i64) -> Result<Vec<Appeal>, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "SELECT id, appeal_date, appellate_court, status FROM appeals WHERE case_id = $1 ORDER BY appeal_date DESC";
    let rows = conn.query(sql, &[ParameterValue::Int64(case_id)])?;

    let appeals: Vec<Appeal> = rows.iter().map(|row| {
        Appeal {
            id: row[0].as_int64().unwrap_or_default(),
            appeal_date: row[1].as_string().unwrap_or_default().to_string(),
            appellate_court: row[2].as_string().unwrap_or_default().to_string(),
            status: row[3].as_string().unwrap_or_default().to_string(),
        }
    }).collect();

    Ok(appeals)
}

// Financial Management

#[server(RecordPayment, "/api")]
pub async fn record_payment(case_id: i64, transaction: FinancialTransaction) -> Result<String, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "INSERT INTO fees_payments (case_id, amount, payment_date, description) VALUES ($1, $2, $3, $4)";
    conn.execute(sql, &[ParameterValue::Int64(case_id), ParameterValue::Float64(transaction.amount), ParameterValue::Str(transaction.transaction_date), ParameterValue::Str(transaction.description)])?;

    Ok("Payment recorded successfully".to_string())
}

#[server(GetCaseFinancials, "/api")]
pub async fn get_case_financials(case_id: i64) -> Result<Vec<FinancialTransaction>, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "SELECT id, amount, payment_date, description FROM fees_payments WHERE case_id = $1 ORDER BY payment_date DESC";
    let rows = conn.query(sql, &[ParameterValue::Int64(case_id)])?;

    let transactions: Vec<FinancialTransaction> = rows.iter().map(|row| {
        FinancialTransaction {
            id: row[0].as_int64().unwrap_or_default(),
            amount: row[1].as_float64().unwrap_or_default(),
            transaction_date: row[2].as_string().unwrap_or_default().to_string(),
            description: row[3].as_string().unwrap_or_default().to_string(),
        }
    }).collect();

    Ok(transactions)
}

// Case Transfer

#[server(TransferCase, "/api")]
pub async fn transfer_case(case_id: i64, to_court_id: i64, reason: String) -> Result<String, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "SELECT transfer_case($1, $2, $3)";
    conn.execute(sql, &[ParameterValue::Int64(case_id), ParameterValue::Int64(to_court_id), ParameterValue::Str(reason)])?;

    Ok("Case transferred successfully".to_string())
}

#[server(GetCaseTransferHistory, "/api")]
pub async fn get_case_transfer_history(case_id: i64) -> Result<Vec<(String, String, String)>, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "SELECT transfer_date, from_court.name, to_court.name FROM case_transfers ct JOIN courts from_court ON ct.from_court_id = from_court.id JOIN courts to_court ON ct.to_court_id = to_court.id WHERE case_id = $1 ORDER BY transfer_date DESC";
    let rows = conn.query(sql, &[ParameterValue::Int64(case_id)])?;

    let transfers: Vec<(String, String, String)> = rows.iter().map(|row| {
        (
            row[0].as_string().unwrap_or_default().to_string(),
            row[1].as_string().unwrap_or_default().to_string(),
            row[2].as_string().unwrap_or_default().to_string(),
        )
    }).collect();

    Ok(transfers)
}

// Case Assignment

#[server(AssignCase, "/api")]
pub async fn assign_case(case_id: i64, judge_id: i64) -> Result<String, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "UPDATE cases SET judge_id = $2 WHERE id = $1";
    conn.execute(sql, &[ParameterValue::Int64(case_id), ParameterValue::Int64(judge_id)])?;

    Ok("Case assigned successfully".to_string())
}

// Case Tagging and Categorization

#[server(AddCaseTag, "/api")]
pub async fn add_case_tag(case_id: i64, tag: String) -> Result<String, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "INSERT INTO case_tags (case_id, tag) VALUES ($1, $2)";
    conn.execute(sql, &[ParameterValue::Int64(case_id), ParameterValue::Str(tag)])?;

    Ok("Tag added successfully".to_string())
}

#[server(RemoveCaseTag, "/api")]
pub async fn remove_case_tag(case_id: i64, tag: String) -> Result<String, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "DELETE FROM case_tags WHERE case_id = $1 AND tag = $2";
    conn.execute(sql, &[ParameterValue::Int64(case_id), ParameterValue::Str(tag)])?;

    Ok("Tag removed successfully".to_string())
}

#[server(GetCaseTags, "/api")]
pub async fn get_case_tags(case_id: i64) -> Result<Vec<String>, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "SELECT tag FROM case_tags WHERE case_id = $1";
    let rows = conn.query(sql, &[ParameterValue::Int64(case_id)])?;

    let tags: Vec<String> = rows.iter().map(|row| {
        row[0].as_string().unwrap_or_default().to_string()
    }).collect();

    Ok(tags)
}

// Related Cases

#[server(LinkRelatedCase, "/api")]
pub async fn link_related_case(case_id: i64, related_case_id: i64, relationship_type: String) -> Result<String, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "INSERT INTO related_cases (case_id, related_case_id, relationship_type) VALUES ($1, $2, $3)";
    conn.execute(sql, &[ParameterValue::Int64(case_id), ParameterValue::Int64(related_case_id), ParameterValue::Str(relationship_type)])?;

    Ok("Cases linked successfully".to_string())
}


[server(GetRelatedCases, "/api")]
pub async fn get_related_cases(case_id: i64) -> Result<Vec<(i64, String, String)>, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "SELECT related_case_id, c.title, rc.relationship_type FROM related_cases rc JOIN cases c ON rc.related_case_id = c.id WHERE rc.case_id = $1";
    let rows = conn.query(sql, &[ParameterValue::Int64(case_id)])?;

    let related_cases: Vec<(i64, String, String)> = rows.iter().map(|row| {
        (
            row[0].as_int64().unwrap_or_default(),
            row[1].as_string().unwrap_or_default().to_string(),
            row[2].as_string().unwrap_or_default().to_string(),
        )
    }).collect();

    Ok(related_cases)
}

// Case Flags

#[server(AddCaseFlag, "/api")]
pub async fn add_case_flag(case_id: i64, flag_type: String) -> Result<String, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "INSERT INTO case_flags (case_id, flag_type, flag_value) VALUES ($1, $2, TRUE)";
    conn.execute(sql, &[ParameterValue::Int64(case_id), ParameterValue::Str(flag_type)])?;

    Ok("Flag added successfully".to_string())
}

#[server(RemoveCaseFlag, "/api")]
pub async fn remove_case_flag(case_id: i64, flag_type: String) -> Result<String, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "DELETE FROM case_flags WHERE case_id = $1 AND flag_type = $2";
    conn.execute(sql, &[ParameterValue::Int64(case_id), ParameterValue::Str(flag_type)])?;

    Ok("Flag removed successfully".to_string())
}

#[server(GetCaseFlags, "/api")]
pub async fn get_case_flags(case_id: i64) -> Result<Vec<String>, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "SELECT flag_type FROM case_flags WHERE case_id = $1 AND flag_value = TRUE";
    let rows = conn.query(sql, &[ParameterValue::Int64(case_id)])?;

    let flags: Vec<String> = rows.iter().map(|row| {
        row[0].as_string().unwrap_or_default().to_string()
    }).collect();

    Ok(flags)
}

// Case Search and Filtering

#[server(SearchCases, "/api")]
pub async fn search_cases(query: String, filters: Vec<(String, String)>) -> Result<Vec<Case>, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let mut sql = "SELECT * FROM cases WHERE (title ILIKE $1 OR case_number ILIKE $1)".to_string();
    let mut params = vec![ParameterValue::Str(format!("%{}%", query))];

    for (i, (field, value)) in filters.iter().enumerate() {
        sql += &format!(" AND {} = ${}", field, i + 2);
        params.push(ParameterValue::Str(value.clone()));
    }

    let rows = conn.query(&sql, &params)?;

    let cases: Vec<Case> = rows.iter().map(|row| {
        Case {
            // Populate Case struct fields from row
            // This will depend on your exact Case struct definition
        }
    }).collect();

    Ok(cases)
}

// Reporting

#[server(GenerateCaseStatusReport, "/api")]
pub async fn generate_case_status_report(start_date: String, end_date: String) -> Result<Vec<(String, i64)>, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "SELECT status, COUNT(*) as count FROM cases WHERE filed_date BETWEEN $1 AND $2 GROUP BY status";
    let rows = conn.query(sql, &[ParameterValue::Str(start_date), ParameterValue::Str(end_date)])?;

    let report: Vec<(String, i64)> = rows.iter().map(|row| {
        (
            row[0].as_string().unwrap_or_default().to_string(),
            row[1].as_int64().unwrap_or_default(),
        )
    }).collect();

    Ok(report)
}

// Notifications

#[server(SetNotificationPreference, "/api")]
pub async fn set_notification_preference(user_id: i64, notification_type: String, enabled: bool) -> Result<String, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "INSERT INTO user_notification_preferences (user_id, notification_type, enabled) VALUES ($1, $2, $3) ON CONFLICT (user_id, notification_type) DO UPDATE SET enabled = $3";
    conn.execute(sql, &[ParameterValue::Int64(user_id), ParameterValue::Str(notification_type), ParameterValue::Bool(enabled)])?;

    Ok("Notification preference set successfully".to_string())
}

// User Activity Logging

#[server(GetCaseActivityLogs, "/api")]
pub async fn get_case_activity_logs(case_id: i64) -> Result<Vec<(String, String, String)>, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "SELECT u.username, ual.action_type, ual.created_at FROM user_activity_logs ual JOIN users u ON ual.user_id = u.id WHERE ual.action_details->>'case_id' = $1 ORDER BY ual.created_at DESC";
    let rows = conn.query(sql, &[ParameterValue::Str(case_id.to_string())])?;

    let logs: Vec<(String, String, String)> = rows.iter().map(|row| {
        (
            row[0].as_string().unwrap_or_default().to_string(),
            row[1].as_string().unwrap_or_default().to_string(),
            row[2].as_string().unwrap_or_default().to_string(),
        )
    }).collect();

    Ok(logs)
}

// Case Timeline

#[server(GetCaseTimeline, "/api")]
pub async fn get_case_timeline(case_id: i64) -> Result<Vec<(String, String, String)>, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "
        SELECT 'Filing' as event_type, filed_date as event_date, 'Case filed' as description FROM cases WHERE id = $1
        UNION ALL
        SELECT 'Docket Entry' as event_type, entry_date as event_date, description FROM dockets WHERE case_id = $1
        UNION ALL
        SELECT 'Hearing' as event_type, hearing_date as event_date, description FROM hearings WHERE case_id = $1
        UNION ALL
        SELECT 'Motion' as event_type, filed_date as event_date, motion_type FROM motions WHERE case_id = $1
        UNION ALL
        SELECT 'Judgment' as event_type, judgment_date as event_date, description FROM judgments WHERE case_id = $1
        ORDER BY event_date
    ";
    let rows = conn.query(sql, &[ParameterValue::Int64(case_id)])?;

    let timeline: Vec<(String, String, String)> = rows.iter().map(|row| {
        (
            row[0].as_string().unwrap_or_default().to_string(),
            row[1].as_string().unwrap_or_default().to_string(),
            row[2].as_string().unwrap_or_default().to_string(),
        )
    }).collect();

    Ok(timeline)
}

// Case Archiving

#[server(ArchiveCase, "/api")]
pub async fn archive_case(case_id: i64) -> Result<String, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "UPDATE cases SET status = 'ARCHIVED', archived_at = CURRENT_TIMESTAMP WHERE id = $1";
    conn.execute(sql, &[ParameterValue::Int64(case_id)])?;

    Ok("Case archived successfully".to_string())
}

#[server(RetrieveArchivedCase, "/api")]
pub async fn retrieve_archived_case(case_id: i64) -> Result<String, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "UPDATE cases SET status = 'ACTIVE', archived_at = NULL WHERE id = $1";
    conn.execute(sql, &[ParameterValue::Int64(case_id)])?;

    Ok("Case retrieved from archive successfully".to_string())
}

// Access Control

#[server(GetUserPermissions, "/api")]
pub async fn get_user_permissions(user_id: i64) -> Result<Vec<String>, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "SELECT p.name FROM permissions p JOIN role_permissions rp ON p.id = rp.permission_id JOIN users u ON u.role_id = rp.role_id WHERE u.id = $1";
    let rows = conn.query(sql, &[ParameterValue::Int64(user_id)])?;

    let permissions: Vec<String> = rows.iter().map(|row| {
        row[0].as_string().unwrap_or_default().to_string()
    }).collect();

    Ok(permissions)
}

// Electronic Signatures

#[server(SignDocument, "/api")]
pub async fn sign_document(user_id: i64, document_id: i64, signature: String) -> Result<String, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "INSERT INTO electronic_signatures (user_id, document_id, signature_date, signature_hash) VALUES ($1, $2, CURRENT_TIMESTAMP, $3)";
    conn.execute(sql, &[ParameterValue::Int64(user_id), ParameterValue::Int64(document_id), ParameterValue::Str(signature)])?;

    Ok("Document signed successfully".to_string())
}

#[server(VerifySignature, "/api")]
pub async fn verify_signature(document_id: i64, signature: String) -> Result<bool, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "SELECT COUNT(*) FROM electronic_signatures WHERE document_id = $1 AND signature_hash = $2";
    let row = conn.query_row(sql, &[ParameterValue::Int64(document_id), ParameterValue::Str(signature)])?;

    let count = row[0].as_int64().unwrap_or_default();

    Ok(count > 0)
}
// User Activities

#[server(LogUserActivity, "/api")]
pub async fn log_user_activity(user_id: i64, action_type: String, details: String) -> Result<String, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "INSERT INTO user_activity_logs (user_id, action_type, action_details) VALUES ($1, $2, $3)";
    conn.execute(sql, &[ParameterValue::Int64(user_id), ParameterValue::Str(action_type), ParameterValue::Str(details)])?;

    Ok("User activity logged successfully".to_string())
}

#[server(GetUserActivities, "/api")]
pub async fn get_user_activities(user_id: i64, limit: i64) -> Result<Vec<(String, String, String)>, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "SELECT action_type, action_details, created_at FROM user_activity_logs WHERE user_id = $1 ORDER BY created_at DESC LIMIT $2";
    let rows = conn.query(sql, &[ParameterValue::Int64(user_id), ParameterValue::Int64(limit)])?;

    let activities: Vec<(String, String, String)> = rows.iter().map(|row| {
        (
            row[0].as_string().unwrap_or_default().to_string(),
            row[1].as_string().unwrap_or_default().to_string(),
            row[2].as_string().unwrap_or_default().to_string(),
        )
    }).collect();

    Ok(activities)
}

// Chatrooms

#[server(CreateChatRoom, "/api")]
pub async fn create_chat_room(case_id: i64, name: String) -> Result<i64, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "INSERT INTO chat_rooms (case_id, name) VALUES ($1, $2) RETURNING id";
    let row = conn.query_row(sql, &[ParameterValue::Int64(case_id), ParameterValue::Str(name)])?;

    Ok(row[0].as_int64().unwrap_or_default())
}

#[server(SendChatMessage, "/api")]
pub async fn send_chat_message(chat_room_id: i64, user_id: i64, content: String) -> Result<String, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "INSERT INTO messages (chat_room_id, user_id, content) VALUES ($1, $2, $3)";
    conn.execute(sql, &[ParameterValue::Int64(chat_room_id), ParameterValue::Int64(user_id), ParameterValue::Str(content)])?;

    Ok("Message sent successfully".to_string())
}

#[server(GetChatMessages, "/api")]
pub async fn get_chat_messages(chat_room_id: i64, limit: i64) -> Result<Vec<(i64, String, String, String)>, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "SELECT m.id, u.username, m.content, m.sent_at FROM messages m JOIN users u ON m.user_id = u.id WHERE m.chat_room_id = $1 ORDER BY m.sent_at DESC LIMIT $2";
    let rows = conn.query(sql, &[ParameterValue::Int64(chat_room_id), ParameterValue::Int64(limit)])?;

    let messages: Vec<(i64, String, String, String)> = rows.iter().map(|row| {
        (
            row[0].as_int64().unwrap_or_default(),
            row[1].as_string().unwrap_or_default().to_string(),
            row[2].as_string().unwrap_or_default().to_string(),
            row[3].as_string().unwrap_or_default().to_string(),
        )
    }).collect();

    Ok(messages)
}

#[server(JoinChatRoom, "/api")]
pub async fn join_chat_room(chat_room_id: i64, user_id: i64) -> Result<String, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "INSERT INTO chat_participants (chat_room_id, user_id) VALUES ($1, $2) ON CONFLICT DO NOTHING";
    conn.execute(sql, &[ParameterValue::Int64(chat_room_id), ParameterValue::Int64(user_id)])?;

    Ok("Joined chat room successfully".to_string())
}

#[server(LeaveChatRoom, "/api")]
pub async fn leave_chat_room(chat_room_id: i64, user_id: i64) -> Result<String, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "UPDATE chat_participants SET left_at = CURRENT_TIMESTAMP WHERE chat_room_id = $1 AND user_id = $2 AND left_at IS NULL";
    conn.execute(sql, &[ParameterValue::Int64(chat_room_id), ParameterValue::Int64(user_id)])?;

    Ok("Left chat room successfully".to_string())
}

// Behavior Tracking

#[server(TrackUserAction, "/api")]
pub async fn track_user_action(user_id: i64, action_type: String) -> Result<String, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "INSERT INTO user_actions (user_id, action_type, action_date) VALUES ($1, $2, CURRENT_TIMESTAMP)";
    conn.execute(sql, &[ParameterValue::Int64(user_id), ParameterValue::Str(action_type)])?;

    Ok("User action tracked successfully".to_string())
}

#[server(GetUserFrequentActions, "/api")]
pub async fn get_user_frequent_actions(user_id: i64) -> Result<Vec<(String, i64)>, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "SELECT action_type, COUNT(*) as frequency FROM user_actions WHERE user_id = $1 GROUP BY action_type ORDER BY frequency DESC LIMIT 5";
    let rows = conn.query(sql, &[ParameterValue::Int64(user_id)])?;

    let actions: Vec<(String, i64)> = rows.iter().map(|row| {
        (
            row[0].as_string().unwrap_or_default().to_string(),
            row[1].as_int64().unwrap_or_default(),
        )
    }).collect();

    Ok(actions)
}

#[server(UpdateUserLastAction, "/api")]
pub async fn update_user_last_action(user_id: i64, action_type: String) -> Result<String, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "INSERT INTO user_last_actions (user_id, action_type, last_used) VALUES ($1, $2, CURRENT_TIMESTAMP) ON CONFLICT (user_id, action_type) DO UPDATE SET last_used = CURRENT_TIMESTAMP";
    conn.execute(sql, &[ParameterValue::Int64(user_id), ParameterValue::Str(action_type)])?;

    Ok("User last action updated successfully".to_string())
}

#[server(GetUserLastActions, "/api")]
pub async fn get_user_last_actions(user_id: i64) -> Result<Vec<(String, String)>, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "SELECT action_type, last_used FROM user_last_actions WHERE user_id = $1 ORDER BY last_used DESC LIMIT 10";
    let rows = conn.query(sql, &[ParameterValue::Int64(user_id)])?;

    let actions: Vec<(String, String)> = rows.iter().map(|row| {
        (
            row[0].as_string().unwrap_or_default().to_string(),
            row[1].as_string().unwrap_or_default().to_string(),
        )
    }).collect();

    Ok(actions)
}

#[server(GetUserPresence, "/api")]
pub async fn get_user_presence(user_id: i64) -> Result<(String, String), ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "SELECT status, last_active FROM user_presence WHERE user_id = $1";
    let row = conn.query_row(sql, &[ParameterValue::Int64(user_id)])?;

    Ok((
        row[0].as_string().unwrap_or_default().to_string(),
        row[1].as_string().unwrap_or_default().to_string(),
    ))
}

#[server(UpdateUserPresence, "/api")]
pub async fn update_user_presence(user_id: i64, status: String) -> Result<String, ServerFnError> {
    let db_url = variables::get("db_url").unwrap();
    let conn = Connection::open(&db_url)?;

    let sql = "INSERT INTO user_presence (user_id, status, last_active) VALUES ($1, $2, CURRENT_TIMESTAMP) ON CONFLICT (user_id) DO UPDATE SET status = $2, last_active = CURRENT_TIMESTAMP";
    conn.execute(sql, &[ParameterValue::Int64(user_id), ParameterValue::Str(status)])?;

    Ok("User presence updated successfully".to_string())
}
