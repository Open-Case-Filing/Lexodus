use std::borrow::Cow;

use crate::domain::models::user::User;
use cfg_if::cfg_if;
use leptos::server;
use leptos::{expect_context, use_context, ServerFnError};

cfg_if! {
if #[cfg(feature = "ssr")] {
    use argon2::{
        password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
        Argon2,
    };
    use crate::functions::{con};
    use crate::errors::LexodusAppError;
    use rand_core::OsRng;
    use spin_sdk::sqlite::{Connection, Value::{Text, Integer}};
    use std::sync::Arc;
    use tracing::info;
    use async_session::{Session, SessionStore};
    use crate::session::{SqliteStore};
    use leptos_spin::ResponseOptions;
    use cookie::Cookie;
    use leptos_spin::RequestParts;
    /// Hash Argon2 password
    pub fn hash_password(password: &[u8]) -> Result<String, LexodusAppError> {
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = argon2.hash_password(password, &salt)?.to_string();
        Ok(password_hash)
    }
    /// Verify Password
    pub fn verify_password(password: &str, stored_password_hash: &str) -> Result<(), LexodusAppError> {
        let argon2 = Argon2::default();
        // Verify password against PHC string
        let parsed_hash = PasswordHash::new(stored_password_hash)?;
        Ok(argon2.verify_password(password.as_bytes(), &parsed_hash)?)
    }

    /// Verify the user is who they say they are
    pub async fn auth_user(name: &str, password: &str, con: &Arc<Connection>) -> Result<User, LexodusAppError>{
        // Does the user exist
        let Ok(Some(user)) = User::get_from_username(name, con).await else{
            return Err(LexodusAppError::AuthError);
        };

        // Check that password is correct
        match verify_password(password, &user.password){
            Ok(_) => Ok(user),
            Err(e) => {println!("Verify Failed: {e}"); Err(LexodusAppError::AuthError)},
        }
    }
    pub fn get_session_cookie_value(req_parts: &RequestParts)-> Result<Option<String>, LexodusAppError>{
    let cookies: Vec<(&String, Cow<'_, str>)> = req_parts
        .headers()
        .iter()
        .filter(|(k, _v)| k == "cookie")
        .map(|(k, v)| (k, String::from_utf8_lossy(v)))
        .collect();
    let cookie_string = cookies.first().map(|(k, v)| v);
    let cookie_jar = match cookie_string {
        Some(c) => Cookie::split_parse(c.clone()),
        None => return Err(LexodusAppError::AuthError),
    };

    let mut session_val = None;
    for cookie in cookie_jar.into_iter(){
        if let Ok(c) = cookie {
            if c.name() == "Lexodus_session" {
                session_val = Some(c.clone().value().to_owned());
                break;
            }
        }
    };
    Ok(session_val)
    }

    pub async fn auth_session(req_parts: &RequestParts, con: &Arc<Connection>)-> Result<User, LexodusAppError>{

    let store = expect_context::<SqliteStore>();
    let session_val = match get_session_cookie_value(req_parts)?{
    Some(sv) => sv,
    None => return Err(LexodusAppError::AuthError),
    };

    let Some(session) = store.load_session(session_val).await? else{
        return Err(LexodusAppError::InternalServerError);
    };
    let Some(user_id) = session.get("user_id") else{
        return Err(LexodusAppError::AuthError);
    };

    let user = match User::get(user_id, con).await?{
    Some(u) => u,
    None => return Err(LexodusAppError::AuthError)
    };
    Ok(user)
    }

    /// Create a new Session and store User id in it
    pub async fn create_session(user_id: i64)-> Result<String, LexodusAppError>{
        let mut session = Session::new();
        session.insert("user_id", user_id)?;

        let session_store = expect_context::<SqliteStore>();
        let cookie_value = session_store.store_session(session).await?.unwrap();
        Ok(cookie_value)
    }

    /// Destroy the Session if it exists
    pub async fn logout_session(cookie_value: &str)-> Result<(), LexodusAppError>{
        let store = expect_context::<SqliteStore>();
        let session = match store.load_session(cookie_value.to_string()).await?{
            Some(s) =>s,
            None => return Ok(())
        };
        store.destroy_session(session).await?;
        Ok(())
    }
}
}

#[tracing::instrument(level = "info", fields(error), ret, err)]
#[server(Login, "/api")]
pub async fn login(
    username: String,
    password: String,
    remember: Option<String>,
) -> Result<(), ServerFnError> {
    let Some(req) = use_context::<leptos_spin::RequestParts>() else {
        return Ok(());
    };
    let con = con()?;
    let user = auth_user(&username, &password, &con).await?;
    let session_cookie = create_session(user.id).await?;

    let res_options = expect_context::<ResponseOptions>();
    res_options.insert_header(
        "Set-Cookie",
        format!("Lexodus_session={session_cookie};Path=/;SameSite=Strict;").as_bytes(),
    );

    Ok(())
}

#[tracing::instrument(level = "info", fields(error), ret, err)]
#[server(Signup, "/api")]
pub async fn signup(
    username: String,
    display_name: String,
    password: String,
    password_confirmation: String,
    remember: Option<String>,
) -> Result<(), ServerFnError> {
    let Some(req) = use_context::<leptos_spin::RequestParts>() else {
        return Ok(());
    };
    let con = con()?;
    if password != password_confirmation {
        return Err(ServerFnError::ServerError(
            "Passwords did not match.".to_string(),
        ));
    }
    // Don't want anyone signing up...but us?
    // if username != "Lexodus" {
    //     println!("AH AH AH, YOU DIDN'T SAY THE MAGIC WORD");
    //     leptos_spin::redirect("/");
    //     return Ok(());
    // }

    let password_hashed = hash_password(password.as_bytes()).unwrap();
    con.execute(
        "INSERT INTO users (username, display_name, password) VALUES (?,?, ?)",
        &[
            Text(username.clone()),
            Text(display_name),
            Text(password_hashed),
        ],
    )
    .map_err(|e| ServerFnError::<LexodusAppError>::ServerError(e.to_string()))?;

    let user = match User::get_from_username(&username, &con)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
    {
        Some(u) => u,
        None => return Err(LexodusAppError::AuthError.into()),
    };

    leptos_spin::redirect("/cases");

    Ok(())
}

#[tracing::instrument(level = "info", fields(error), ret, err)]
#[server(Logout, "/api")]
pub async fn logout() -> Result<(), ServerFnError> {
    println!("LOGGING OUT");
    let Some(req) = use_context::<leptos_spin::RequestParts>() else {
        return Ok(());
    };
    let con = con()?;
    let Some(session) = get_session_cookie_value(&req)? else {
        return Ok(());
    };
    logout_session(&session).await?;

    // Delete session cookie by expiring it
    let res_parts = expect_context::<ResponseOptions>();
    res_parts.insert_header(
        "Set-Cookie",
        "Lexodus_session=no;Path=/; Expires=Thu, 01 Jan 1970 00:00:01 GMT;",
    );

    res_parts.insert_header(
        "Set-Cookie",
        "sessionid=no;Path=/; Expires=Thu, 01 Jan 1970 00:00:01 GMT;",
    );
    leptos_spin::redirect("/");

    Ok(())
}
