use actix_web::body::EitherBody;
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::{
    header::{HeaderName, HeaderValue},
    Method,
};
use actix_web::web::Data;
use actix_web::Error;
use actix_web::HttpResponse;
use futures::future::{ok, LocalBoxFuture, Ready};
use log::{debug, error, info, warn};

use crate::constants::constants::{AUTHORIZATION, IGNORE_ROUTES};
use crate::prisma::PrismaClient;
use crate::utils::jwt_token_utils::*;
use crate::ErrorMessage;

pub struct Authentication;

impl<S, B> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthenticationMiddleware { service })
    }
}

pub struct AuthenticationMiddleware<S> {
    service: S,
}

struct AuthenticationResult {
    authenticate_pass: bool,
    message: String,
}

fn verification(authen_header: &HeaderValue, client: &Data<PrismaClient>) -> Result<bool, String> {
    let mut error_message: String = String::new();

    if is_auth_header_valid(authen_header) {
        let token_res = authen_header.to_str();
        match token_res {
            Ok(t_str) => {
                let token = t_str[6..t_str.len()].trim();
                let decoded_token_data = decode_token(token.into());
                match decoded_token_data {
                    Ok(token_data) => {
                        let token_verification =
                            futures::executor::block_on(verify_token(token_data, client));

                        match token_verification {
                            Ok(msg) => {
                                debug!("Authenticated:  {}", msg);
                                // authenticate_pass = true
                                return Ok(true);
                            }
                            Err(e) => {
                                // authenticate_pass = false;
                                error_message = "Token could not be verified!".to_string();
                                error!("{} reason: {}", error_message.clone(),e);
                                return Err(error_message);
                            }
                        }
                    }
                    Err(err) => {
                        error_message =
                            format!("Token could not be parsed due: {}", err.to_string());
                        debug!("{}", error_message);
                        return Err(error_message);
                    }
                }
            }
            Err(_) => {
                error_message = String::from("Auth header conversion to string failed");
                error!("{}", error_message);
                return Err(error_message);
            }
        }
    } else {
        error_message = String::from("Invalid token header");
        error!("{}", error_message);
        return Err(error_message);
    }
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let mut authenticate_pass: bool = false;
        let mut error_message: String = String::new();

        // Bypass some account routes
        let mut headers = req.headers().clone();
        headers.append(
            HeaderName::from_static("content-length"),
            HeaderValue::from_static("true"),
        );
        if Method::OPTIONS == *req.method() {
            authenticate_pass = true;
        } else {
            info!("Current path : {}", req.path());
            for ignore_route in IGNORE_ROUTES.iter() {
                if req.path().starts_with(ignore_route) {
                    authenticate_pass = true;
                    break;
                }
            }
        }

        if !authenticate_pass {
            //TODO - Connect to database
            // Parse the auth header
            // Decode the token
            // Verify and Validate the token by user id

            if let Some(client) = req.app_data::<Data<PrismaClient>>() {
                if let Some(authen_header) = req.headers().get(AUTHORIZATION) {
                    info!("Parsing authorization header...");
                    let verification_result = verification(authen_header, client);
                    match verification_result {
                        Ok(authenticated) => {
                            authenticate_pass = authenticated;
                        }
                        Err(err) => {
                            error!("{}", err);
                            error_message = err;
                        }
                    }
                } else {
                    error_message = String::from("Authentication header is missing");
                    warn!("{}", error_message);
                }
            }
        }

        if !authenticate_pass {
            let (request, _pl) = req.into_parts();
            let response = HttpResponse::Unauthorized()
                .json(ErrorMessage {
                    message: "Unauthorized access".into(),
                    error: Some(String::from(error_message.clone())),
                })
                .map_into_right_body();

            return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
        }

        let res = self.service.call(req);

        Box::pin(async move { res.await.map(ServiceResponse::map_into_left_body) })
    }
}
