use {
    crate::{
        enums::errors::*,
        features::auth::model::Claims,
    },
    jsonwebtoken::{
        decode,
        encode,
        DecodingKey,
        EncodingKey,
        Header,
        Validation,
    },
    std::env,
    uuid::Uuid,
};

#[warn(unused_variables)]
pub fn encode_jwt(user_id: Uuid, user_email: String) -> Result<String> {
    let claims = Claims {
        sub: user_email,
        exp: todo!(),
        user_id,
    };

    let secret = env::var("JWT_SECRET").map_err(|_| Error::EnvVarNotFound("JWT_SECRET".into()))?;

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(Error::EncodeJwtFailed)
}

pub fn decode_jwt(token: String) -> Result<Claims> {
    let secret = env::var("JWT_SECRET").map_err(|_| Error::EnvVarNotFound("JWT_SECRET".into()))?;

    let claims = decode(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(Error::DecodeJwtFailed)?
    .claims;

    Ok(claims)
}
