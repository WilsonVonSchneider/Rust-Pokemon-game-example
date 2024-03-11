use super::data::{RequestUserData, RequestEmailData};
use actix_web::{web, ResponseError};
use http::StatusCode;
use validr::*;

#[allow(dead_code)]
async fn test_actix_route_handler_user_attributes(
    test: web::Json<RequestUserData>,
) -> StatusCode {
    match test.into_inner().validate(){
        Ok(_) => http::StatusCode::OK,
        Err(e) => e.error_response().status(),
    }
}

#[allow(dead_code)]
async fn test_actix_route_handler_email_attributes(
    test: web::Json<RequestEmailData>,
) -> StatusCode {
    match test.into_inner().validate(){
        Ok(_) => http::StatusCode::OK,
        Err(e) => e.error_response().status(),
    }
}

#[actix_web::main]
#[test]
async fn test_request_create_new_user_failed_validation_empty_required_field_email() {
    let data = RequestUserData {
        email: None,
        first_name: Some("example_name".to_string()),
        last_name: Some("example_last_name".to_string()),
        password: Some("12345Abcdefgh.".to_string()),
    };
    let response = test_actix_route_handler_user_attributes(web::Json(data)).await;
    assert_eq!(response, http::StatusCode::UNPROCESSABLE_ENTITY)
}

#[actix_web::main]
#[test]
async fn test_request_create_new_user_failed_validation_email_format() {
    let data = RequestUserData {
        email: Some("example_email".to_string()),
        first_name: Some("example_name".to_string()),
        last_name: Some("example_last_name".to_string()),
        password: Some("12345Abcdefgh.".to_string()),
    };
    let response = test_actix_route_handler_user_attributes(web::Json(data)).await;
    assert_eq!(response, http::StatusCode::UNPROCESSABLE_ENTITY)
}

#[actix_web::main]
#[test]
async fn test_request_create_new_user_failed_validation_empty_required_field_first_name() {
    let data = RequestUserData {
        email: Some("example@email.com".to_string()),
        first_name: None,
        last_name: Some("example_last_name".to_string()),
        password: Some("12345Abcdefgh.".to_string()),
    };
    let response = test_actix_route_handler_user_attributes(web::Json(data)).await;
    assert_eq!(response, http::StatusCode::UNPROCESSABLE_ENTITY)
}

#[actix_web::main]
#[test]
async fn test_request_create_new_user_failed_validation_empty_required_field_last_name() {
    let data = RequestUserData {
        email: Some("example@email.com".to_string()),
        first_name: Some("example_first_name".to_string()),
        last_name: None,
        password: Some("12345Abcdefgh.".to_string()),
    };
    let response = test_actix_route_handler_user_attributes(web::Json(data)).await;
    assert_eq!(response, http::StatusCode::UNPROCESSABLE_ENTITY)
}

#[actix_web::main]
#[test]
async fn test_request_create_new_user_failed_validation_empty_required_field_password() {
    let data = RequestUserData {
        email: Some("example@email.com".to_string()),
        first_name: Some("example_first_name".to_string()),
        last_name: Some("example_last_name".to_string()),
        password: None,
    };
    let response = test_actix_route_handler_user_attributes(web::Json(data)).await;
    assert_eq!(response, http::StatusCode::UNPROCESSABLE_ENTITY)
}

#[actix_web::main]
#[test]
async fn test_request_create_new_user_failed_validation_password_8_characters_long() {
    let data = RequestUserData {
        email: Some("example@email.com".to_string()),
        first_name: Some("example_first_name".to_string()),
        last_name: Some("example_last_name".to_string()),
        password: Some("123Qwe.".to_string()),
    };
    let response = test_actix_route_handler_user_attributes(web::Json(data)).await;
    assert_eq!(response, http::StatusCode::UNPROCESSABLE_ENTITY)
}

#[actix_web::main]
#[test]
async fn test_request_create_new_user_failed_validation_password_weak() {
    let data = RequestUserData {
        email: Some("example@email.com".to_string()),
        first_name: Some("example_first_name".to_string()),
        last_name: Some("example_last_name".to_string()),
        password: Some("12345678".to_string()),
    };
    let response = test_actix_route_handler_user_attributes(web::Json(data)).await;
    assert_eq!(response, http::StatusCode::UNPROCESSABLE_ENTITY)
}


#[actix_web::main]
#[test]
async fn test_request_resend_verification_email_failed_validation_empty_required_field_email() {
    let data = RequestEmailData {
        email: None,
    };
    let response = test_actix_route_handler_email_attributes(web::Json(data)).await;
    assert_eq!(response, http::StatusCode::UNPROCESSABLE_ENTITY)
}

#[actix_web::main]
#[test]
async fn test_request_resend_verification_email_failed_validation_email_format() {
    let data = RequestEmailData {
        email: Some("example".to_string()),
    };
    let response = test_actix_route_handler_email_attributes(web::Json(data)).await;
    assert_eq!(response, http::StatusCode::UNPROCESSABLE_ENTITY)
}