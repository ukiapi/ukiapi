use crate::body::Body;
use crate::extract::Request;
use crate::response::AxumResponse;
use crate::routing::Router;
use crate::tower::ServiceExt;
use crate::RustAPI;
use http_body_util::BodyExt;
use serde::Serialize;

/// A simple client for testing RustAPI endpoints.
pub struct TestClient {
    router: Router<()>,
}

impl TestClient {
    /// Create a new TestClient from a RustAPI instance and its state.
    pub fn new<S>(api: RustAPI<S>, state: S) -> Self
    where
        S: Clone + Send + Sync + 'static,
    {
        Self {
            router: api.build_router(state),
        }
    }

    /// Perform a GET request.
    pub fn get(&self, uri: &str) -> RequestBuilder {
        RequestBuilder::new(self.router.clone(), "GET", uri)
    }

    /// Perform a POST request with a body.
    pub fn post<T: Serialize>(&self, uri: &str, body: &T) -> RequestBuilder {
        let json = serde_json::to_vec(body).unwrap();
        RequestBuilder::new(self.router.clone(), "POST", uri)
            .header("Content-Type", "application/json")
            .body(Body::from(json))
    }
}

pub struct RequestBuilder {
    router: Router<()>,
    builder: axum::http::request::Builder,
    body: Body,
}

impl RequestBuilder {
    fn new(router: Router<()>, method: &str, uri: &str) -> Self {
        Self {
            router,
            builder: Request::builder().method(method).uri(uri),
            body: Body::empty(),
        }
    }

    pub fn header(mut self, key: &str, value: impl Into<String>) -> Self {
        self.builder = self.builder.header(key, value.into());
        self
    }

    pub fn body(mut self, body: Body) -> Self {
        self.body = body;
        self
    }

    pub async fn send(self) -> AxumResponse {
        let req = self.builder.body(self.body).unwrap();
        self.router.oneshot(req).await.unwrap()
    }
}

pub trait ResponseExt {
    fn json<T: serde::de::DeserializeOwned>(self) -> impl std::future::Future<Output = T> + Send;
}

impl ResponseExt for AxumResponse {
    async fn json<T: serde::de::DeserializeOwned>(self) -> T {
        let body_bytes = self.into_body().collect().await.unwrap().to_bytes();
        serde_json::from_slice(&body_bytes).unwrap()
    }
}
