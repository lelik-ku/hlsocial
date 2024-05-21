
use std::error::Error;
use actix_web::{
    web::{Data, self},
    HttpResponse, Responder,
};
use chrono::NaiveDate;
use serde_json;
use pwhash::sha512_crypt;

use super::{AppState, User, UserCreate, UserCreateResult, UserUpdate};
use crate::{error::Error as Err, api::ANSWER_OK};


pub async fn users_get_all(state: Data<AppState>) -> impl Responder {
    match sql_users_get_all(&state).await {
        Ok(s) => match serde_json::to_string(&s) {
            Ok(j) => HttpResponse::Ok().body(j),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string())
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

async fn sql_users_get_all(state: &Data<AppState>) -> Result<Vec<User>, Box<dyn Error>> {
    let sql = "SELECT * FROM users ORDER BY user_id";
    let us = sqlx::query_as::<_, User> (sql)
    .fetch_all(&state.db)
    .await?;

    Ok(us)
}


pub async fn user_get(state: Data<AppState>, path: web::Path<i64>) -> impl Responder {
    let user_id = path.into_inner();
    match sql_users_get_by_id(&state, user_id).await {
        Ok(None) => HttpResponse::NotFound().body(Err::NotFoundError.to_string()),
        Ok(Some(s)) => match serde_json::to_string(&s) {
            Ok(j) => HttpResponse::Ok().body(j),
            Err(_) => HttpResponse::InternalServerError().body("Can't create JSON output")
        }
        Err(e) => HttpResponse::BadRequest().body(e.to_string())
    }
}

async fn sql_users_get_by_id(state: &Data<AppState>, user_id: i64) -> Result<Option<User>, Box<dyn Error>> {
    let sql = "SELECT * FROM users WHERE user_id = ($1)";
    let u = sqlx::query_as::<_, User> (sql)
    .bind(user_id)
    .fetch_optional(&state.db)
    .await?;

    Ok(u)
}


pub async fn user_delete(state: Data<AppState>, path: web::Path<i64>) -> impl Responder {
    let user_id = path.into_inner();
    match sql_users_delete_by_id(&state, user_id).await {
        Ok(0) => HttpResponse::NotFound().body(Err::NotFoundError.to_string()),
        Ok(_) => HttpResponse::Ok().body(ANSWER_OK),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

async fn sql_users_delete_by_id(state: &Data<AppState>, user_id: i64) -> Result<u64, Box<dyn Error>> {
    let sql = "DELETE FROM users WHERE user_id = ($1)";
    let res = sqlx::query(sql)
    .bind(user_id)
    .execute(&state.db)
    .await?;

    Ok(res.rows_affected())
}


pub async fn user_post(state: Data<AppState>, user: web::Json<UserCreate>) -> impl Responder {
    let user = user.into_inner();
    match sql_user_post(&state, user).await {
        Ok(id) => {
            let body = UserCreateResult { user_id: id };
            HttpResponse::Ok()
            .body(serde_json::to_string(&body).unwrap_or_else(|_| "".to_string()))
        },
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

async fn sql_user_post(state: &Data<AppState>, user: UserCreate) -> Result<i64, Box<dyn Error>> {
    let pwhash = sha512_crypt::hash(user.passwd)?;
    let birthdate = match &user.birthdate {
        Some(d) => {
            Some(NaiveDate::parse_from_str(&d, "%Y-%m-%dT%H:%M:%S.000Z").unwrap_or_default())
        },
        _ => None
    };
    let sql = "INSERT INTO users 
    (first_name, second_name, email, pwhash, gender, birthdate, biography, city)
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
    RETURNING user_id";
    let u = sqlx::query_as::<_, UserCreateResult>(sql)
    .bind(&user.first_name)
    .bind(&user.second_name)
    .bind(&user.email)
    .bind(&pwhash)
    .bind(&user.gender)
    .bind(&birthdate)
    .bind(&user.biography)
    .bind(&user.city)
    .fetch_one(&state.db)
    .await?;

    Ok(u.user_id)
}

pub async fn user_put(state: Data<AppState>, path: web::Path<i64>, user: web::Json<UserUpdate>) -> impl Responder {
    let mut user = user.into_inner();
    user.user_id = path.into_inner();
    match sql_user_put(&state, user).await {
        Ok(0) => HttpResponse::NotFound().body(Err::NotFoundError.to_string()),
        Ok(_) => HttpResponse::Ok().body(ANSWER_OK),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

async fn sql_user_put(state: &Data<AppState>, user: UserUpdate) -> Result<u64, Box<dyn Error>> {
    let birthdate = match &user.birthdate {
        Some(d) => {
            Some(NaiveDate::parse_from_str(&d, "%Y-%m-%dT%H:%M:%S.000Z").unwrap_or_default())
        },
        _ => None
    };
    let sql = "UPDATE users SET
    (first_name, second_name, email, gender, birthdate, biography, city)
    VALUES ($1, $2, $3, $4, $5, $6, $7) WHERE user_id = $8";
    let res = sqlx::query(sql)
    .bind(&user.first_name)
    .bind(&user.second_name)
    .bind(&user.email)
    .bind(&user.gender)
    .bind(&birthdate)
    .bind(&user.biography)
    .bind(&user.city)
    .bind(&user.user_id)
    .execute(&state.db)
    .await?;

    Ok(res.rows_affected())
}

pub async fn search(state: Data<AppState>, path: web::Path<String>) -> impl Responder {
    let text = path.into_inner();
    match sql_search(&state, text).await {
        Ok(s) if s.len() == 0 => HttpResponse::NotFound().body(Err::NotFoundError.to_string()),
        Ok(s) => match serde_json::to_string(&s) {
            Ok(s) => HttpResponse::Ok().body(s),            
            Err(e) => HttpResponse::InternalServerError().body(e.to_string())
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

async fn sql_search(state: &Data<AppState>, text: String) -> Result<Vec<User>, Box<dyn Error>> {
    let sql = "SELECT * FROM users 
    WHERE email LIKE ($1)
    OR first_name LIKE ($1)
    OR second_name LIKE ($1)
    OR city LIKE ($1)
    ORDER BY user_id";
    let us = sqlx::query_as::<_, User> (sql)
    .bind(format!("%{}%",text))
    .fetch_all(&state.db)
    .await?;

    Ok(us)
}
