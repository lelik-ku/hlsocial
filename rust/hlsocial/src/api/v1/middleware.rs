use std::future::{ready, Ready};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, error::ErrorUnauthorized, error::ErrorForbidden, http,
};
use futures_util::future::LocalBoxFuture;

use crate::{error::Error as Err, api::TOKEN, config};


use super::token::{validate_jwt, Role};


// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct Auth;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service }))
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        match req.path() {
            "/v1/login" | "/v1/login/" | "/v1/logout" | "/v1/logout/" | "/v1/register" | "/v1/register/" => {},
            _ => {
                match req.cookie(TOKEN) {
                    Some(token) => {
                        let config = config::config();
                        let jwt_secret = config.jwt_secret;
                        match validate_jwt(token.value(), &jwt_secret) {
                            Ok(Role::User) => {
                                if req.method() == http::Method::DELETE || req.method() == http::Method::PUT {
                                    return Box::pin(async { Err(ErrorForbidden("Not allowed for users"))} )}
                                }
                            Ok(_) => {},
                            Err(e) => return Box::pin(async { Err(ErrorUnauthorized(e)) })
                        }
                    },
                    _ => return Box::pin(async { Err(ErrorUnauthorized(Err::Unauthorized)) })
                };
            }
        };

        let fut = self.service.call(req);

        Box::pin(async move {
            Ok(fut.await?)
        })
    }
}
