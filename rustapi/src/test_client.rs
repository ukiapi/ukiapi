use crate::body::Body;
use crate::extract::Request;
use crate::response::AxumResponse;
use crate::routing::Router;
use crate::tower::ServiceExt;
use crate::RustAPI;

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
    pub async fn get(&self, uri: &str) -> AxumResponse {
        let req = Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty())
            .unwrap();
        self.router.clone().oneshot(req).await.unwrap()
    }

    /// Perform a POST request with a body.
    pub async fn post(&self, uri: &str, body: Body) -> AxumResponse {
        let req = Request::builder()
            .method("POST")
            .uri(uri)
            .body(body)
            .unwrap();
        self.router.clone().oneshot(req).await.unwrap()
    }
}
