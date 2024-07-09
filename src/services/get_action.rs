use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Hearing {
    pub case_id: String,
    pub description: String,
    pub date: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Motion {
    pub case_id: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Filing {
    pub case_id: String,
    pub  description: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CourtroomAssignment {
    pub case_id: String,
    pub courtroom: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ActionMenuData {
    pub upcoming_hearings: Vec<Hearing>,
    pub  pending_motions: Vec<Motion>,
    pub  recent_filings: Vec<Filing>,
    pub  courtroom_assignments: Vec<CourtroomAssignment>,
}

#[server(GetActionMenuData, "/api")]
pub async fn get_action_menu_data() -> Result<ActionMenuData, ServerFnError> {
    Ok(ActionMenuData {
        upcoming_hearings: vec![
            Hearing { case_id: "2024-001".into(), description: "Hearing on June 15".into(), date: "2024-06-15".into() },
            Hearing { case_id: "2024-002".into(), description: "Hearing on June 20".into(), date: "2024-06-20".into() },
            // Add more hearings
        ],
        pending_motions: vec![
            Motion { case_id: "2024-005".into(), description: "Motion to Dismiss".into() },
            Motion { case_id: "2024-006".into(), description: "Motion for Summary Judgment".into() },
            // Add more motions
        ],
        recent_filings: vec![
            Filing { case_id: "2024-008".into(), description: "Plaintiff's Response".into() },
            Filing { case_id: "2024-009".into(), description: "Defendant's Answer".into() },
            // Add more filings
        ],
        courtroom_assignments: vec![
            CourtroomAssignment { case_id: "2024-011".into(), courtroom: "Courtroom 3B".into() },
            CourtroomAssignment { case_id: "2024-012".into(), courtroom: "Courtroom 2A".into() },
            // Add more assignments
        ],
    })
}
