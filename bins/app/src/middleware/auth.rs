use actix_web::{
    body::MessageBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use error::Error as AuthError;
use futures::future;
use std::{
    cell::RefCell,
    rc::Rc,
    task::{Context, Poll},
};
use support::helpers::verify_access_token;

pub struct Auth {
    skip: Vec<String>,
}

impl Auth {
    pub fn new() -> Self {
        Auth { skip: vec![] }
    }
}

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    B: MessageBody,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        future::ok(AuthMiddleware::<S> {
            service: Rc::new(RefCell::new(service)),
            skip: self.skip.clone(),
        })
    }
}

pub struct AuthMiddleware<S> {
    service: std::rc::Rc<std::cell::RefCell<S>>,
    skip: Vec<String>,
}

impl Auth {
    pub fn add_ignored_routes(mut self, mut routes: Vec<String>) -> Auth {
        self.skip.append(&mut routes);
        self
    }
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    B: MessageBody,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future =
        future::Either<future::Ready<Result<ServiceResponse<B>, actix_web::Error>>, S::Future>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let skip = self.skip.contains(&req.match_pattern().unwrap_or_default());
        if skip {
            return future::Either::Right(self.service.call(req));
        }

        let token: Option<String> = req
            .headers()
            .get("Authorization")
            .and_then(|header_value| header_value.to_str().ok())
            .and_then(|header_str| {
                header_str.strip_prefix("Bearer ").map(|end| end.to_string())
            });
        let token = match token {
            None => {
                return future::Either::Left(future::err(
                    AuthError::Unauthorized("Unauthorized".to_string()).into(),
                ));
            }
            Some(value) => value,
        };
        //verify access token
        let auth_user_id = match verify_access_token::verify_access_token(&token) {
            Ok(user_id) => user_id,
            Err(e) => match e {
                AuthError::Unauthorized(_) => {
                    return future::Either::Left(future::err(
                        AuthError::Unauthorized("Unauthorized".to_string()).into(),
                    ));
                }
                _ => {
                    return future::Either::Left(future::err(
                        AuthError::InternalError("Internal error".to_string()).into(),
                    ));
                }
            },
        };
        req.extensions_mut().insert(auth_user_id);
        let fut = self.service.call(req);
        future::Either::Right(fut)
    }
}
