use crate::extract::query::Query as AxumQuery;
use crate::extract::Request;
pub use crate::extract::{FromRequest, FromRequestParts};
use crate::http::request::Parts;
use crate::http::StatusCode;
use crate::response::Json;
use serde_json::{json, Value};
use validator::Validate;

/// An extractor for query parameters that performs validation.
pub struct Query<T: Validate>(pub T);

impl<T, S> FromRequestParts<S> for Query<T>
where
    T: serde::de::DeserializeOwned + Validate + Send + Sync + 'static,
    S: Send + Sync + 'static,
{
    type Rejection = (StatusCode, Json<Value>);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let AxumQuery(query) = AxumQuery::<T>::from_request_parts(parts, state)
            .await
            .map_err(|e| {
                (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    Json(json!({
                        "detail": format!("Invalid query parameters: {}", e),
                        "errors": null
                    })),
                )
            })?;

        query.validate().map_err(|e| {
            (
                StatusCode::UNPROCESSABLE_ENTITY,
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
    type Rejection = (StatusCode, Json<Value>);

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(body) = Json::<T>::from_request(req, state).await.map_err(|e| {
            (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({
                    "detail": format!("Invalid JSON: {}", e),
                })),
            )
        })?;

        body.validate().map_err(|e| {
            (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({
                    "detail": format!("Validation failed: {}", e),
                })),
            )
        })?;

        Ok(ValidatedJson(body))
    }
}
