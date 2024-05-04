use anyhow::Result;

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    aud: String,
    exp: usize,
}

impl Claims {
    fn new(sub: String, aud: String, exp: String) -> Claims {
        Claims {
            sub,
            aud,
            exp: get_exp(exp),
        }
    }
}

fn get_exp(exp: String) -> usize {
    let mut time: chrono::prelude::DateTime<Utc> = Utc::now();
    let mut tmp_exp = exp.as_str();

    if exp.contains('d') {
        let times: Vec<&str> = tmp_exp.split('d').collect();
        let days = match times[0].parse::<i64>() {
            Ok(number) => number,
            Err(_) => panic!("Invalid exp day number: {}", times[0]),
        };
        time += Duration::days(days);
        tmp_exp = times[1];
    }

    if exp.contains('h') {
        let times: Vec<&str> = tmp_exp.split('h').collect();
        let hours = match times[0].parse::<i64>() {
            Ok(number) => number,
            Err(_) => panic!("Invalid exp hour number: {}", times[0]),
        };
        time += Duration::hours(hours);
        tmp_exp = times[1];
    }
    if exp.contains('m') {
        let times: Vec<&str> = tmp_exp.split('m').collect();
        let mintes = match times[0].parse::<i64>() {
            Ok(number) => number,
            Err(_) => panic!("Invalid exp hour number: {}", times[0]),
        };
        time += Duration::hours(mintes);
    }
    if exp.contains('s') {
        let times: Vec<&str> = tmp_exp.split('s').collect();
        let seconds = match times[0].parse::<i64>() {
            Ok(number) => number,
            Err(_) => panic!("Invalid exp second number: {}", times[0]),
        };
        time += Duration::seconds(seconds);
    }
    time.timestamp() as usize
}

pub fn process_jwt_sign(sub: String, aud: String, exp: String) -> Result<String> {
    let header = Header::default();
    let claims = Claims::new(sub, aud, exp);
    let encoding_key = EncodingKey::from_secret("secret".as_ref());
    Ok(encode(&header, &claims, &encoding_key)?)
}

pub fn process_jwt_verify(token: &str) -> Result<TokenData<Claims>> {
    let mut validation = Validation::default();
    validation.set_audience(&["device1", "device2", "device3"]);
    let token = decode::<Claims>(
        token,
        &DecodingKey::from_secret("secret".as_ref()),
        &validation,
    )?;
    Ok(token)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process_jwt_sign() -> Result<()> {
        let token = process_jwt_sign("sub1".to_owned(), "device1".to_owned(), "1".to_owned())?;
        let claims = process_jwt_verify(&token)?.claims;
        assert_eq!(claims.sub, "sub1");
        assert_eq!(claims.aud, "device1");
        let token = process_jwt_sign("sub1".to_owned(), "devicex".to_owned(), "1".to_owned())?;
        let token_data = process_jwt_verify(&token);
        assert!(token_data.is_err());
        Ok(())
    }
}
