use std::error::Error;
use actix_web::{
    get,
    web::{Data, self},
    HttpResponse, Responder,
};
use sqlx::FromRow;
use serde::Serialize;
use serde_json;

use super::AppState;


#[derive(Serialize, FromRow, Debug)]
pub struct User {
    user_id: i64,
    first_name: Option<String>,
    second_name: Option<String>,
    // pwhash: String,
    email: String,
    gender: Option<String>,
    birthdate: Option<String>,
    biography: Option<String>,
    city: Option<String>,
}

pub async fn sql_users_get(state: Data<AppState>) -> Result<Vec<User>, Box<dyn Error>> {
    let sql = "SELECT * FROM users";
    let us = sqlx::query_as::<_, User> (sql)
    .fetch_all(&state.db)
    .await?;

    Ok(us)
}

pub async fn sql_user_get(state: Data<AppState>, user_id: i64) -> Result<Option<User>, Box<dyn Error>> {
    let sql = "SELECT * FROM users WHERE user_id = ($1)";
    let u = sqlx::query_as::<_, User> (sql)
    .bind(user_id)
    .fetch_optional(&state.db)
    .await?;

    Ok(u)
}

#[get("/users")]
async fn users_get(state: Data<AppState>) -> impl Responder {
    match sql_users_get(state).await {
        Ok(s) => match serde_json::to_string(&s) {
            Ok(j) => HttpResponse::Ok().body(j),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string())
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

#[get("/user/{user_id}")]
async fn user_get(state: Data<AppState>, path: web::Path<i64>) -> impl Responder {
    let user_id = path.into_inner();
    match sql_user_get(state, user_id).await {
        Ok(s) => match serde_json::to_string(&s) {
            Ok(j) => HttpResponse::Ok().body(j),
            Err(_) => HttpResponse::InternalServerError().body("Can't create JSON output")
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}
