#![allow(unused_variables)]
#![allow(unused_imports)]
pub mod auth;
pub mod user;

use crate::errors::LexodusAppError;
use cfg_if::cfg_if;
use leptos::server_fn::server_fn_error;

cfg_if! {
    if #[cfg(feature = "ssr")] {

        use spin_sdk::sqlite::{Connection};

        use leptos::*;
        use std::sync::Arc;

        pub fn con() -> Result<Arc<Connection>, LexodusAppError> {
            use_context::<Arc<Connection>>()
                .ok_or("Connection missing.")
                .map_err(|_| LexodusAppError::DBConnectionNotFound)
        }
}}
