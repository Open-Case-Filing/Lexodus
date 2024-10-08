mod app;
mod components;
mod functions;
mod layouts;
mod models;
mod pages;
mod services;
mod domain;
mod errors;
mod session;
mod providers;
mod presentation;
#[cfg(feature = "ssr")]
mod server;

use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "hydrate")] {

  use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    pub fn hydrate() {
      use app::*;

      console_error_panic_hook::set_once();

      leptos::mount_to_body(App);
    }
}
}
