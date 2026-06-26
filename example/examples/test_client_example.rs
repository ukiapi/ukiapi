use rustapi::{get, routes, TestClient, Json};

#[get("/hello")]
async fn hello_handler() -> Json<serde_json::Value> {
    rustapi::json!({"message": "hello"}).into()
}

#[tokio::main]
async fn main() {
    let state = ();
    let api = routes![(), hello_handler_route()];

    let client = TestClient::new(api, state);

    let response = client.get("/hello").await;
    assert_eq!(response.status(), 200);

    println!("Test passed!");
}
