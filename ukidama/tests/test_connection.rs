use ukidama::{get, routes, HTTPConnection, TestClient};

#[get("/client-addr")]
async fn get_client_addr(conn: HTTPConnection) -> String {
    conn.client_addr.to_string()
}

#[tokio::test]
async fn test_client_addr() {
    let api = routes![(), get_client_addr_route()];
    let client = TestClient::new(api, ());

    // Test default fallback
    let resp = client.get("/client-addr").send().await;
    let body = axum::body::to_bytes(resp.into_body(), 1024).await.unwrap();
    assert_eq!(body, "0.0.0.0:0");
}
