use crate::layouts::wide::Wide_Layout;
use colored::*;
use leptos::*;
use leptos::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Keep all the enum and struct definitions as they are...

#[derive(Clone, Debug, Serialize, Deserialize)]
struct CaseFlowOutput {
    table_rows: Vec<(String, String)>,
    debug_output: String,
}

#[server(GenerateCaseFlow)]
pub async fn generate_case_flow() -> Result<CaseFlowOutput, ServerFnError> {
    let user = User {
        id: String::from("123"),
        name: String::from("John Doe"),
        role: String::from("Attorney"),
    };

    let mut case_flow = CaseFlow::new(user);

    // Simulate the case flow (keep this part as it is)...

    // Generate table rows
    let mut table_rows = Vec::new();
    let add_row = |field: &str, value: ColoredString| {
        table_rows.push((field.to_string(), value.to_string()));
    };

    add_row(
        "Login Status",
        format!("{:?}", case_flow.login_status).green(),
    );
    add_row("Current Step", case_flow.current_step.yellow());
    add_row(
        "Case ID",
        case_flow
            .case
            .as_ref()
            .map_or("N/A".into(), |c| c.id.clone())
            .cyan(),
    );
    add_row(
        "Court Type",
        case_flow
            .case
            .as_ref()
            .map_or("N/A".into(), |c| c.court_type.clone())
            .cyan(),
    );
    add_row(
        "Case Type",
        case_flow
            .case
            .as_ref()
            .map_or("N/A".into(), |c| c.case_type.clone())
            .cyan(),
    );
    add_row(
        "Action",
        case_flow
            .case
            .as_ref()
            .map_or("N/A".into(), |c| c.action.clone())
            .cyan(),
    );
    add_row(
        "Lead Event",
        case_flow
            .case
            .as_ref()
            .map_or("N/A".into(), |c| c.lead_event.clone())
            .cyan(),
    );
    add_row(
        "Document Type",
        case_flow
            .case
            .as_ref()
            .map_or("N/A".into(), |c| c.document_type.clone())
            .cyan(),
    );
    add_row(
        "Parties Count",
        case_flow.parties.len().to_string().magenta(),
    );
    add_row(
        "Documents Count",
        case_flow.documents.len().to_string().magenta(),
    );
    add_row(
        "Payment Status",
        format!(
            "{:?}",
            case_flow
                .payment
                .as_ref()
                .map_or(PaymentStatus::NotRequired, |p| p.status)
        )
        .green(),
    );
    add_row(
        "Payment Amount",
        format!(
            "${:.2}",
            case_flow.payment.as_ref().map_or(0.0, |p| p.amount)
        )
        .green(),
    );
    add_row(
        "Error Status",
        format!("{:?}", case_flow.data_error_status).red(),
    );
    add_row(
        "Document Status",
        format!("{:?}", case_flow.document_status).yellow(),
    );

    // Generate debug output
    let debug_output = format!("{:#?}", case_flow);

    Ok(CaseFlowOutput {
        table_rows,
        debug_output,
    })
}
#[component]
pub fn Add() -> impl IntoView {
    view! {
          <Wide_Layout>
          <h1>"Add Case"</h1>
          </Wide_Layout>
    }
}
