
use leptos::provide_context;
use leptos_spin::{
    render_best_match_to_stream_with_context, server_fn::register_explicit, RouteTable,
};
use spin_sdk::http::{IncomingRequest, ResponseOutparam};
use spin_sdk::pg::{self};
use spin_sdk::{http_component, variables};
use std::sync::Arc;
use dotenv::dotenv;



#[http_component]
async fn handle_lexodus(req: IncomingRequest, resp_out: ResponseOutparam) {
    let mut conf = leptos::get_configuration(None).await.unwrap();
    conf.leptos_options.output_name = "lexodus".to_owned();
    // let token = variables::get("token").unwrap();
    // let dev_value = variables::get("dev_value").unwrap();
    // let db_url = variables::get("db_url").unwrap();
    // let response = format!(
    //     "Hello, world!
    //     DB_URL: {}",
    //     // token,
    //     // dev_value,
    //     db_url
    // );
    // println!("Configuration: {}", response);
    dotenv().ok();

    // register_explicit::<crate::pages::cases::UpdateCount>();
    let app_router = crate::app::App;

    let mut routes = RouteTable::build(app_router);
    routes.add_server_fn_prefix("/api").unwrap();

    // let con =
    //     Arc::new(spin_sdk::sqlite::Connection::open("default").expect("Failed to open lexodus db"));
    // let conn = Arc::new(pg::Connection::open(&db_url).expect("Failed to open postgres db"));

    // let _create_pg_tables = "CREATE TABLE PERSONS (
    //     id SERIAL PRIMARY KEY,
    //     lastname varchar(255),
    //     firstname varchar(255),
    //     address varchar(255),
    //     city varchar(255)
    // );";

    // let _create_table_sqlite = conn.execute(_create_pg_tables, &[]);

    // let sql = "INSERT INTO Persons (lastname, firstname, address, city)
    // VALUES ('John', 'Marshall', 'Great Chief Justice rd', 'Washington D.C.');";
    // let _ = conn.execute(sql, &[]);

    // Create sqlite tables

//     let _sql_create_tables = "CREATE TABLE PERSONS (
//     id SERIAL PRIMARY KEY,
//     lastname varchar(255),
//     firstname varchar(255),
//     address varchar(255),
//     city varchar(255)
// );";

//     let _ = con.execute(_sql_create_tables, &[]);

//     let sql = "INSERT INTO Persons (lastname, firstname, address, city)
// VALUES ('John', 'Marshall', 'Great Chief Justice', 'Washington D.C.');";
//     let _ = con.execute(sql, &[]);

// Setup up Store for user sessions
  // let store = SqliteStore::from_connection(con.clone());
  // store.migrate().await.expect("Failed to migrate sessions!");

    // Register server functions
    // case-management :: Case Management - Criminal / Civil Case
    register_explicit::<crate::infrastructure::server::functions::case_functions::CreateCase>();
    // case-management :: Multi-District Litigation
    register_explicit::<crate::infrastructure::server::functions::mdl::CreateMDLProceeding>();
    register_explicit::<crate::infrastructure::server::functions::mdl::AddCaseToMDL>();
    register_explicit::<crate::infrastructure::server::functions::mdl::RemandCaseFromMDL>();
    register_explicit::<crate::infrastructure::server::functions::mdl::get_mdl_cases>();
    // Change these when we get CQRS pattern down (todo!)
    register_explicit::<crate::pages::cases::CreateCase>();
    register_explicit::<crate::pages::cases::GetCases>();
    register_explicit::<crate::pages::user_management::CreateUser>();
    register_explicit::<crate::pages::user_management::GetUsers>();

    render_best_match_to_stream_with_context(
        req,
        resp_out,
        &routes,
        app_router,
        move || {
            provide_context(con.clone());
            provide_context(conn.clone());

        },
        &conf.leptos_options,
    )
    .await
}
