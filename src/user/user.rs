use crate::{prisma::PrismaClient, utils::jwt_token_utils::generate_token_by_id, ErrorMessage};
use actix_web::{post, web, HttpResponse, Responder};
use log::error;
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Validate)]
struct LoginDto {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1))]
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct LoginResponseDto {
    token: String,
    user_id: i32
}

#[derive(Deserialize, Validate)]
struct SignUpDto {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String,
    #[validate(length(min = 1))]
    pub name: String,
}

#[post("/api/auth/login")]
async fn login(body: web::Json<LoginDto>, client: web::Data<PrismaClient>) -> impl Responder {
    let user_found = client
        .user()
        .find_first(vec![crate::prisma::user::email::equals(body.email.clone())])
        .exec()
        .await.unwrap();

    if user_found.is_some(){
        let user = user_found.unwrap();

        //check if the passwords match
        if user.password != body.password {
            return HttpResponse::Unauthorized().json(json!({
                "success":false,
                "message":"User has either entered a wrong password or email"
            }));
        }


        let token_gen = generate_token_by_id(user.id.into(), user.email);
        match token_gen{
            Ok(res) =>{
                return HttpResponse::Ok().json(LoginResponseDto { token: res, user_id:user.id });
            },
            Err(err) =>{
                error!("Error occured {}", err);
                return HttpResponse::InternalServerError().into();
            }
        }
    }    

    return HttpResponse::Unauthorized().into();
}

#[post("/api/auth/signup")]
async fn sign_up(
    body: web::Json<SignUpDto>,
    client: web::Data<PrismaClient>,
) -> impl Responder {
    let create_user = client
        .user()
        .create(body.name.to_owned(), body.email.to_owned(), body.password.to_owned(), vec![])
        .exec()
        .await;

    match create_user {
        Ok(result) => {
            return HttpResponse::Ok().json(json!({
                "success":true,
                "user": result
            }));
        }
        Err(err) => {
            return HttpResponse::BadRequest().json(ErrorMessage {
                message: "User could not be created".into(),
                error: Some(err.to_string()),
            });
        }
    }
}


pub fn routes_config(cfg : &mut web::ServiceConfig){
    cfg.service(sign_up).service(login);
}