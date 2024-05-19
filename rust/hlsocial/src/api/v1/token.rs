
use chrono::Utc;
use jsonwebtoken::{encode, decode, Algorithm, EncodingKey, Header, Validation, DecodingKey, errors::Error};
use serde::{Deserialize, Serialize};

// use crate::error::Error;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Role {
    Admin,
    User,
}

#[derive(Serialize, Deserialize, Debug)]
struct Claims {
    sub: String,
    role: Role,
    exp: usize,
}

pub fn create_jwt(user_id: &str, role: Role, jwt_secret: &str) -> Result<String, Error> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::try_minutes(10).unwrap())
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id.to_owned(),
        role: role,
        exp: expiration as usize,
    };
    let header = Header::new(Algorithm::HS512);
    encode(
        &header,
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref())
    )
//    .map_err(|_| Error::JWTTokenCreationError)
}

pub fn validate_jwt(jwt: &str, jwt_secret: &str) -> Result<Role, Error> {
    decode::<Claims>(
        jwt,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &Validation::new(Algorithm::HS512))
    .map(|t| t.claims.role)
//    .map_err(|_| Error::JWTTokenError)
}
