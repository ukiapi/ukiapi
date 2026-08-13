use serde::{Deserialize, Serialize};
use ukiapi::{get, post, routes, JsonSchema, Query, TestClient, ValidatedJson};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate, JsonSchema)]
struct TestQuery {
    #[validate(range(min = 1, max = 100))]
    page: u32,
    #[validate(length(min = 1, max = 50))]
    name: String,
}

#[derive(Debug, Deserialize, Serialize, Validate, JsonSchema)]
struct TestBody {
    #[validate(length(min = 1, max = 100))]
    title: String,
    #[validate(range(min = 0))]
    count: i32,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
struct QueryResponse {
    page: u32,
    name: String,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
struct BodyResponse {
    title: String,
    count: i32,
}

#[get("/query")]
async fn query_handler(Query(query): Query<TestQuery>) -> ukiapi::Json<QueryResponse> {
    ukiapi::Json(QueryResponse {
        page: query.page,
        name: query.name,
    })
}

#[post("/body")]
async fn body_handler(ValidatedJson(body): ValidatedJson<TestBody>) -> ukiapi::Json<BodyResponse> {
    ukiapi::Json(BodyResponse {
        title: body.title,
        count: body.count,
    })
}

#[tokio::test]
async fn test_query_extractor_valid() {
    let api = routes![(), query_handler_route()];
    let client = TestClient::new(api, ());

    let response = client.get("/query?page=5&name=test").send().await;
    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_query_extractor_validation_failure() {
    let api = routes![(), query_handler_route()];
    let client = TestClient::new(api, ());

    let response = client.get("/query?page=0&name=test").send().await;
    assert_eq!(response.status(), 422);
}

#[tokio::test]
async fn test_query_extractor_missing_param() {
    let api = routes![(), query_handler_route()];
    let client = TestClient::new(api, ());

    let response = client.get("/query?page=1").send().await;
    assert_eq!(response.status(), 422);
}

#[tokio::test]
async fn test_validated_json_extractor_valid() {
    let api = routes![(), body_handler_route()];
    let client = TestClient::new(api, ());

    let body = TestBody {
        title: "Test".to_string(),
        count: 5,
    };
    let response = client.post("/body", &body).send().await;
    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_validated_json_extractor_invalid_body() {
    let api = routes![(), body_handler_route()];
    let client = TestClient::new(api, ());

    let body = TestBody {
        title: "".to_string(),
        count: 5,
    };
    let response = client.post("/body", &body).send().await;
    assert_eq!(response.status(), 422);
}

#[tokio::test]
async fn test_validated_json_extractor_validation_failure() {
    let api = routes![(), body_handler_route()];
    let client = TestClient::new(api, ());

    let body = TestBody {
        title: "Test".to_string(),
        count: -1,
    };
    let response = client.post("/body", &body).send().await;
    assert_eq!(response.status(), 422);
}

#[tokio::test]
async fn test_validated_json_extractor_malformed_json() {
    let api = routes![(), body_handler_route()];
    let client = TestClient::new(api, ());

    // We can use the TestClient::post method if we pass an empty string slice or something else,
    // but the test client serializes `T` into JSON. So `client.post("/body", &"")` sends `""` which is a valid JSON string,
    // but not a valid `TestBody` struct (which expects an object).
    // Let's send a valid JSON string instead of an object to trigger a JSON schema error from serde.
    let response = client.post("/body", &"this is a string, not a test body object").send().await;

    // We expect UNPROCESSABLE_ENTITY (422) as it fails the serde deserialization
    // mapped in `ValidatedJson`'s `from_request`.
    assert_eq!(response.status(), 422);

    // Let's also verify the exact structure of the error detail.
    use ukiapi::test_client::ResponseExt;
    let body: serde_json::Value = response.json().await;
    let detail = body.get("detail").unwrap().as_str().unwrap();
    assert!(detail.starts_with("Invalid JSON: "));
}

#[tokio::test]
async fn test_query_extractor_malformed_query() {
    let api = routes![(), query_handler_route()];
    let client = TestClient::new(api, ());

    // Send malformed query parameter that fails serde parsing
    // page is expected to be u32, send a string instead
    let response = client.get("/query?page=not_a_number&name=test").send().await;

    assert_eq!(response.status(), 422);

    use ukiapi::test_client::ResponseExt;
    let body: serde_json::Value = response.json().await;
    let detail = body.get("detail").unwrap().as_str().unwrap();
    assert!(detail.starts_with("Invalid query parameters: "));
}
