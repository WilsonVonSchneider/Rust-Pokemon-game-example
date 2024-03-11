use crate::application::auth::login::contract::LoginContract;
use actix_web::{web, HttpResponse, HttpRequest, cookie::{Cookie, time}};
use support::helpers::http;
use error::Error;


pub async fn handle_logout<T: LoginContract>(
    service: web::Data<T>,
    request: HttpRequest
) -> Result<HttpResponse, Error> {
    // retrieve refresh token from cookie
    let refresh_token = http::get_token_from_cookie(&request)?;
    // logout
    service.logout(&refresh_token).await?;
    // expire refresh token
    let remove_cookie = Cookie::build("refresh_token", refresh_token).path("/").max_age(time::Duration::minutes(0)).finish();
    // set expired refresh token to cookie (it will effectivly delete refresh token cookie)
    Ok(HttpResponse::Ok().cookie(remove_cookie).json("user logged out"))
}