use std::error::Error;

use actix_web::{
    web::{Data, self},
    HttpResponse, Responder, cookie::{Cookie, time::Duration, SameSite},
};
use pwhash::sha512_crypt;

use super::{AppState, UserPwhash, UserLoginByEmail, UserLoginResult};
    
use crate::{error::Error as Err, api::ANSWER_OK, api::TOKEN};
use crate::api::v1::token;


pub async fn login(state: Data<AppState>, user: web::Json<UserLoginByEmail>) -> impl Responder {
    let user = user.into_inner();
    match sql_login_by_email(&state, user).await {
        Ok(id) => {
            let body = UserLoginResult { user_id: id };
            // TODO: get role from DB
            let role = if id == 1 { token::Role::Admin } else { token::Role::User };
            match token::create_jwt(&id.to_string(), role, &state.jwt_secret) {
                Ok(token) => HttpResponse::Ok()
                    .cookie(
                        Cookie::build(TOKEN, token)
                        .secure(true)
                        .http_only(true)
                        .path("/")
                        .same_site(SameSite::Strict)
                        .max_age(Duration::days(1))
                        .finish()
                    )
                    .body(serde_json::to_string(&body).unwrap_or_else(|_| "".to_string())),
                Err(_) => HttpResponse::InternalServerError()
                    .body(Err::JWTTokenCreationError.to_string())
            }
        },
        Err(e) => HttpResponse::Forbidden().body(e.to_string())
    }
}

async fn sql_login_by_email(state: &Data<AppState>, user: UserLoginByEmail) -> Result<i64, Box<dyn Error>> {
    let sql = "SELECT user_id, pwhash FROM users WHERE email = ($1)";
    let u = sqlx::query_as::<_, UserPwhash> (sql)
    .bind(user.email)
    .fetch_optional(&state.db)
    .await?;

    match u {
        Some(db_user) => match sha512_crypt::verify(user.passwd, &db_user.pwhash) {
            true => Ok(db_user.user_id),
            _ => Err(Err::WrongCredentialsError)?
        },
        _ => {
            // Just to make responces equal by text and by time
            //hash(user.passwd, DEFAULT_COST)?;
            Err(Err::NotFoundError)?
        }
    }
}

pub async fn logout() -> impl Responder {
    HttpResponse::Ok()
        .cookie(
            Cookie::build(TOKEN, "")
            .secure(true)
            .http_only(true)
            .path("/")
            .same_site(SameSite::Strict)
            .max_age(Duration::seconds(0))
            .finish()
        )
        .body(ANSWER_OK)
}
