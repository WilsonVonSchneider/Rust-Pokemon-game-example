use actix_web::{HttpResponse, ResponseError};
use diesel::result::Error as DieselError;
use r2d2::Error as R2D2Error;
use reqwest::Error as ReqError;
use std::{convert::From, fmt};
use validr::error::ValidationErrors;
use lettre::transport::smtp::Error as LettreError;

#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
pub enum Error {
    // storage / file upload error
    NotFound,
    NotFoundWithCause(String),
    BadRequest(String),
    R2D2Error(R2D2Error),
    Diesel(DieselError),
    Validation(ValidationErrors),
    Reqwest(ReqError),
    InternalError(String),
    InternalServerError(String),
    Lettre(LettreError),
    Forbidden(String),
    Unauthorized(String),
    WrongGuess(String)
}

// Allow the use of "{}" format specifier
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::NotFound => write!(f, "Not Found"),
            Error::NotFoundWithCause(ref cause) => write!(f, "Not found: {cause}"),
            Error::R2D2Error(ref cause) => write!(f, "DB Pool Error: {cause}"),
            Error::BadRequest(ref cause) => write!(f, "Bad request: {}", cause),
            Error::Diesel(ref cause) => write!(f, "DB Error: {cause}"),
            Error::Validation(ref cause) => {
                write!(f, "Validation error: {cause}")
            }
            Error::Reqwest(ref cause) => {
                write!(f, "External API error: {cause}")
            }
            Error::InternalError(ref cause) => write!(f, "{cause}"),
            Error::InternalServerError(ref cause) => write!(f, "{cause}"),
            Error::Lettre(ref cause) => write!(f, "Mail Error: {cause}"),
            Error::Forbidden(ref cause) => write!(f, "Forbidden error: {cause}"),
            Error::Unauthorized(ref cause) => write!(f, "Authorization error: {cause}"),
            Error::WrongGuess(ref cause) => write!(f, "Wrong guess: {cause}"),
        }
    }
}

impl From<R2D2Error> for Error {
    fn from(cause: R2D2Error) -> Error {
        Error::R2D2Error(cause)
    }
}

impl From<DieselError> for Error {
    fn from(cause: DieselError) -> Error {
        if cause == DieselError::NotFound {
            return Error::NotFound;
        }
        Error::Diesel(cause)
    }
}

impl From<ValidationErrors> for Error {
    fn from(cause: ValidationErrors) -> Error {
        Error::Validation(cause)
    }
}

impl From<ReqError> for Error {
    fn from(cause: ReqError) -> Error {
        Error::Reqwest(cause)
    }
}

impl From<LettreError> for Error {
    fn from(cause: LettreError) -> Error {
        Error::Lettre(cause)
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        let mut response = match self {
            Error::NotFound => HttpResponse::NotFound(),
            Error::NotFoundWithCause(_) => HttpResponse::NotFound(),
            Error::Forbidden(_) => HttpResponse::Forbidden(),
            Error::Unauthorized(_) => HttpResponse::Unauthorized(),
            Error::BadRequest(_) => HttpResponse::BadRequest(),
            Error::Validation(_) => HttpResponse::UnprocessableEntity(),
            Error::R2D2Error(_) => HttpResponse::BadGateway(),
            Error::WrongGuess(_) => HttpResponse::Ok(),
            _ => HttpResponse::InternalServerError(),
        };

        response.json(self.into_error_body())
    }
}

impl Error {
    pub fn add_cause_if_not_found(self, cause: &str) -> Error {
        match &self {
            Error::NotFound => Error::NotFoundWithCause(cause.to_string()),
            Error::NotFoundWithCause(_) => Error::NotFoundWithCause(cause.to_string()),
            _ => self,
        }
    }

    pub fn is_not_found(&self) -> bool {
        matches!(self, Error::NotFound | Error::NotFoundWithCause(_))
    }

    pub fn is_validation(&self) -> bool {
        matches!(self, Error::Validation(_))
    }

    pub fn into_error_body(&self) -> ErrorBody {
        match *self {
            Error::NotFound => ErrorBody {
                message: Some("Entity not found".to_string()),
                code: "not_found".to_string(),
                cause: None,
                payload: None,
            },
            Error::NotFoundWithCause(ref cause) => ErrorBody {
                message: Some("Entity not found".to_string()),
                code: "not_found".to_string(),
                cause: Some(cause.to_string()),
                payload: None,
            },
            Error::BadRequest(ref cause) => ErrorBody {
                message: Some("Bad request".to_string()),
                code: "general".to_string(),
                cause: Some(cause.to_string()),
                payload: None,
            },
            Error::R2D2Error(ref cause) => ErrorBody {
                message: Some("DB Pool Error".to_string()),
                code: "no_connections_available".to_string(),
                cause: Some(cause.to_string()),
                payload: None,
            },
            Error::Diesel(ref cause) => ErrorBody {
                message: Some("DB Error".to_string()),
                code: "db".to_string(),
                cause: Some(cause.to_string()),
                payload: None,
            },
            Error::Validation(ref errors) => ErrorBody {
                message: Some("invalid-body".to_owned()),
                code: 400.to_string(),
                cause: Some("invalid-body".to_owned()),
                payload: Some(serde_json::to_value(errors.get_errors()).unwrap()),
            },
            Error::Reqwest(ref cause) => ErrorBody {
                message: Some("External API error".to_string()),
                code: 500.to_string(),
                cause: Some(cause.to_string()),
                payload: None,
            },
            Error::InternalError(ref cause) => ErrorBody {
                message: Some("Something went wrong".to_string()),
                code: "server_error".to_string(),
                cause: Some(cause.to_string()),
                payload: None,
            },
            Error::InternalServerError(ref cause) => ErrorBody {
                message: Some("Something went wrong".to_string()),
                code: "server_error".to_string(),
                cause: Some(cause.to_string()),
                payload: None,
            },
            Error::Lettre(ref cause) => ErrorBody {
                message: Some("External API error".to_string()),
                code: 500.to_string(),
                cause: Some(cause.to_string()),
                payload: None,
            },
            Error::Unauthorized(ref cause) => ErrorBody {
                message: Some("Unauthorized error".to_string()),
                code: "unauthorized".to_string(),
                cause: Some(cause.to_string()),
                payload: None,
            },
            Error::Forbidden(ref cause) => ErrorBody {
                message: Some("Forbidden error".to_string()),
                code: "forbidden".to_string(),
                cause: Some(cause.to_string()),
                payload: None,
            },
            Error::WrongGuess(ref cause) => ErrorBody {
                message: Some("Try again".to_string()),
                code: "wrong guess".to_string(),
                cause: Some(cause.to_string()),
                payload: None,
            },

        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ErrorBody {
    pub message: Option<String>,
    pub code: String,
    pub cause: Option<String>,
    pub payload: Option<serde_json::Value>,
}
