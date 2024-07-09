use crate::models::case::Case;
use chrono::{TimeZone, Utc};
use leptos::*;



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
                case.case_name.to_lowercase().contains(term)
                    || case.court.to_lowercase().contains(term)
                    || case.case_number.to_lowercase().contains(term)
                    || case
                        .assigned_to
                        .as_ref()
                        .map(|s| s.to_lowercase().contains(term))
                        .unwrap_or(false)
                    || case.status.to_lowercase().contains(term)
                    || case.jurisdiction_type.to_lowercase().contains(term)
                    || case
                        .nature_of_suit
                        .as_ref()
                        .map(|s| s.to_lowercase().contains(term))
                        .unwrap_or(false)
                    || case
                        .cause
                        .as_ref()
                        .map(|s| s.to_lowercase().contains(term))
                        .unwrap_or(false)
                    || case.docket_number.to_lowercase().contains(term)
                    || is_date_match(case.date_filed)
                    || case.date_last_filing.map(is_date_match).unwrap_or(false)
            })
        })
        .collect())
}



// Function to return all cases
fn get_all_cases() -> Vec<Case> {
    vec![
        Case {
            id: "1".into(),
            case_name: "Smith v. Johnson".into(),
            case_number: "2:20-cv-01234".into(),
            court: "Eastern District of New York".into(),
            date_filed: Utc.with_ymd_and_hms(2020, 5, 15, 0, 0, 0).unwrap(),
            date_last_filing: Some(Utc.with_ymd_and_hms(2023, 11, 30, 0, 0, 0).unwrap()),
            nature_of_suit: Some("Contract".into()),
            jurisdiction_type: "Diversity".into(),
            assigned_to: Some("Judge Brown".into()),
            cause: Some("Breach of Contract".into()),
            docket_number: "1234".into(),
            status: "Open".into(),
        },
        Case {
            id: "2".into(),
            case_name: "United States v. Doe".into(),
            case_number: "3:21-cr-05678".into(),
            court: "Northern District of California".into(),
            date_filed: Utc.with_ymd_and_hms(2021, 8, 3, 0, 0, 0).unwrap(),
            date_last_filing: Some(Utc.with_ymd_and_hms(2024, 1, 15, 0, 0, 0).unwrap()),
            nature_of_suit: Some("Criminal".into()),
            jurisdiction_type: "Federal Question".into(),
            assigned_to: Some("Judge Smith".into()),
            cause: Some("Wire Fraud".into()),
            docket_number: "5678".into(),
            status: "Closed".into(),
        },
        Case {
            id: "3".into(),
            case_name: "In re XYZ Corp".into(),
            case_number: "1:22-bk-09876".into(),
            court: "Southern District of New York".into(),
            date_filed: Utc.with_ymd_and_hms(2022, 12, 1, 0, 0, 0).unwrap(),
            date_last_filing: None,
            nature_of_suit: Some("Bankruptcy".into()),
            jurisdiction_type: "Federal Question".into(),
            assigned_to: Some("Judge Davis".into()),
            cause: Some("Chapter 11".into()),
            docket_number: "9876".into(),
            status: "Open".into(),
        },
        Case {
            id: "4".into(),
            case_name: "Johnson v. City of Springfield".into(),
            case_number: "4:19-cv-07890".into(),
            court: "District of Massachusetts".into(),
            date_filed: Utc.with_ymd_and_hms(2019, 3, 22, 0, 0, 0).unwrap(),
            date_last_filing: Some(Utc.with_ymd_and_hms(2023, 9, 5, 0, 0, 0).unwrap()),
            nature_of_suit: Some("Civil Rights".into()),
            jurisdiction_type: "Federal Question".into(),
            assigned_to: Some("Judge Martinez".into()),
            cause: Some("Employment Discrimination".into()),
            docket_number: "4321".into(),
            status: "Closed".into(),
        },
        Case {
            id: "5".into(),
            case_name: "In re Green Energy Solutions".into(),
            case_number: "2:23-bk-12345".into(),
            court: "Central District of California".into(),
            date_filed: Utc.with_ymd_and_hms(2023, 6, 10, 0, 0, 0).unwrap(),
            date_last_filing: Some(Utc.with_ymd_and_hms(2024, 2, 28, 0, 0, 0).unwrap()),
            nature_of_suit: Some("Bankruptcy".into()),
            jurisdiction_type: "Federal Question".into(),
            assigned_to: Some("Judge Williams".into()),
            cause: Some("Chapter 7".into()),
            docket_number: "5432".into(),
            status: "Open".into(),
        },
        Case {
            id: "6".into(),
            case_name: "Thompson v. National Bank".into(),
            case_number: "1:18-cv-56789".into(),
            court: "Northern District of Illinois".into(),
            date_filed: Utc.with_ymd_and_hms(2018, 11, 5, 0, 0, 0).unwrap(),
            date_last_filing: Some(Utc.with_ymd_and_hms(2022, 7, 19, 0, 0, 0).unwrap()),
            nature_of_suit: Some("Banking".into()),
            jurisdiction_type: "Diversity".into(),
            assigned_to: Some("Judge Garcia".into()),
            cause: Some("Predatory Lending".into()),
            docket_number: "6543".into(),
            status: "Closed".into(),
        },
        Case {
            id: "7".into(),
            case_name: "United States v. Smith Corp".into(),
            case_number: "5:24-cr-10101".into(),
            court: "Eastern District of Pennsylvania".into(),
            date_filed: Utc.with_ymd_and_hms(2024, 1, 3, 0, 0, 0).unwrap(),
            date_last_filing: None,
            nature_of_suit: Some("Criminal".into()),
            jurisdiction_type: "Federal Question".into(),
            assigned_to: Some("Judge Thompson".into()),
            cause: Some("Tax Evasion".into()),
            docket_number: "7654".into(),
            status: "Open".into(),
        },
        Case {
            id: "8".into(),
            case_name: "Wilson v. State University".into(),
            case_number: "3:17-cv-24680".into(),
            court: "Middle District of Florida".into(),
            date_filed: Utc.with_ymd_and_hms(2017, 9, 15, 0, 0, 0).unwrap(),
            date_last_filing: Some(Utc.with_ymd_and_hms(2024, 3, 1, 0, 0, 0).unwrap()),
            nature_of_suit: Some("Education".into()),
            jurisdiction_type: "Federal Question".into(),
            assigned_to: Some("Judge Rodriguez".into()),
            cause: Some("Title IX Violation".into()),
            docket_number: "8765".into(),
            status: "Open".into(),
        },
        Case {
            id: "9".into(),
            case_name: "Brown v. Tech Innovations Inc.".into(),
            case_number: "2:21-cv-98765".into(),
            court: "Western District of Washington".into(),
            date_filed: Utc.with_ymd_and_hms(2021, 4, 12, 0, 0, 0).unwrap(),
            date_last_filing: Some(Utc.with_ymd_and_hms(2023, 12, 5, 0, 0, 0).unwrap()),
            nature_of_suit: Some("Intellectual Property".into()),
            jurisdiction_type: "Federal Question".into(),
            assigned_to: Some("Judge Anderson".into()),
            cause: Some("Patent Infringement".into()),
            docket_number: "9876".into(),
            status: "Open".into(),
        },
        Case {
            id: "10".into(),
            case_name: "Garcia v. Insurance Co.".into(),
            case_number: "4:22-cv-54321".into(),
            court: "Southern District of Texas".into(),
            date_filed: Utc.with_ymd_and_hms(2022, 2, 28, 0, 0, 0).unwrap(),
            date_last_filing: Some(Utc.with_ymd_and_hms(2024, 1, 20, 0, 0, 0).unwrap()),
            nature_of_suit: Some("Insurance".into()),
            jurisdiction_type: "Diversity".into(),
            assigned_to: Some("Judge Patel".into()),
            cause: Some("Bad Faith Insurance Practices".into()),
            docket_number: "1357".into(),
            status: "Open".into(),
        },
        Case {
            id: "11".into(),
            case_name: "United States v. Jones".into(),
            case_number: "1:23-cr-11111".into(),
            court: "District of Columbia".into(),
            date_filed: Utc.with_ymd_and_hms(2023, 7, 7, 0, 0, 0).unwrap(),
            date_last_filing: None,
            nature_of_suit: Some("Criminal".into()),
            jurisdiction_type: "Federal Question".into(),
            assigned_to: Some("Judge Washington".into()),
            cause: Some("Cybercrime".into()),
            docket_number: "2468".into(),
            status: "Open".into(),
        },
        Case {
            id: "12".into(),
            case_name: "Environmental Group v. Industrial Corp.".into(),
            case_number: "3:20-cv-13579".into(),
            court: "District of Oregon".into(),
            date_filed: Utc.with_ymd_and_hms(2020, 9, 30, 0, 0, 0).unwrap(),
            date_last_filing: Some(Utc.with_ymd_and_hms(2023, 11, 15, 0, 0, 0).unwrap()),
            nature_of_suit: Some("Environmental".into()),
            jurisdiction_type: "Federal Question".into(),
            assigned_to: Some("Judge Chen".into()),
            cause: Some("Clean Water Act Violation".into()),
            docket_number: "3690".into(),
            status: "Closed".into(),
        },
        Case {
            id: "13".into(),
            case_name: "Taylor v. Social Media Giant".into(),
            case_number: "5:19-cv-24680".into(),
            court: "Northern District of California".into(),
            date_filed: Utc.with_ymd_and_hms(2019, 12, 10, 0, 0, 0).unwrap(),
            date_last_filing: Some(Utc.with_ymd_and_hms(2024, 2, 14, 0, 0, 0).unwrap()),
            nature_of_suit: Some("Privacy".into()),
            jurisdiction_type: "Federal Question".into(),
            assigned_to: Some("Judge Kim".into()),
            cause: Some("Data Privacy Violation".into()),
            docket_number: "4812".into(),
            status: "Open".into(),
        },
        Case {
            id: "14".into(),
            case_name: "In re Midwest Manufacturing".into(),
            case_number: "2:21-bk-36912".into(),
            court: "Northern District of Ohio".into(),
            date_filed: Utc.with_ymd_and_hms(2021, 11, 20, 0, 0, 0).unwrap(),
            date_last_filing: Some(Utc.with_ymd_and_hms(2023, 8, 5, 0, 0, 0).unwrap()),
            nature_of_suit: Some("Bankruptcy".into()),
            jurisdiction_type: "Federal Question".into(),
            assigned_to: Some("Judge O'Connor".into()),
            cause: Some("Chapter 11 Reorganization".into()),
            docket_number: "5934".into(),
            status: "Closed".into(),
        },
        Case {
            id: "15".into(),
            case_name: "Consumer Class v. Auto Manufacturer".into(),
            case_number: "1:22-cv-78901".into(),
            court: "Eastern District of Michigan".into(),
            date_filed: Utc.with_ymd_and_hms(2022, 5, 5, 0, 0, 0).unwrap(),
            date_last_filing: Some(Utc.with_ymd_and_hms(2024, 3, 10, 0, 0, 0).unwrap()),
            nature_of_suit: Some("Consumer Protection".into()),
            jurisdiction_type: "Diversity".into(),
            assigned_to: Some("Judge Lee".into()),
            cause: Some("Product Liability".into()),
            docket_number: "7056".into(),
            status: "Open".into(),
        },
    ]
}
