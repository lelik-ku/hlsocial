use actix_web::{App, HttpServer, web::Data};
use sqlx::{Postgres, Pool};

use crate::config;

pub(crate) mod v1;


pub struct AppState {
    db: Pool<Postgres>
}

pub async fn start_server(config: config::Config, pool: Pool<Postgres>) -> std::io::Result<()> {
    println!("INFO: Starting server at http://{}:{}", config.http_host, config.http_port);
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState {db: pool.clone()}))
            .service(v1::users_get)
            .service(v1::user_get)
            // .service(echo)
            // .route("/hey", web::get().to(manual_hello))
    })
    .bind(format!("{}:{}", config.http_host, config.http_port))?
    .run()
    .await
}