use std::error::Error;
use actix_web::{
    get, post, put, delete,
    web::{Data, self},
    HttpResponse, Responder,
};
use sqlx::FromRow;
use chrono::NaiveDate;
use serde::{Serialize, Deserialize};
use serde_json;

use super::AppState;


#[derive(Serialize, FromRow, Debug)]
pub struct User {
    user_id: i64,
    first_name: Option<String>,
    second_name: Option<String>,
    email: String,
    gender: Option<String>,
    birthdate: Option<NaiveDate>,
    biography: Option<String>,
    city: Option<String>,
}

#[derive(Deserialize, FromRow, Debug)]
pub struct UserCreate {
    first_name: Option<String>,
    second_name: Option<String>,
    pwhash: String,
    email: String,
    gender: Option<String>,
    birthdate: Option<NaiveDate>,
    biography: Option<String>,
    city: Option<String>,
}

#[derive(Deserialize, FromRow, Debug)]
pub struct UserCreateResult {
    user_id: i64,
}

pub async fn sql_users_get_all(state: Data<AppState>) -> Result<Vec<User>, Box<dyn Error>> {
    let sql = "SELECT * FROM users";
    let us = sqlx::query_as::<_, User> (sql)
    .fetch_all(&state.db)
    .await?;

    Ok(us)
}

pub async fn sql_users_get_by_ids(state: Data<AppState>, user_id: i64) -> Result<Option<User>, Box<dyn Error>> {
    let sql = "SELECT * FROM users WHERE user_id = ($1)";
    let u = sqlx::query_as::<_, User> (sql)
    .bind(user_id)
    .fetch_optional(&state.db)
    .await?;

    Ok(u)
}

pub async fn sql_users_delete_by_ids(state: Data<AppState>, user_id: i64) -> Result<u64, Box<dyn Error>> {
    let sql = "DELETE FROM users WHERE user_id = ($1)";
    let res = sqlx::query(sql)
    .bind(user_id)
    .execute(&state.db)
    .await?;

    Ok(res.rows_affected())
}

pub async fn sql_user_post(state: Data<AppState>, user: UserCreate) -> Result<i64, Box<dyn Error>> {
    let sql = "INSERT INTO users 
    (first_name, second_name, pwhash, email, gender, birthdate, biography, city)
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
    RETURNING user_id";
    let u = sqlx::query_as::<_, UserCreateResult>(sql)
    .bind(&user.first_name)
    .bind(&user.second_name)
    .bind(&user.pwhash)
    .bind(&user.email)
    .bind(&user.gender)
    .bind(&user.birthdate)
    .bind(&user.biography)
    .bind(&user.city)
    .fetch_one(&state.db)
    .await?;

    Ok(u.user_id)
}

// pub async fn sql_user_put(state: Data<AppState>, user: User) -> Result<(), Box<dyn Error>> {
//     let sql = "UPDATE users SET 
//     (first_name, second_name, email, gender, birthdate, biography, city)
//     VALUES ($1, $2, $3, $4, $5, $6, $7, $8) WHERE user_id = $9";
//     sqlx::query_as::<_, User> (sql)
//     .bind(&user.first_name)
//     .bind(&user.second_name)
//     .bind(&user.email)
//     .bind(&user.gender)
//     .bind(&user.birthdate)
//     .bind(&user.biography)
//     .bind(&user.city)
//     .bind(&user.user_id)
//     .fetch_optional(&state.db)
//     .await?;

//     Ok(())
// }

#[get("/users")]
async fn users_get_all(state: Data<AppState>) -> impl Responder {
    match sql_users_get_all(state).await {
        Ok(s) => match serde_json::to_string(&s) {
            Ok(j) => HttpResponse::Ok().body(j),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string())
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

#[get("/users/{user_id}")]
async fn user_get(state: Data<AppState>, path: web::Path<i64>) -> impl Responder {
    let user_id = path.into_inner();
    match sql_users_get_by_ids(state, user_id).await {
        Ok(None) => HttpResponse::NotFound().body("Not found"),
        Ok(Some(s)) => match serde_json::to_string(&s) {
            Ok(j) => HttpResponse::Ok().body(j),
            Err(_) => HttpResponse::InternalServerError().body("Can't create JSON output")
        }
        Err(e) => HttpResponse::BadRequest().body(e.to_string())
    }
}

#[delete("/users/{user_id}")]
async fn user_delete(state: Data<AppState>, path: web::Path<i64>) -> impl Responder {
    let user_id = path.into_inner();
    match sql_users_delete_by_ids(state, user_id).await {
        Ok(0) => HttpResponse::NotFound().body("Not found"),
        Ok(_) => HttpResponse::Ok().body("Success"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

#[post("/users/")]
async fn user_post(state: Data<AppState>, user: web::Json<UserCreate>) -> impl Responder {
    let user: UserCreate = user.into_inner();
    match sql_user_post(state, user).await {
        Ok(id) => HttpResponse::Ok().body(id.to_string()),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

// #[put("/users/{user_id}")]
// async fn user_put(state: Data<AppState>, path: web::Path<i64>, user: web::Json<UserCreate>) -> impl Responder {
//     let user: UserCreate = user.into_inner();
//     let user.user_id = path.into_inner();
//     match sql_user_put(state, user_id).await {
//         Ok(_) => HttpResponse::Ok().body("Success"),
//         Err(e) => HttpResponse::InternalServerError().body(e.to_string())
//     }
// }
