use example::models::{LoginRequest, TokenResponse, UserClaims};
use example::routes::*;
use example::AppState;
use std::sync::{Arc, Mutex};
use ukidama::test_client::ResponseExt;
use ukidama::TestClient;

#[tokio::test]
async fn test_auth_flow() {
    std::env::set_var("JWT_SECRET", "test_secret");
    let state = AppState {
        items: Arc::new(Mutex::new(Vec::new())),
    };

    let api = ukidama::routes![AppState, auth_router()];
    let client = TestClient::new(api, state);

    // 1. Try to access /me without token
    let response = client.get("/me").send().await;
    assert_eq!(response.status(), 401);

    // 2. Login
    let login_data = LoginRequest {
        username: "testuser".to_string(),
    };
    let response = client.post("/login", &login_data).send().await;
    assert_eq!(response.status(), 200);

    let token_resp: TokenResponse = response.json().await;
    assert_eq!(token_resp.token_type, "Bearer");

    // 3. Access /me with token
    let response = client
        .get("/me")
        .header(
            "Authorization",
            format!("Bearer {}", token_resp.access_token),
        )
        .send()
        .await;
    assert_eq!(response.status(), 200);

    let claims: UserClaims = response.json().await;
    assert_eq!(claims.sub, "testuser");
}

#[tokio::test]
async fn test_auth_invalid_token() {
    std::env::set_var("JWT_SECRET", "test_secret");
    let state = AppState {
        items: Arc::new(Mutex::new(Vec::new())),
    };

    let api = ukidama::routes![AppState, auth_router()];
    let client = TestClient::new(api, state);

    let response = client
        .get("/me")
        .header("Authorization", "Bearer invalid-token")
        .send()
        .await;
    assert_eq!(response.status(), 401);
}
