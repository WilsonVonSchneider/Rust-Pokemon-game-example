use super::data::RequestLoginData;
use actix_web::{web, ResponseError};
use http::StatusCode;
use validr::*;

#[allow(dead_code)]
async fn test_actix_route_handler_login_attributes(
    test: web::Json<RequestLoginData>,
) -> StatusCode {
    match test.into_inner().validate(){
        Ok(_) => http::StatusCode::OK,
        Err(e) => e.error_response().status(),
    }
}

#[actix_web::main]
#[test]
async fn test_request_login_failed_validation_empty_required_field_email() {
    let data = RequestLoginData {
        email: None,
        password: Some("12345Abcdefgh.".to_string()),
    };
    let response = test_actix_route_handler_login_attributes(web::Json(data)).await;
    assert_eq!(response, http::StatusCode::UNPROCESSABLE_ENTITY)
}

#[actix_web::main]
#[test]
async fn test_request_login_failed_validation_field_email_format() {
    let data = RequestLoginData {
        email: Some("example_email".to_string()),
        password: Some("12345Abcdefgh.".to_string()),
    };
    let response = test_actix_route_handler_login_attributes(web::Json(data)).await;
    assert_eq!(response, http::StatusCode::UNPROCESSABLE_ENTITY)
}

#[actix_web::main]
#[test]
async fn test_request_login_failed_validation_empty_required_field_password() {
    let data = RequestLoginData {
        email: Some("example@example.net".to_string()),
        password: None,
    };
    let response = test_actix_route_handler_login_attributes(web::Json(data)).await;
    assert_eq!(response, http::StatusCode::UNPROCESSABLE_ENTITY)
}

