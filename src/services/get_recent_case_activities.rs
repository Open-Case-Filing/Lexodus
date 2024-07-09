use leptos::*;

use crate::models::recent_case_activity::RecentCaseActivity;

#[server(GetRecentCaseActivities, "/api")]
pub async fn get_recent_case_activities() -> Result<Vec<RecentCaseActivity>, ServerFnError> {
    Ok(vec![
        RecentCaseActivity {
            case_name: "Smith v. Johnson".into(),
            case_number: "2:20-cv-01234".into(),
            activity: "Filing of Motion".into(),
            date: "2024-05-09".into(),
            involved_parties: "Leslie Alexander vs. State".into(),
            status: "Pending".into(),
        },
        // Add more recent case activities
    ])
}
