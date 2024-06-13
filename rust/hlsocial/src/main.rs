use sqlx::postgres::PgPoolOptions;
// use scrypt::{
//     password_hash::{
//         rand_core::OsRng,
//         PasswordHash, PasswordHasher, SaltString
//     },
//     Scrypt
// };

mod config;
mod api;
mod error;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize config from environment variables
    let config = config::config();

    // PSQL Connection
    let pgurl = format!("postgres://{}:{}@{}:{}/{}", config.db_user, config.db_pass, config.db_host, config.db_port, config.db_name);
    let pool = PgPoolOptions::new().max_connections(99).connect(&pgurl).await;
    println!("INFO: Connecting to PostgreSQL server: {}@{}:{}...", config.db_user, config.db_host, config.db_port);
    let pool = match pool {
        Ok(p) => p,
        Err(e) => panic!("ERROR: Can't connect to PostgreSQL server: {}@{}:{} due to: {:?}", config.db_user, config.db_host, config.db_port, e)
    };
    println!("INFO: Applying migrations...");
    sqlx::migrate!("./migrations").run(&pool).await.unwrap_or_else(
        |err| {
            panic!("ERROR: Can't apply migrations to PostgreSQL server: {}@{}:{} due to: {:?}",
            config.db_user,
            config.db_host,
            config.db_port,
            err)
        }
    );

    // API Server
    api::start_server(config, pool).await
}
