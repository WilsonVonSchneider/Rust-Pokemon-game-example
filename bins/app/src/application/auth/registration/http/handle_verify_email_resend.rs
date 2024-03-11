use crate::application::auth::registration::{
    contract::RegisterContract, data::RequestEmailData
};
use actix_web::{http::header, web, HttpRequest, HttpResponse};
use error::Error;
use support::helpers::base_url;
use validr::Validation;

pub async fn handle_verify_email_resend<T: RegisterContract>(
    service: web::Data<T>,
    data: web::Json<RequestEmailData>,
    request: HttpRequest,
) -> Result<HttpResponse, Error> {
    // get base_url from request
    let base_url = base_url::get_base_url(&request);
    // validate the data
    let user_email = data.into_inner().validate()?.email.unwrap();
    // create action token. Return action token so id can be set to header
    let action_token = service.create_action_token(&user_email, &base_url).await?;
    //create response
    let mut response = HttpResponse::Created().finish();
    // If IS_DEV is true set email_token to response header for testing purpose
    if &config::get_default("IS_DEV", "")[..] == "true" {
        response.headers_mut().insert(
            header::HeaderName::from_static("email_token"),
            header::HeaderValue::from_str(&action_token.token)
            .expect("Failed to create header value"),
        );
    } 
    // return response
    Ok(response)
}
