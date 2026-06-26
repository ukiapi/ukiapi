use crate::dependencies::Dependency;
use crate::HTTPException;
use axum::http::{header::AUTHORIZATION, request::Parts, StatusCode};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{de::DeserializeOwned, Serialize};
use std::marker::PhantomData;

/// Helper to extract Bearer token from Authorization header.
pub struct HTTPBearer;

impl HTTPBearer {
    /// Extract the token from the request parts.
    pub fn extract(parts: &Parts) -> Result<String, HTTPException> {
        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .ok_or_else(|| {
                HTTPException::new(StatusCode::UNAUTHORIZED, "Missing Authorization header")
            })?;

        if !auth_header.to_lowercase().starts_with("bearer ") {
            return Err(HTTPException::new(
                StatusCode::UNAUTHORIZED,
                "Invalid Authorization header format. Expected 'Bearer <token>'",
            ));
        }

        Ok(auth_header[7..].trim().to_string())
    }
}

/// Encode a JWT token with the given claims and secret.
pub fn encode_jwt<T: Serialize>(
    claims: &T,
    secret: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}

/// Decode and validate a JWT token with the given secret.
pub fn decode_jwt<T: DeserializeOwned>(
    token: &str,
    secret: &str,
) -> Result<T, jsonwebtoken::errors::Error> {
    let validation = Validation::default();
    decode::<T>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    )
    .map(|data| data.claims)
}

/// A dependency that extracts and validates a JWT token.
///
/// It expects the secret to be in the `JWT_SECRET` environment variable,
/// falling back to "secret" if not set.
pub struct JWTAuth<T, S = ()> {
    _marker: PhantomData<(T, S)>,
}

impl<T, S> Dependency<S> for JWTAuth<T, S>
where
    T: DeserializeOwned + Clone + Send + Sync + 'static,
    S: Send + Sync + 'static,
{
    type Output = T;

    async fn resolve(parts: &mut Parts, _state: &S) -> Result<Self::Output, HTTPException> {
        let token = HTTPBearer::extract(parts)?;
        let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());

        decode_jwt(&token, &secret).map_err(|e| {
            HTTPException::new(StatusCode::UNAUTHORIZED, format!("Invalid token: {}", e))
        })
    }
}

/// Helper for OAuth2 Password Flow.
pub struct OAuth2PasswordBearer {
    pub token_url: String,
}

impl OAuth2PasswordBearer {
    pub fn new(token_url: impl Into<String>) -> Self {
        Self {
            token_url: token_url.into(),
        }
    }
}

impl<S> Dependency<S> for OAuth2PasswordBearer
where
    S: Send + Sync + 'static,
{
    type Output = String;

    async fn resolve(parts: &mut Parts, _state: &S) -> Result<Self::Output, HTTPException> {
        HTTPBearer::extract(parts)
    }
}
