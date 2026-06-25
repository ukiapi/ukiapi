use axum::{
    extract::{FromRequest, FromRequestParts},
    http::request::Parts,
    Json,
};
use serde_json::{json, Value};
use validator::Validate;

/// An extractor for query parameters that performs validation.
pub struct Query<T: Validate>(pub T);

impl<T, S> FromRequestParts<S> for Query<T>
where
    T: serde::de::DeserializeOwned + Validate + Send + Sync + 'static,
    S: Send + Sync + 'static,
{
    type Rejection = (axum::http::StatusCode, Json<Value>);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let axum::extract::Query(query) =
            axum::extract::Query::<T>::from_request_parts(parts, state)
                .await
                .map_err(|e| {
                    (
                        axum::http::StatusCode::UNPROCESSABLE_ENTITY,
                        Json(json!({
                            "detail": format!("Invalid query parameters: {}", e),
                            "errors": null
                        })),
                    )
                })?;

        query.validate().map_err(|e| {
            (
                axum::http::StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({
                    "detail": format!("Validation failed: {}", e),
                })),
            )
        })?;

        Ok(Query(query))
    }
}

/// An extractor for JSON bodies that performs validation.
pub struct ValidatedJson<T: Validate>(pub T);

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: serde::de::DeserializeOwned + Validate + Send + Sync + 'static,
    S: Send + Sync + 'static,
{
    type Rejection = (axum::http::StatusCode, Json<Value>);

    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(body) = Json::<T>::from_request(req, state).await.map_err(|e| {
            (
                axum::http::StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({
                    "detail": format!("Invalid JSON: {}", e),
                })),
            )
        })?;

        body.validate().map_err(|e| {
            (
                axum::http::StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({
                    "detail": format!("Validation failed: {}", e),
                })),
            )
        })?;

        Ok(ValidatedJson(body))
    }
}
