use leptos_spin::{render_best_match_to_stream, server_fn::register_explicit, RouteTable};
use spin_sdk::http::{IncomingRequest, ResponseOutparam};
use spin_sdk::{http_component, variables};
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

    // Register server functions
    register_explicit::<crate::functions::save_count::SaveCount>();
    register_explicit::<crate::services::case_service::SearchCases>();

    let app = crate::app::App;

    let mut routes = RouteTable::build(app);
    routes.add_server_fn_prefix("/api").unwrap();

    render_best_match_to_stream(req, resp_out, &routes, app, &conf.leptos_options).await
}
