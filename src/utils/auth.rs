use chrono::{Duration, Utc};
use jsonwebtoken::errors::Error;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: i64,
    iat: i64,
    id: i32,
    email: String,
}

pub struct JSONWebToken {
    pub(crate) secret: String,
}

impl JSONWebToken {
    pub fn decode(&self, jwt: &String) -> Result<TokenData<Claims>, Error> {
        let token_data: Result<TokenData<Claims>, Error> = decode::<Claims>(
            &jwt,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::new(Algorithm::HS512),
        );

        token_data
    }

    pub fn encode(&self, id: i32, email: String) -> String {
        let now = Utc::now();
        let expiry = Duration::days(1);

        let my_claims = Claims {
            exp: (now + expiry).timestamp(),
            iat: now.timestamp(),
            id,
            email,
        };

        let header = Header {
            kid: Some(self.secret.to_owned()),
            alg: Algorithm::HS512,
            ..Default::default()
        };

        let encoding_key = EncodingKey::from_secret(self.secret.as_bytes());

        let token = match encode(&header, &my_claims, &encoding_key) {
            Ok(_token) => _token,
            Err(_) => panic!(), // in practice, you would return the error
        };

        token
    }
}




