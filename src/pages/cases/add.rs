use crate::layouts::wide::Wide_Layout;
use leptos::*;
use serde::{Deserialize, Serialize};

// Keep all the enum and struct definitions as they are...

#[derive(Clone, Debug, Serialize, Deserialize)]
struct CaseFlowOutput {
    table_rows: Vec<(String, String)>,
    debug_output: String,
}

#[component]
pub fn Add() -> impl IntoView {
    view! {
          <Wide_Layout>
          <h1>"Add Case"</h1>
          </Wide_Layout>
    }
}
