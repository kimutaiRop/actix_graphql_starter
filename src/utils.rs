use chrono::{Duration, Utc};
use dotenvy::dotenv;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    id: String, // Optional. Audience
    exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
}

pub fn generate_jwt(id: &i32) -> String {
    dotenv().ok();
    let secret: String = env::var("SECRET_KEY").expect("JWT_SECRET must be set");
    let issued_at = (Utc::now() + Duration::seconds(60 * 30)).timestamp() as usize;
    let my_claims = Claims {
        exp: issued_at + 1800,
        id: id.to_string(),
    };
    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(secret.as_ref()),
    );
    token.unwrap()
}

pub fn get_user_id(token: &str) -> i32 {
    dotenv().ok();
    let secret = env::var("SECRET_KEY").expect("JWT_SECRET must be set");

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    );

    match token_data {
        Ok(data) => data.claims.id.parse::<i32>().unwrap(),
        Err(_e) => 0,
    }
}

// GENERATE TOKEN from email
#[derive(Debug, Serialize, Deserialize)]
struct VerificationToken {
    email: String,
    exp: usize,
}

pub fn verify_token(email: &str) -> String {
    dotenv().ok();
    let secret: String = env::var("SECRET_KEY").expect("JWT_SECRET must be set");

    let issued_at = (Utc::now() + Duration::seconds(60 * 30)).timestamp() as usize;
    // expores after 1 week
    let my_claims = VerificationToken {
        exp: issued_at + 604800,
        email: email.to_string(),
    };

    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(secret.as_ref()),
    );
    token.unwrap().to_string()
}

// extract email from token
pub fn extract_email(token: &str) -> String {
    dotenv().ok();
    let secret = env::var("SECRET_KEY").expect("JWT_SECRET must be set");

    let token_data = decode::<VerificationToken>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    );

    match token_data {
        Ok(data) => data.claims.email.to_string(),
        Err(_e) => "".to_string(),
    }
}

