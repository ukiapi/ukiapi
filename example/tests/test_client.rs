use example::routes::*;
use example::AppState;
use rustapi::TestClient;
use std::sync::{Arc, Mutex};

#[tokio::test]
async fn test_hello_endpoint() {
    let state = AppState {
        items: Arc::new(Mutex::new(Vec::new())),
    };

    let api = rustapi::routes![AppState, hello_route().with_state::<AppState>()];

    let client = TestClient::new(api, state);

    let response = client.get("/hello").send().await;
    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_items_list() {
    let state = AppState {
        items: Arc::new(Mutex::new(Vec::new())),
    };

    let api = rustapi::routes![AppState, items_router()];

    let client = TestClient::new(api, state);

    let response = client.get("/items").send().await;
    assert_eq!(response.status(), 200);
}
