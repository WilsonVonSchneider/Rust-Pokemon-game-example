use chrono::{Duration, Utc};
use error::Error;
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;
use rand::Rng;

pub fn generate_tokens(user_id: &str) -> Result<JwtTokens, Error> {
    // Generate refresh token
    let refresh_token_secret = &config::get_default("REFRESH_TOKEN_SECRET", "")[..];
    let expiration = (Utc::now() + Duration::days(1)).to_string();
    let refresh_token = generate_token(refresh_token_secret, user_id, expiration)?;

    // Generate access token
    let access_token_secret = &config::get_default("ACCESS_TOKEN_SECRET", "")[..];
    let expiration = (Utc::now() + Duration::seconds(90)).to_string();
    let access_token = generate_token(access_token_secret, user_id, expiration)?;

    // Return Tokens
    Ok(JwtTokens::new(refresh_token, access_token))
}

pub struct JwtTokens {
    pub refresh_token: String,
    pub access_token: String,
}
impl JwtTokens {
    pub fn new(refresh_token: String, access_token: String) -> Self {
        Self {
            refresh_token,
            access_token
        }
    }
    
}

// generate token function
pub fn generate_token(
    token_secret: &str,
    user_id: &str,
    expiration: String,
) -> Result<String, Error> {
    // generate key
    let key: Hmac<Sha256> = match Hmac::new_from_slice(token_secret.to_string().as_bytes()) {
        Ok(key) => key,
        Err(_) => return Err(Error::InternalError("Internal Error".to_string())),
    };
    // set claims
    let mut token_claims = BTreeMap::new();
    token_claims.insert("sub", user_id.to_string());
    token_claims.insert("exp", expiration);
    //generate token
    let token = match token_claims.sign_with_key(&key) {
        Ok(token) => token,
        Err(_) => return Err(Error::InternalError("Internal Error".to_string())),
    };
    Ok(token)
}

pub fn generate_random_action_token(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let characters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let token: String = (0..length)
        .map(|_| characters.chars().nth(rng.gen_range(0..characters.len())).unwrap())
        .collect();
    token
}