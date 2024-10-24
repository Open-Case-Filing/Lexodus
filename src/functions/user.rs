use crate::domain::models::user::{SafeUser, User};
use cfg_if::cfg_if;
use leptos::{expect_context, server, use_context, ServerFnError};

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::functions::{con};
        use super::auth::auth_session;
    }
}

#[tracing::instrument(level = "info", fields(error), ret, err)]
#[server(GetUser, "/api")]
/// Get the current user if it exists by checking the user's session against the DB
pub async fn get_user() -> Result<Option<User>, ServerFnError> {
    let Some(req) = use_context::<leptos_spin::RequestParts>() else {
        return Err(ServerFnError::MissingArg("Missing Request".to_string()));
    };
    let con = con()?;
    let user = match auth_session(&req, &con).await {
        Ok(u) => Some(u),
        Err(_) => None,
    };
    Ok(user)
}

#[tracing::instrument(level = "info", fields(error), ret, err)]
#[server(GetSafeUser, "/api")]
/// Get the current user if it exists by checking the user's session against the DB
pub async fn get_safe_user() -> Result<Option<SafeUser>, ServerFnError> {
    let Some(req) = use_context::<leptos_spin::RequestParts>() else {
        return Err(ServerFnError::MissingArg("Missing Request".to_string()));
    };
    let con = con()?;

    // Redirect all non logged in users to Nedry!
    let safe_user = match auth_session(&req, &con).await {
        Ok(u) => Some(u.into()),
        Err(_) => None,
    };

    Ok(safe_user)
}
