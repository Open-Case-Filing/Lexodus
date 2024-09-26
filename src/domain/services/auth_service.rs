// src/domain/services/auth_service.rs
// src/domain/services/mod.rs
use leptos::*;
use jsonwebtoken::{decode, DecodingKey, Validation, errors::Error as JwtError};
use serde::{Deserialize, Serialize};
use anyhow::Result;
use crate::domain::repositories::user_repository::UserRepository;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,  // user_id
    exp: usize,   // expiration time
}

pub struct AuthService<UR: UserRepository> {
    user_repo: UR,
    jwt_secret: String,
}

impl<UR: UserRepository> AuthService<UR> {
    pub fn new(user_repo: UR, jwt_secret: String) -> Self {
        Self { user_repo, jwt_secret }
    }

    pub async fn verify_token(&self, token: &str) -> Result<i64> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_ref()),
            &Validation::default()
        ).map_err(|e: JwtError| anyhow::anyhow!("Token verification failed: {}", e))?;

        let user_id: i64 = token_data.claims.sub.parse()
            .map_err(|_| anyhow::anyhow!("Invalid user ID in token"))?;

        // Optionally, verify if the user still exists and is active
        if let Some(_user) = self.user_repo.find_by_id(user_id).await? {
            Ok(user_id)
        } else {
            Err(anyhow::anyhow!("User not found or inactive"))
        }
    }

    // Additional methods like create_token, refresh_token, etc. can be added here
}
