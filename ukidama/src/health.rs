use crate::response::Json;
use serde_json::{json, Value};
use std::sync::OnceLock;
use std::time::Instant;

static STARTED: OnceLock<Instant> = OnceLock::new();
static HEALTH_TITLE: OnceLock<String> = OnceLock::new();
static HEALTH_VERSION: OnceLock<String> = OnceLock::new();

pub(crate) fn init(title: &str, version: &str) {
    STARTED.get_or_init(Instant::now);
    HEALTH_TITLE.get_or_init(|| title.to_string());
    HEALTH_VERSION.get_or_init(|| version.to_string());
}

pub(crate) async fn handler() -> Json<Value> {
    let uptime = STARTED.get().map(|i| i.elapsed().as_secs()).unwrap_or(0);

    Json(json!({
        "status": "ok",
        "title": HEALTH_TITLE.get().map(|s| s.as_str()).unwrap_or("Ukidama"),
        "version": HEALTH_VERSION.get().map(|s| s.as_str()).unwrap_or("0.0.0"),
        "uptime_seconds": uptime,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_handler_returns_ok() {
        let Json(response) = handler().await;
        assert_eq!(response["status"], "ok");
    }

    #[tokio::test]
    async fn test_health_handler_has_title() {
        let Json(response) = handler().await;
        assert!(response["title"].is_string());
    }

    #[tokio::test]
    async fn test_health_handler_has_version() {
        let Json(response) = handler().await;
        assert!(response["version"].is_string());
    }

    #[tokio::test]
    async fn test_health_handler_has_uptime() {
        let Json(response) = handler().await;
        assert!(response["uptime_seconds"].is_number());
    }

    #[tokio::test]
    async fn test_health_handler_json_structure() {
        let Json(response) = handler().await;
        assert!(response.get("status").is_some());
        assert!(response.get("title").is_some());
        assert!(response.get("version").is_some());
        assert!(response.get("uptime_seconds").is_some());
    }
}
