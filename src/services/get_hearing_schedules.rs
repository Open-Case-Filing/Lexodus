use leptos::*;

use crate::models::hearing_schedule::HearingSchedule;

#[server(GetHearingSchedules, "/api")]
pub async fn get_hearing_schedules() -> Result<Vec<HearingSchedule>, ServerFnError> {
    Ok(vec![
        HearingSchedule {
            case_name: "Thompson v. National Bank".into(),
            case_number: "1:18-cv-56789".into(),
            date: "2024-06-20".into(),
            time: "10:00 AM".into(),
            courtroom: "Courtroom 2A".into(),
            status: "Scheduled".into(),
        },
        // Add more hearing schedules
    ])
}
