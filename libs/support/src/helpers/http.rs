use actix_web::{HttpRequest, HttpMessage};
use error::Error;
use std::str::FromStr;


/// Parse given parameter easily from the request path string
pub fn part_from_path<T: FromStr>(req: &HttpRequest, name: &str) -> Result<T, Error> {
    let value: T = match req.match_info().get(name) {
        Some(val) => match val.parse() {
            Ok(v) => v,
            Err(_) => return Err(Error::BadRequest(format!("path_attribute_missing:{name}"))),
        },
        None => return Err(Error::BadRequest(format!("path_attribute_missing:{name}"))),
    };

    Ok(value)
}

// Get refresh token from http cookie
pub fn get_token_from_cookie(req: &HttpRequest) -> Result<String, Error> {
    match req.cookie("refresh_token") {
        Some(cookie) => {
            Ok(cookie.value().to_string())
        }
        None => {
            Err(Error::Unauthorized("Unauthorized".to_string()))
        }
    }
}

// Retrieve authenticated user id from request
pub fn get_authenticated_user_id_from_request(req: &HttpRequest) -> Result<String, Error> {
    match req.extensions().get::<String>() {
        Some(user_id) => Ok(user_id.clone()),
        None => Err(Error::Unauthorized("Unauthorized".to_string())),
    }
}

/// Get api version from the route path
pub fn get_api_version(req: &HttpRequest) -> String {
    match req.match_info().get("version") {
        Some(value) => value.parse().unwrap_or_else(|_| "v1".to_string()),
        None => "v1".to_string(),
    }
}


