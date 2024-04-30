use anyhow::Result;
use fancy_duration::ParseFancyDuration;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    aud: String,
    exp: u64,
}

pub const SECRET_KEY: &[u8; 6] = b"secret";

pub fn process_jwt_sign(sub: String, aud: String, exp: String) -> Result<String> {
    let exp = Duration::parse_fancy_duration(exp)?;
    let exp = jsonwebtoken::get_current_timestamp() + exp.as_secs();

    let my_claims = Claims { sub, aud, exp };

    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(SECRET_KEY),
    )?;

    Ok(token)
}

pub fn process_jwt_verify(token: String) -> Result<bool> {
    let token_data = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(SECRET_KEY),
        &Validation::default(),
    )?;
    println!("{:?}", token_data.claims);
    println!("{:?}", token_data.header);
    Ok(true)
}
