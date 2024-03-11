use crate::application::auth::login::{
    contract::LoginContract, data::{RequestLoginData, ResponseData}
};
use actix_web::{web, HttpResponse, cookie::Cookie};
use error::Error;
use validr::Validation;

pub async fn handle_login<T: LoginContract>(
    service: web::Data<T>,
    data: web::Json<RequestLoginData>,
) -> Result<HttpResponse, Error> {
    // validate login data
    let data = data.into_inner().validate()?.insertable();
    // create jwt tokens
    let jwt_tokens = service.login(data).await?;
    // create response
    let response = ResponseData::new(jwt_tokens.access_token);
    // store refresh token as a HTTP cookie
    let refresh_token_cookie = Cookie::build("refresh_token", jwt_tokens.refresh_token).path("/").finish();
    // respond with access token and set refresh token cookie
    Ok(HttpResponse::Ok().cookie(refresh_token_cookie).json(response))
}