use std::error::Error;
use actix_web::{
    get, post, put, delete,
    web::{Data, self},
    HttpResponse, Responder,
};
use bcrypt::{hash, DEFAULT_COST, verify};
use sqlx::FromRow;
use chrono::NaiveDate;
use serde::{Serialize, Deserialize};
use serde_json;

use super::AppState;


const ANSWER_OK: &str = "Success";
const ANSWER_NOT_FOUND: &str = "Not Found";
const ANSWER_ID_ERROR: &str = "Identification Error";
const ANSWER_AUTH_ERROR: &str = "Authentication Error";

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

#[derive(Deserialize)]
pub struct UserLoginByEmail {
    email: String,
    passwd: String,
}

#[derive(Deserialize, FromRow, Debug)]
pub struct UserPwhash {
    user_id: i64,
    pwhash: String,
}

#[derive(Deserialize, FromRow, Debug)]
pub struct UserCreate {
    first_name: Option<String>,
    second_name: Option<String>,
    email: String,
    passwd: String,
    gender: Option<String>,
    birthdate: Option<NaiveDate>,
    biography: Option<String>,
    city: Option<String>,
}

#[derive(Deserialize, FromRow, Debug)]
pub struct UserUpdate {
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
pub struct UserCreateResult {
    user_id: i64,
}

#[derive(Deserialize, FromRow, Debug)]
pub struct UserQuickSearch {
    text: String,
}


pub async fn sql_login_by_email(state: Data<AppState>, user: UserLoginByEmail) -> Result<i64, Box<dyn Error>> {
    let sql = "SELECT user_id, pwhash FROM users WHERE email = ($1)";
    let u = sqlx::query_as::<_, UserPwhash> (sql)
    .bind(user.email)
    .fetch_optional(&state.db)
    .await?;

    match u {
        //Some(h) => match password_auth::verify_password(user.passwd, &h.pwhash) {
        Some(h) => match verify(user.passwd, &h.pwhash) {
            Ok(true) => Ok(h.user_id),
            _ => Err(ANSWER_AUTH_ERROR)?
        },
        _ => {
            //Just to make responces equal by text and by time
            //hash(user.passwd, DEFAULT_COST)?;
            Err(ANSWER_ID_ERROR)?
        }
    }
}

pub async fn sql_users_get_all(state: Data<AppState>) -> Result<Vec<User>, Box<dyn Error>> {
    let sql = "SELECT * FROM users";
    let us = sqlx::query_as::<_, User> (sql)
    .fetch_all(&state.db)
    .await?;

    Ok(us)
}

pub async fn sql_users_get_by_id(state: Data<AppState>, user_id: i64) -> Result<Option<User>, Box<dyn Error>> {
    let sql = "SELECT * FROM users WHERE user_id = ($1)";
    let u = sqlx::query_as::<_, User> (sql)
    .bind(user_id)
    .fetch_optional(&state.db)
    .await?;

    Ok(u)
}

pub async fn sql_users_delete_by_id(state: Data<AppState>, user_id: i64) -> Result<u64, Box<dyn Error>> {
    let sql = "DELETE FROM users WHERE user_id = ($1)";
    let res = sqlx::query(sql)
    .bind(user_id)
    .execute(&state.db)
    .await?;

    Ok(res.rows_affected())
}

pub async fn sql_user_post(state: Data<AppState>, user: UserCreate) -> Result<i64, Box<dyn Error>> {
    let pwhash = hash(user.passwd, DEFAULT_COST)?;
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
    .bind(&user.birthdate)
    .bind(&user.biography)
    .bind(&user.city)
    .fetch_one(&state.db)
    .await?;

    Ok(u.user_id)
}

pub async fn sql_quicksearch(state: Data<AppState>, text: String) -> Result<Vec<User>, Box<dyn Error>> {
    let sql = "SELECT * FROM users WHERE email LIKE ($1)";
    let us = sqlx::query_as::<_, User> (sql)
    .bind(format!("{}%",text))
    .fetch_all(&state.db)
    .await?;

    Ok(us)
}

pub async fn sql_user_put(state: Data<AppState>, user: UserUpdate) -> Result<u64, Box<dyn Error>> {
    let sql = "UPDATE users SET
    (first_name, second_name, email, gender, birthdate, biography, city)
    VALUES ($1, $2, $3, $4, $5, $6, $7) WHERE user_id = $8";
    let res = sqlx::query(sql)
    .bind(&user.first_name)
    .bind(&user.second_name)
    .bind(&user.email)
    .bind(&user.gender)
    .bind(&user.birthdate)
    .bind(&user.biography)
    .bind(&user.city)
    .bind(&user.user_id)
    .execute(&state.db)
    .await?;

    Ok(res.rows_affected())
}

#[post("/login")]
async fn login(state: Data<AppState>, user: web::Json<UserLoginByEmail>) -> impl Responder {
    let user = user.into_inner();
    match sql_login_by_email(state, user).await {
        Ok(id) => HttpResponse::Ok().body(id.to_string()),
        Err(e) => HttpResponse::Forbidden().body(e.to_string())
    }
}

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
    match sql_users_get_by_id(state, user_id).await {
        Ok(None) => HttpResponse::NotFound().body(ANSWER_NOT_FOUND),
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
    match sql_users_delete_by_id(state, user_id).await {
        Ok(0) => HttpResponse::NotFound().body(ANSWER_NOT_FOUND),
        Ok(_) => HttpResponse::Ok().body(ANSWER_OK),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

#[post("/users")]
async fn user_post(state: Data<AppState>, user: web::Json<UserCreate>) -> impl Responder {
    let user = user.into_inner();
    match sql_user_post(state, user).await {
        Ok(id) => HttpResponse::Ok().body(id.to_string()),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

#[post("/quicksearch")]
async fn quicksearch(state: Data<AppState>, j: web::Json<UserQuickSearch>) -> impl Responder {
    let s = j.into_inner();
    match sql_quicksearch(state, s.text).await {
        Ok(s) if s.len() == 0 => HttpResponse::NotFound().body(ANSWER_NOT_FOUND),
        Ok(s) => match serde_json::to_string(&s) {
            Ok(s) => HttpResponse::Ok().body(s),            
            Err(e) => HttpResponse::InternalServerError().body(e.to_string())
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

#[put("/users/{user_id}")]
async fn user_put(state: Data<AppState>, path: web::Path<i64>, user: web::Json<UserUpdate>) -> impl Responder {
    let mut user = user.into_inner();
    user.user_id = path.into_inner();
    match sql_user_put(state, user).await {
        Ok(0) => HttpResponse::NotFound().body(ANSWER_NOT_FOUND),
        Ok(_) => HttpResponse::Ok().body(ANSWER_OK),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}
