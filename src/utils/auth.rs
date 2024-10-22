use chrono::{Duration, Utc};
use jsonwebtoken::errors::Error;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub(crate) exp: i64,
    pub(crate) iat: i64,
    pub(crate) id: i32,
    pub(crate) email: String,
}

pub struct JSONWebToken {
    pub(crate) secret: String,
}

#[derive(Serialize, Deserialize)]
pub struct Token {
    pub token: String,
    pub refresh_token: String,
}

impl JSONWebToken {
    pub fn decode(&self, jwt: String) -> Result<TokenData<Claims>, Error> {
        let token_data: Result<TokenData<Claims>, Error> = decode::<Claims>(
            &jwt,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::new(Algorithm::HS512),
        );

        token_data
    }

    pub fn encode(&self, id: i32, email: String) -> Token {
        let now = Utc::now();
        let expiry = Duration::hours(1);

        let mut claims = Claims {
            exp: (now + expiry).timestamp(),
            iat: now.timestamp(),
            id,
            email,
        };

        let header = Header::new(Algorithm::HS512);
        let encoding_key = EncodingKey::from_secret(self.secret.as_bytes());
        let token = encode(&header, &claims, &encoding_key).unwrap_or_else(|err| err.to_string());

        let refresh_expiry = Duration::days(7);
        claims.exp = (now + refresh_expiry).timestamp();
        let refresh_token = encode(&header, &claims, &encoding_key).unwrap_or_else(|err| err.to_string());

        Token { token, refresh_token }
    }
}
