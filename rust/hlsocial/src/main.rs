use actix_web::{web::Data, App, HttpServer};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
// use scrypt::{
//     password_hash::{
//         rand_core::OsRng,
//         PasswordHash, PasswordHasher, SaltString
//     },
//     Scrypt
// };

mod config;
mod services;


pub struct AppState {
    db: Pool<Postgres>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize config from environment variables
    let config = config::config();

    // PSQL Connection
    let pgurl = format!("postgres://{}:{}@{}:{}/{}", config.db_user, config.db_pass, config.db_host, config.db_port, config.db_name);
    let pool = PgPoolOptions::new().max_connections(5).connect(&pgurl).await;
    let pool = match pool {
        Ok(p) => p,
        Err(e) => panic!("ERROR: Can't connect to PostgreSQL server: {}@{}:{} due to: {:?}", config.db_user, config.db_host, config.db_port, e)
    };
    let _ = match sqlx::migrate!("./migrations").run(&pool).await {
        Ok(m) => m,
        Err(e) => println!("WARNING: Can't apply migrations to PostgreSQL server: {}@{}:{} due to: {:?}", config.db_user, config.db_host, config.db_port, e)
    };

    // API Server
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState {db: pool.clone()}))
            .service(services::get_users)
            // .service(echo)
            // .route("/hey", web::get().to(manual_hello))
    })
    .bind(format!("{}:{}", config.http_host, config.http_port))?
    .run()
    .await
}

// fn main() {
//     let password = b"admin";
//     let salt = SaltString::generate(&mut OsRng);
//     println!("Salt: {}", &salt);
//     let password_hash = Scrypt.hash_password(password, &salt);
//     println!("Hash: {}", &password_hash);
//     // let parsed_hash = PasswordHash::new(&password_hash)?;
//     // println!("{}", &parsed_hash);
// }
