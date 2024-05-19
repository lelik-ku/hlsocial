use thiserror::Error;


#[derive(Error, Debug)]
pub enum Error {
    #[error("wrong credentials")]
    WrongCredentialsError,
    // #[error("jwt token not valid")]
    // JWTTokenError,
    #[error("jwt token creation error")]
    JWTTokenCreationError,
    // #[error("no auth header")]
    // NoAuthHeaderError,
    // #[error("invalid auth header")]
    // InvalidAuthHeaderError,
    // #[error("not permitted")]
    // NoPermissionError,
    #[error("not found")]
    NotFoundError,
    #[error("unauthorized")]
    Unauthorized,
}
