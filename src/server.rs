use crate::session::SqliteStore;
use leptos::provide_context;
use leptos_spin::{render_best_match_to_stream_with_context, server_fn::register_explicit, RouteTable};
use spin_sdk::http::{IncomingRequest, ResponseOutparam};
use spin_sdk::{http_component, sqlite::{Connection, Value}, variables};
use std::sync::Arc;

use serde::Serialize;


#[derive(Serialize)]
struct ToDo {
    id: u32,
    description: String,
    due: String,
}

#[http_component]
async fn handle_lexodus(req: IncomingRequest, resp_out: ResponseOutparam) {
    let mut conf = leptos::get_configuration(None).await.unwrap();
    conf.leptos_options.output_name = "lexodus".to_owned();
    // let expected = variables::get("token").unwrap();
    // let expected2 = variables::get("dev_value").unwrap();
    // let response = format!(
    //     "Hello, world! Token: {}, DEV_VALUE: {}",
    //     expected, expected2
    // );
    // println!("Vault Response: {}", response);

    // register_explicit::<crate::pages::cases::UpdateCount>();
    let app_router = crate::app::App;

    let mut routes = RouteTable::build(app_router);
    routes.add_server_fn_prefix("/api").unwrap();

    let con = Arc::new(Connection::open("default").expect("Failed to open lexodus db"));

    // Setup up Store for user sessions
    let store = SqliteStore::from_connection(con.clone());
    store.migrate().await.expect("Failed to migrate sessions!");


    let execute_params = [
        Value::Text("Try out Spin SQLite".to_owned()),
        Value::Text("Friday".to_owned()),
    ];
    con.execute(
        "INSERT INTO todos (description, due) VALUES (?, ?)",
        execute_params.as_slice(),
    );



 

    // Register server functions
    register_explicit::<crate::functions::save_count::SaveCount>();
    register_explicit::<crate::services::case_service::SearchCases>();
    register_explicit::<crate::pages::cases::AddCase>();

    render_best_match_to_stream_with_context(
        req,
        resp_out,
        &routes,
        app_router,
        move || {
            provide_context(con.clone());
            provide_context(store.clone());
        },
        &conf.leptos_options,
    )
    .await
    
}
