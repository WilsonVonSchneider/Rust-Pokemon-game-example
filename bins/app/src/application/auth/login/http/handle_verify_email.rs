use crate::application::auth::login::{
    contract::LoginContract, data::ResponseData
};
use actix_web::{web, HttpResponse, cookie::Cookie, HttpRequest};
use error::Error;
use support::helpers::http::part_from_path;


pub async fn handle_verify_email<T: LoginContract>(
    service: web::Data<T>,
    request: HttpRequest
) -> Result<HttpResponse, Error> {
    // extract action token id from url
    let action_token = part_from_path::<String>(&request, "action_token")?;
    // check action token
    let action_token = service.check_action_token(&action_token).await?;
    // verify email and login
    let jwt_tokens = service.verify_email_and_login(action_token).await?;
    //create response
    let response = ResponseData::new(jwt_tokens.access_token);
    //store refresh token as a HTTP cookie
    let refresh_token_cookie = Cookie::build("refresh_token", jwt_tokens.refresh_token).path("/").finish();
    //respond with access token and set refresh token cookie
    Ok(HttpResponse::Ok().cookie(refresh_token_cookie).json(response))
}