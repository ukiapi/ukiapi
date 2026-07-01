use serde::Serialize;
use serde_json::Value;

/// Converts a serializable object to a JSON-compatible `serde_json::Value`.
/// This is similar to FastAPI's `jsonable_encoder`.
pub fn jsonable_encoder<T: Serialize>(data: T) -> Value {
    serde_json::to_value(data).unwrap_or(Value::Null)
}
