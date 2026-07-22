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
    pub fn extract(parts: &Parts) -> Result<&str, HTTPException> {
        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .ok_or_else(|| {
                HTTPException::new(StatusCode::UNAUTHORIZED, "Missing Authorization header")
            })?;

        // ⚡ Bolt: Use .as_bytes() and .eq_ignore_ascii_case() to avoid heap allocation in this hot path
        if auth_header.len() < 7 || !auth_header.as_bytes()[..7].eq_ignore_ascii_case(b"bearer ") {
            return Err(HTTPException::new(
                StatusCode::UNAUTHORIZED,
                "Invalid Authorization header format. Expected 'Bearer <token>'",
            ));
        }

        // ⚡ Bolt: Return &str instead of String to avoid heap allocation
        Ok(auth_header[7..].trim())
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
        let secret = std::env::var("JWT_SECRET").map_err(|_| {
            HTTPException::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Server misconfiguration",
            )
        })?;

        decode_jwt(token, &secret).map_err(|e| {
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
        HTTPBearer::extract(parts).map(|s| s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::header::{HeaderMap, HeaderValue};
    use axum::http::request::Parts;

    #[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
    struct TestClaims {
        sub: String,
        exp: usize,
    }

    fn create_test_parts(headers: HeaderMap) -> Parts {
        let mut request = axum::http::Request::builder().body(()).unwrap();
        *request.headers_mut() = headers;
        let (parts, _) = request.into_parts();
        parts
    }

    #[test]
    fn test_http_bearer_extract_valid() {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_static("Bearer test_token_123"),
        );
        let parts = create_test_parts(headers);
        let token = HTTPBearer::extract(&parts).unwrap();
        assert_eq!(token, "test_token_123");
    }

    #[test]
    fn test_http_bearer_extract_missing_header() {
        let parts = create_test_parts(HeaderMap::new());
        let result = HTTPBearer::extract(&parts);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().status_code, StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn test_http_bearer_extract_wrong_format() {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_static("Basic dXNlcjpwYXNz"),
        );
        let parts = create_test_parts(headers);
        let result = HTTPBearer::extract(&parts);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().status_code, StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn test_http_bearer_extract_with_extra_spaces() {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_static("Bearer  token_with_spaces"),
        );
        let parts = create_test_parts(headers);
        let token = HTTPBearer::extract(&parts).unwrap();
        assert_eq!(token, "token_with_spaces");
    }

    #[test]
    fn test_encode_decode_jwt_roundtrip() {
        let claims = TestClaims {
            sub: "user123".to_string(),
            exp: 9999999999, // Far future
        };
        let secret = "test_secret";
        let token = encode_jwt(&claims, secret).unwrap();
        let decoded: TestClaims = decode_jwt(&token, secret).unwrap();
        assert_eq!(decoded.sub, "user123");
        assert_eq!(decoded.exp, 9999999999);
    }

    #[test]
    fn test_decode_jwt_wrong_secret() {
        let claims = TestClaims {
            sub: "user123".to_string(),
            exp: 9999999999, // Far future
        };
        let token = encode_jwt(&claims, "correct_secret").unwrap();
        let result = decode_jwt::<TestClaims>(&token, "wrong_secret");
        assert!(result.is_err());
    }

    #[test]
    fn test_encode_jwt_produces_valid_token() {
        let claims = TestClaims {
            sub: "test_user".to_string(),
            exp: 9999999999, // Far future
        };
        let token = encode_jwt(&claims, "secret").unwrap();
        assert!(!token.is_empty());
        assert!(token.contains('.'));
    }

    #[test]
    fn test_oauth2_password_bearer_new() {
        let oauth = OAuth2PasswordBearer::new("/api/login");
        assert_eq!(oauth.token_url, "/api/login");
    }

    #[test]
    fn test_http_bearer_extract_empty_token() {
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, HeaderValue::from_static("Bearer "));
        let parts = create_test_parts(headers);
        let token = HTTPBearer::extract(&parts).unwrap();
        assert!(token.is_empty());
    }

    #[test]
    fn test_http_bearer_extract_case_insensitive() {
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, HeaderValue::from_static("bearer my_token"));
        let parts = create_test_parts(headers);
        let token = HTTPBearer::extract(&parts).unwrap();
        assert_eq!(token, "my_token");
    }
}
