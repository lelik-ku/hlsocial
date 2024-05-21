use actix_web::{App, HttpServer, web::{Data, self}, middleware::NormalizePath};
use sqlx::{Postgres, Pool};

use crate::{config, api::v1::middleware::Auth};

mod v1;


pub struct AppState {
    db: Pool<Postgres>,
    jwt_secret: String,
}

pub const ANSWER_OK: &str = "Success";
pub const TOKEN: &str = "token";

pub async fn start_server(config: config::Config, pool: Pool<Postgres>) -> std::io::Result<()> {
    println!("INFO: Starting server at http://{}:{}", config.http_host, config.http_port);
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState {db: pool.clone(), jwt_secret: config.jwt_secret.clone()}))
            .wrap(NormalizePath::default())
            .wrap(Auth)
            .route("/v1/login", web::post().to(v1::login::login))
            .route("/v1/logout", web::post().to(v1::login::logout))
            .route("/v1/users", web::get().to(v1::user::users_get_all))
            .route("/v1/register", web::post().to(v1::user::user_post))
            .route("/v1/users/{user_id}", web::get().to(v1::user::user_get))
            .route("/v1/users/{user_id}", web::put().to(v1::user::user_put))
            .route("/v1/users/{user_id}", web::delete().to(v1::user::user_delete))
            .route("/v1/search/{text}", web::get().to(v1::user::search))
    })
    .bind(format!("{}:{}", config.http_host, config.http_port))?
    .run()
    .await
}
