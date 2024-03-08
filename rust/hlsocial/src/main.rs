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


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize config from environment variables
    let config = config::config();

    // PSQL Connection
    let pgurl = format!("postgres://{}:{}@{}:{}/{}", config.db_user, config.db_pass, config.db_host, config.db_port, config.db_name);
    let pool = PgPoolOptions::new().max_connections(5).connect(&pgurl).await;
    println!("INFO: Connecting to PostgreSQL server: {}@{}:{}...", config.db_user, config.db_host, config.db_port);
    let pool = match pool {
        Ok(p) => p,
        Err(e) => panic!("ERROR: Can't connect to PostgreSQL server: {}@{}:{} due to: {:?}", config.db_user, config.db_host, config.db_port, e)
    };
    println!("INFO: Applying migrations...");
    let _ = match sqlx::migrate!("./migrations").run(&pool).await {
        Ok(m) => m,
        Err(e) => panic!("ERROR: Can't apply migrations to PostgreSQL server: {}@{}:{} due to: {:?}", config.db_user, config.db_host, config.db_port, e)
    };

    // Debug
    // let r = api::users_read(Data::new(AppState {db: pool.clone()})).await;
    // println!("{:?}", r);

    // API Server
    api::start_server(config, pool).await
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
