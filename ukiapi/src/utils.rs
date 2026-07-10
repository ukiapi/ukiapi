use serde::Serialize;
use serde_json::Value;

/// Converts a serializable object to a JSON-compatible `serde_json::Value`.
/// This is similar to FastAPI's `jsonable_encoder`.
pub fn jsonable_encoder<T: Serialize>(data: T) -> Value {
    serde_json::to_value(data).unwrap_or(Value::Null)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[derive(Serialize)]
    struct TestStruct {
        name: String,
        value: i32,
    }

    #[test]
    fn test_jsonable_encoder_struct() {
        let data = TestStruct {
            name: "test".to_string(),
            value: 42,
        };
        let result = jsonable_encoder(data);
        assert_eq!(result, json!({"name": "test", "value": 42}));
    }

    #[test]
    fn test_jsonable_encoder_string() {
        let result = jsonable_encoder("hello");
        assert_eq!(result, json!("hello"));
    }

    #[test]
    fn test_jsonable_encoder_number() {
        let result = jsonable_encoder(42);
        assert_eq!(result, json!(42));
    }

    #[test]
    fn test_jsonable_encoder_vec() {
        let data = vec![1, 2, 3];
        let result = jsonable_encoder(data);
        assert_eq!(result, json!([1, 2, 3]));
    }

    #[test]
    fn test_jsonable_encoder_nested_struct() {
        #[derive(Serialize)]
        struct Inner {
            x: i32,
        }
        #[derive(Serialize)]
        struct Outer {
            inner: Inner,
            name: String,
        }
        let data = Outer {
            inner: Inner { x: 10 },
            name: "test".to_string(),
        };
        let result = jsonable_encoder(data);
        assert_eq!(result, json!({"inner": {"x": 10}, "name": "test"}));
    }

    #[test]
    fn test_jsonable_encoder_null() {
        let result = jsonable_encoder(None::<String>);
        assert_eq!(result, Value::Null);
    }

    #[test]
    fn test_jsonable_encoder_bool() {
        let result = jsonable_encoder(true);
        assert_eq!(result, json!(true));
    }

    #[test]
    fn test_jsonable_encoder_float() {
        let result = jsonable_encoder(1.5);
        assert_eq!(result, json!(1.5));
    }
}
