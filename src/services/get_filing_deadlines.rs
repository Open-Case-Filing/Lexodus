use leptos::*;

use crate::models::filing_deadline::FilingDeadline;

#[server(GetFilingDeadlines, "/api")]
pub async fn get_filing_deadlines() -> Result<Vec<FilingDeadline>, ServerFnError> {
    Ok(vec![
        FilingDeadline {
            case_name: "Wilson v. State University".into(),
            case_number: "3:17-cv-24680".into(),
            deadline: "2024-06-15".into(),
            description: "Submit additional evidence".into(),
            status: "Upcoming".into(),
        },
        // Add more filing deadlines
    ])
}