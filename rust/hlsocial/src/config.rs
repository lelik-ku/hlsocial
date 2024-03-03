use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "DB_HOST")]
    pub db_host: String,

    #[envconfig(from = "DB_PORT", default = "5432")]
    pub db_port: u16,

    #[envconfig(from = "DB_USER")]
    pub db_user: String,

    #[envconfig(from = "DB_PASS")]
    pub db_pass: String,

    #[envconfig(from = "DB_NAME")]
    pub db_name: String,

    #[envconfig(from = "HTTP_HOST", default = "127.0.0.1")]
    pub http_host: String,

    #[envconfig(from = "HTTP_PORT", default = "8080")]
    pub http_port: u16,
}

pub fn config() -> Config {
    return Config::init_from_env().unwrap();
}


