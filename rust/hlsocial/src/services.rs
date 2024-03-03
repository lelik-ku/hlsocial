use actix_web::{
    get,
    web::Data,
    HttpResponse, Responder,
};
use serde::Serialize;
use sqlx::{self, prelude::FromRow};
use crate::AppState;


#[derive(Serialize, FromRow)]
pub struct User {
    user_id: i64,
    first_name: Option<String>,
    second_name: Option<String>,
    // pwhash: String,
    email: String,
    gender: Option<String>,
    birthdate: Option<String>,
    biography: Option<String>,
    city: Option<String>
}

#[get("/users")]
pub async fn get_users(state: Data<AppState>) -> impl Responder {
    let sql = "SELECT user_id, first_name, second_name, email, gender, birthdate, biography, city FROM users";
    let query = sqlx::query_as::<_, User>(sql);
    match query.fetch_all(&state.db).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => HttpResponse::NotFound().json(format!("ERROR: {:?}", e))
    }
}

// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }
