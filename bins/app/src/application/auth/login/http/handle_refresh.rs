use crate::application::auth::login::{
    contract::LoginContract, data::ResponseData
};
use actix_web::{web, HttpResponse, HttpRequest};
use support::helpers::http;
use error::Error;


pub async fn handle_refresh<T: LoginContract>(
    service: web::Data<T>,
    request: HttpRequest
) -> Result<HttpResponse, Error> {
    // retrieve refresh token from cookie
    let refresh_token = http::get_token_from_cookie(&request)?;
    // refresh access token
    let access_token = service.refresh(&refresh_token).await?;
    // create response
    let response = ResponseData::new(access_token);
    Ok(HttpResponse::Ok().json(response))
}