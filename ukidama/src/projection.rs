use crate::response::{AxumResponse, IntoResponse};
use axum::Json;
use serde::Serialize;
use std::collections::HashSet;

/// A wrapper for responses that allows dynamic field projection (include/exclude)
/// at runtime, similar to Pydantic in FastAPI.
pub struct Projected<T> {
    data: T,
    include: Option<HashSet<String>>,
    exclude: Option<HashSet<String>>,
}

impl<T> Projected<T> {
    /// Create a new `Projected` response.
    pub fn new(data: T) -> Self {
        Self {
            data,
            include: None,
            exclude: None,
        }
    }

    /// Specify fields to include in the serialized JSON.
    pub fn include<I, S>(mut self, fields: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let mut set = self.include.unwrap_or_default();
        for field in fields {
            set.insert(field.into());
        }
        self.include = Some(set);
        self
    }

    /// Specify fields to exclude from the serialized JSON.
    pub fn exclude<I, S>(mut self, fields: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let mut set = self.exclude.unwrap_or_default();
        for field in fields {
            set.insert(field.into());
        }
        self.exclude = Some(set);
        self
    }
}

fn filter_value(
    value: &mut serde_json::Value,
    include: &Option<HashSet<String>>,
    exclude: &Option<HashSet<String>>,
) {
    if let Some(obj) = value.as_object_mut() {
        if let Some(inc) = include {
            obj.retain(|k, _| inc.contains(k));
        }
        if let Some(exc) = exclude {
            obj.retain(|k, _| !exc.contains(k));
        }
        // Recursively filter nested objects
        for val in obj.values_mut() {
            filter_value(val, include, exclude);
        }
    } else if let Some(arr) = value.as_array_mut() {
        for item in arr.iter_mut() {
            filter_value(item, include, exclude);
        }
    }
}

impl<T> IntoResponse for Projected<T>
where
    T: Serialize + Send + Sync + 'static,
{
    fn into_response(self) -> AxumResponse {
        match serde_json::to_value(&self.data) {
            Ok(mut val) => {
                filter_value(&mut val, &self.include, &self.exclude);
                Json(val).into_response()
            }
            Err(e) => {
                use crate::http::StatusCode;
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Serialization error: {}", e),
                )
                    .into_response()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;
    use serde_json::json;

    #[derive(Serialize, Clone)]
    struct User {
        id: i32,
        name: String,
        email: String,
    }

    #[test]
    fn test_include_fields() {
        let user = User {
            id: 1,
            name: "Alice".into(),
            email: "alice@example.com".into(),
        };

        let projected = Projected::new(user).include(vec!["id", "name"]);
        let mut val = serde_json::to_value(&projected.data).unwrap();
        filter_value(&mut val, &projected.include, &projected.exclude);

        assert_eq!(val, json!({ "id": 1, "name": "Alice" }));
    }

    #[test]
    fn test_exclude_fields() {
        let user = User {
            id: 1,
            name: "Alice".into(),
            email: "alice@example.com".into(),
        };

        let projected = Projected::new(user).exclude(vec!["email"]);
        let mut val = serde_json::to_value(&projected.data).unwrap();
        filter_value(&mut val, &projected.include, &projected.exclude);

        assert_eq!(val, json!({ "id": 1, "name": "Alice" }));
    }

    #[test]
    fn test_include_and_exclude_fields() {
        let user = User {
            id: 1,
            name: "Alice".into(),
            email: "alice@example.com".into(),
        };

        let projected = Projected::new(user)
            .include(vec!["id", "name", "email"])
            .exclude(vec!["email"]);
        let mut val = serde_json::to_value(&projected.data).unwrap();
        filter_value(&mut val, &projected.include, &projected.exclude);

        assert_eq!(val, json!({ "id": 1, "name": "Alice" }));
    }

    #[test]
    fn test_array_projection() {
        let users = vec![
            User {
                id: 1,
                name: "Alice".into(),
                email: "alice@example.com".into(),
            },
            User {
                id: 2,
                name: "Bob".into(),
                email: "bob@example.com".into(),
            },
        ];

        let projected = Projected::new(users).include(vec!["name"]);
        let mut val = serde_json::to_value(&projected.data).unwrap();
        filter_value(&mut val, &projected.include, &projected.exclude);

        assert_eq!(
            val,
            json!([
                { "name": "Alice" },
                { "name": "Bob" }
            ])
        );
    }

    #[derive(Serialize, Clone)]
    struct NestedUser {
        id: i32,
        name: String,
        details: UserDetails,
    }

    #[derive(Serialize, Clone)]
    struct UserDetails {
        age: i32,
        email: String,
    }

    #[test]
    fn test_nested_projection() {
        let user = NestedUser {
            id: 1,
            name: "Alice".into(),
            details: UserDetails {
                age: 30,
                email: "alice@example.com".into(),
            },
        };

        let projected = Projected::new(user).exclude(vec!["email"]);
        let mut val = serde_json::to_value(&projected.data).unwrap();
        filter_value(&mut val, &projected.include, &projected.exclude);

        assert_eq!(
            val,
            json!({
                "id": 1,
                "name": "Alice",
                "details": {
                    "age": 30
                }
            })
        );
    }
}
