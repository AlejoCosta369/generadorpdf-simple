use argon2::password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use argon2::Argon2;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;
use cookie::Cookie;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

const COOKIE_NAME: &str = "session";
const TOKEN_TTL_SECS: i64 = 60 * 60 * 24 * 7; // 7 dias

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,
    pub exp: i64,
}

fn jwt_secret() -> String {
    std::env::var("JWT_SECRET").expect("JWT_SECRET must be set")
}

pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .expect("password hashing failed")
        .to_string()
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    let Ok(parsed_hash) = PasswordHash::new(hash) else {
        return false;
    };
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

pub fn create_token(user_id: i64) -> String {
    let claims = Claims {
        sub: user_id,
        exp: (chrono::Utc::now().timestamp()) + TOKEN_TTL_SECS,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret().as_bytes()),
    )
    .expect("failed to encode jwt")
}

pub fn session_cookie(token: String) -> Cookie<'static> {
    Cookie::build((COOKIE_NAME, token))
        .path("/")
        .http_only(true)
        .max_age(cookie::time::Duration::seconds(TOKEN_TTL_SECS))
        .build()
}

pub fn clear_cookie() -> Cookie<'static> {
    Cookie::build((COOKIE_NAME, ""))
        .path("/")
        .http_only(true)
        .max_age(cookie::time::Duration::ZERO)
        .build()
}

pub struct AuthUser {
    pub user_id: i64,
}

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let cookie_header = parts
            .headers
            .get(axum::http::header::COOKIE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        let token = cookie_header
            .split(';')
            .filter_map(|c| Cookie::parse(c.trim()).ok())
            .find(|c| c.name() == COOKIE_NAME)
            .map(|c| c.value().to_string())
            .ok_or((StatusCode::UNAUTHORIZED, "no autenticado"))?;

        let data = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(jwt_secret().as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| (StatusCode::UNAUTHORIZED, "sesion invalida"))?;

        Ok(AuthUser {
            user_id: data.claims.sub,
        })
    }
}
