use axum::extract::DefaultBodyLimit;
use axum::{
    extract::Request,
    middleware::{self, Next},
    Router,
};
use std::convert::Infallible;
use std::future::Future;
use tower_http::{
    compression::CompressionLayer, cors::CorsLayer, timeout::TimeoutLayer, trace::TraceLayer,
};

/// Re-exports for common middleware layers.
pub mod layers {
    pub use axum::extract::DefaultBodyLimit;
    pub use tower_http::{
        compression::CompressionLayer, cors::CorsLayer, timeout::TimeoutLayer, trace::TraceLayer,
    };
}

/// A trait for adding middleware to Ukidama or APIRouter.
pub trait MiddlewareExt<S>: Sized
where
    S: Clone + Send + Sync + 'static,
{
    /// Add a layer (middleware) to the router.
    fn layer<F>(self, f: F) -> Self
    where
        F: Fn(Router<S>) -> Router<S> + Send + Sync + 'static;

    /// Add a custom middleware function.
    ///
    /// # Example
    /// ```rust
    /// use ukidama::Request;
    /// use axum::middleware::Next;
    /// use axum::response::IntoResponse;
    ///
    /// async fn my_middleware(req: Request, next: Next) -> impl IntoResponse {
    ///     println!("Request to: {}", req.uri());
    ///     next.run(req).await
    /// }
    ///
    /// // api.middleware(my_middleware)
    /// ```
    fn middleware<F, Fut>(self, f: F) -> Self
    where
        F: Fn(Request, Next) -> Fut + Clone + Send + Sync + 'static,
        Fut: Future<Output = axum::response::Response> + Send + 'static,
    {
        self.layer(move |router| router.layer(middleware::from_fn(f.clone())))
    }

    /// Add a tower layer.
    fn use_layer<L>(self, layer: L) -> Self
    where
        L: tower::Layer<axum::routing::Route> + Clone + Send + Sync + 'static,
        L::Service: tower::Service<Request> + Clone + Send + Sync + 'static,
        <L::Service as tower::Service<Request>>::Response:
            axum::response::IntoResponse + Send + 'static,
        <L::Service as tower::Service<Request>>::Error: Into<Infallible> + Send + 'static,
        <L::Service as tower::Service<Request>>::Future: Send + 'static,
    {
        self.layer(move |router| router.layer(layer.clone()))
    }

    /// Add CORS middleware.
    fn cors(self, layer: CorsLayer) -> Self {
        self.use_layer(layer)
    }

    /// Add compression middleware.
    fn compression(self) -> Self {
        self.use_layer(CompressionLayer::new())
    }

    /// Add logging/tracing middleware using `TraceLayer`.
    fn logger(self) -> Self {
        self.use_layer(TraceLayer::new_for_http())
    }

    /// Add timeout middleware.
    fn timeout(self, duration: std::time::Duration) -> Self {
        self.use_layer(TimeoutLayer::new(duration))
    }

    /// Add request body size limit middleware.
    fn body_limit(self, limit: usize) -> Self {
        self.use_layer(DefaultBodyLimit::max(limit))
    }
}
