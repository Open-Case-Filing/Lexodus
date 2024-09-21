use crate::models::case::Case;
use chrono::{TimeZone, Utc};
use leptos::*;

// Function to return all cases
#[allow(dead_code)]
fn get_all_cases() -> Vec<Case> {
    vec![
        Case {
          id: 1,
          case_number: "2:20-cv-01234".to_string(),
          title: "Smith v. Johnson".to_string(),
          status: "Open".to_string(),
          filed_date: Utc.with_ymd_and_hms(2020, 5, 15, 0, 0, 0).unwrap(),
          closed_date: None,
          court_id: 1, // Assuming 1 is the ID for Eastern District of New York
          current_court_id: 1,
          judge_id: Some(101), // Assuming 101 is Judge Brown's ID

        },
        Case {
          id: 2,
          case_number: "3:21-cr-05678".to_string(),
          title: "United States v. Doe".to_string(),
          status: "Closed".to_string(),
          filed_date: Utc.with_ymd_and_hms(2021, 8, 3, 0, 0, 0).unwrap(),
          closed_date: Some(Utc.with_ymd_and_hms(2024, 1, 15, 0, 0, 0).unwrap()),
          court_id: 2, // Assuming 2 is the ID for Northern District of California
          current_court_id: 2,
          judge_id: Some(102), // Assuming 102 is Judge Smith's ID

        },
        Case {
          id: 3,
          case_number: "1:22-bk-09876".to_string(),
          title: "In re XYZ Corp".to_string(),
          status: "Open".to_string(),
          filed_date: Utc.with_ymd_and_hms(2022, 12, 1, 0, 0, 0).unwrap(),
          closed_date: None,
          court_id: 3, // Assuming 3 is the ID for Southern District of New York
          current_court_id: 3,
          judge_id: Some(103), // Assuming 103 is Judge Davis's ID

        },
        Case {
          id: 15,
          case_number: "1:22-cv-78901".to_string(),
          title: "Consumer Class v. Auto Manufacturer".to_string(),
          status: "Open".to_string(),
          filed_date: Utc.with_ymd_and_hms(2022, 5, 5, 0, 0, 0).unwrap(),
          closed_date: Some(Utc.with_ymd_and_hms(2024, 5, 5, 0, 0, 0).unwrap()),
          court_id: 15, // Assuming 15 is the ID for Eastern District of Michigan
          current_court_id: 15,
          judge_id: Some(115), // Assuming 115 is Judge Lee's ID

        },

    ]
}

#[server(SearchCases, "/api")]
pub async fn search_cases(search: String) -> Result<Vec<Case>, ServerFnError> {
    let all_cases = get_all_cases();

    let search_terms: Vec<String> = search
        .split_whitespace()
        .map(|s| s.to_lowercase())
        .collect();

    Ok(all_cases
        .into_iter()
        .filter(|case| {
            search_terms.iter().all(|term| {
                // Define is_date_match closure
                let is_date_match = |date: chrono::DateTime<Utc>| {
                    let date_str = date.format("%Y-%m-%d").to_string().to_lowercase();
                    let year_str = date.format("%Y").to_string();
                    let month_str = date.format("%m").to_string();
                    let day_str = date.format("%d").to_string();

                    term == &year_str
                        || term == &month_str
                        || term == &day_str
                        || date_str.contains(term)
                        || chrono::NaiveDate::parse_from_str(term, "%Y-%m-%d")
                            .map(|parsed_date| parsed_date == date.naive_utc().date())
                            .unwrap_or(false)
                };

                // Use is_date_match in our filter conditions
                case.title.to_lowercase().contains(term)
                    || case.case_number.to_lowercase().contains(term)
                    || case.status.to_lowercase().contains(term)
                    || case.court_id.to_string().contains(term)
                    || case.current_court_id.to_string().contains(term)
                    || case.judge_id.map_or(false, |id| id.to_string().contains(term))
                    || is_date_match(case.filed_date)
                    || case.closed_date.map_or(false, |date| is_date_match(date))
            })
        })
        .collect())
}
