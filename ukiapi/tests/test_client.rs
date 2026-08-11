use ukiapi::{get, post, routes, TestClient};

#[get("/test")]
async fn test_get() -> &'static str {
    "get"
}

#[post("/test")]
async fn test_post() -> &'static str {
    "post"
}

#[tokio::test]
async fn test_client_methods() {
    let api = routes![(), test_get_route(), test_post_route()];
    let client = TestClient::new(api, ());

    let resp = client.get("/test").send().await;
    assert_eq!(resp.status(), 200);

    let resp = client.post("/test", &"").send().await;
    assert_eq!(resp.status(), 200);

    // Test that the newly added methods can be called correctly
    // They should return 404 since there are no routes defined for them using the macro
    let resp = client.put("/test", &"").send().await;
    assert_eq!(resp.status(), 405); // axum returns 405 Method Not Allowed

    let resp = client.patch("/test", &"").send().await;
    assert_eq!(resp.status(), 405); // axum returns 405 Method Not Allowed

    let resp = client.delete("/test").send().await;
    assert_eq!(resp.status(), 405); // axum returns 405 Method Not Allowed
}

#[tokio::test]
async fn test_client_empty_body_serialization() {
    let api = ukiapi::routes![(), ukiapi::routing::Route::post("/test", |_: ukiapi::extract::Request| async { "OK" })];
    let client = ukiapi::TestClient::new(api, ());

    // Test that passing a reference to an empty string literal successfully builds a request
    // and does not panic during JSON serialization in TestClient methods.
    let req = client.post("/test", &"");
    let res = req.send().await;
    assert_eq!(res.status(), axum::http::StatusCode::OK);
}
