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
