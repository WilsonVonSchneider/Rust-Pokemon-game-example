use crate::store::models::user::User;
use chrono::{NaiveDateTime, Utc};
use error::Error;
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;

pub fn verify_refresh_token(user: &User, refresh_token: &str) -> Result<(), Error> {
    // get refresh token secret from config
    let refresh_token_secret = &config::get_default("REFRESH_TOKEN_SECRET", "")[..];
    // set key for token verification
    let key: Hmac<Sha256> =
        Hmac::new_from_slice(refresh_token_secret.to_string().as_bytes()).unwrap();
    // verify refresh token and get claims
    let claims: BTreeMap<String, String> = match refresh_token.verify_with_key(&key) {
        Ok(claims) => claims,
        Err(_) => {
            return Err(Error::Unauthorized("Unathorized".to_string()));
        }
    };
    // get token_expires claim
    let token_expires = match claims.get("exp") {
        Some(exp) => exp,
        None => {
            return Err(Error::InternalError("Internal error".to_string()));
        }
    };
    // get sub claim
    let token_sub = match claims.get("sub") {
        Some(sub) => sub,
        None => {
            return Err(Error::InternalError("Internal error".to_string()));
        }
    };
    // turn token_expires claim from string to NaiveDateTime format
    let token_expires =
        match NaiveDateTime::parse_from_str(token_expires, "%Y-%m-%d %H:%M:%S%.f %Z") {
            Ok(exp) => exp,
            Err(_) => {
                return Err(Error::InternalError("Internal error".to_string()));
            }
        };
    // if token is expired return unauthorized
    if token_expires <= Utc::now().naive_utc() {
        return Err(Error::Unauthorized("Unathorized".to_string()));
    }
    // if user_id doesn't match sub claim return unauthorized
    if &user.id != token_sub {
        return Err(Error::Unauthorized("Unathorized".to_string()));
    }
    Ok(())
}
