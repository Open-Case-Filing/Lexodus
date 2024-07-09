use leptos::*;

use crate::models::court_order::CourtOrder;

#[server(GetCourtOrders, "/api")]
pub async fn get_court_orders() -> Result<Vec<CourtOrder>, ServerFnError> {
    Ok(vec![
        CourtOrder {
            order_number: "12345".into(),
            case_name: "United States v. Doe".into(),
            date: "2024-05-10".into(),
            order_details: "Order to dismiss".into(),
            status: "Active".into(),
        },
        // Add more court orders
    ])
}