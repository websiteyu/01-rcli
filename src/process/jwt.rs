use anyhow::Result;

use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    aud: String,
    exp: String,
}

impl Claims {
    fn new(sub: String, aud: String, exp: String) -> Claims {
        Claims { sub, aud, exp }
    }
}

pub fn process_jwt_sign(sub: String, aud: String, exp: String) -> Result<String> {
    let header = Header::default();
    let claims = Claims::new(sub, aud, exp);
    let encoding_key = EncodingKey::from_secret("secret".as_ref());
    Ok(encode(&header, &claims, &encoding_key)?)
}

pub fn process_jwt_verify(token: String) -> Result<()> {
    println!("token: {}", token);
    Ok(())
}
