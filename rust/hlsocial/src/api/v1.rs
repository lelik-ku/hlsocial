
use sqlx::FromRow;
use chrono::NaiveDate;
use serde::{Serialize, Deserialize};

pub mod token;
pub mod login;
pub mod user;
pub mod middleware;

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
