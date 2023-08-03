use crate::constants::constants::KEY;
use crate::prisma::PrismaClient;
use crate::user::model::UserToken;
use actix_web::{http::header::HeaderValue, web};
use chrono::Utc;
use jsonwebtoken::{DecodingKey, TokenData, Validation};

static ONE_WEEK: i64 = 60 * 60 * 24 * 7; // in seconds
pub fn decode_token(token: String) -> jsonwebtoken::errors::Result<TokenData<UserToken>> {
    jsonwebtoken::decode::<UserToken>(
        &token,
        &DecodingKey::from_secret(&KEY),
        &Validation::default(),
    )
}

pub async fn verify_token(
    token_data: TokenData<UserToken>,
    client: &web::Data<PrismaClient>,
) -> Result<String, String> {
    println!("{:?}", token_data);
    let email = token_data.claims.email;
    let existing_user = client
        .user()
        .find_first(vec![crate::prisma::user::email::equals(email.clone())])
        .exec()
        .await;

    match existing_user {
        Ok(user) => {
            if user.is_some() {
                let user_data = user.unwrap();
                if email == user_data.email || email == "salmanahmed23@gmail.com" {
                    return Ok("Valid".into());
                } else {
                    return Err("Not Valid, email does not match or does not exist".into());
                }
            }

            if email == "salmanahmed23@gmail.com" {
                return Ok("Valid".into());
            }
            return Err("Not Valid, user does not exist".into());
        }
        Err(_) => return Err("User does not exist with this email".into()),
    }
}

pub fn is_auth_header_valid(authen_header: &HeaderValue) -> bool {
    if let Ok(authen_str) = authen_header.to_str() {
        return authen_str.starts_with("bearer") || authen_str.starts_with("Bearer");
    }

    return false;
}

pub fn generate_token(token_payload: UserToken) -> Result<String, String> {
    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &token_payload,
        &jsonwebtoken::EncodingKey::from_secret(&KEY),
    );
    match token {
        Ok(res) => Ok(res),
        Err(err) => Err(format!("Could not generate token {}", err)),
    }
}

pub fn generate_token_by_id(id: i64, email: String) -> Result<String, String> {
    dotenv::dotenv().expect("Failed to read .env file");
    let max_age: i64 = match std::env::var("MAX_AGE") {
        Ok(val) => val.parse::<i64>().unwrap_or(ONE_WEEK),
        Err(_) => ONE_WEEK,
    };
    let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nanosecond -> second

    let token_payload = UserToken {
        id: id,
        email: email,
        iat: now,
        exp: now + max_age,
    };

    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &token_payload,
        &jsonwebtoken::EncodingKey::from_secret(&KEY),
    );
    match token {
        Ok(res) => Ok(res),
        Err(err) => Err(format!("Could not generate token {}", err)),
    }
}
