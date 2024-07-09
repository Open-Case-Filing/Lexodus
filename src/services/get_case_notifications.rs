use leptos::*;

use crate::models::case_notification::CaseNotification;

#[server(GetCaseNotifications, "/api")]
pub async fn get_case_notifications() -> Result<Vec<CaseNotification>, ServerFnError> {
    Ok(vec![
        CaseNotification {
            notification_id: "notif-001".into(),
            case_name: "Garcia v. Insurance Co.".into(),
            date: "2024-05-18".into(),
            message: "New evidence submitted".into(),
            status: "Unread".into(),
        },
        // Add more case notifications
    ])
}