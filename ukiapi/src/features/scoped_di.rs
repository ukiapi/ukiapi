use crate::background_tasks::BackgroundTasks;
use crate::extract::FromRequestParts;
use crate::http::request::Parts;
use crate::response::HTTPException;
use axum::http::StatusCode;
use futures::future::BoxFuture;
use std::fmt::Display;
use std::future::Future;
use std::marker::PhantomData;

/// An error that might occur when resolving a scoped dependency.
pub enum ScopedDiError {
    /// A generic message error.
    Message(String),
    /// A pre-constructed HTTPException.
    Http(HTTPException),
}

impl Display for ScopedDiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Message(msg) => write!(f, "{}", msg),
            Self::Http(err) => write!(f, "HTTPException: {}", err),
        }
    }
}

impl From<ScopedDiError> for HTTPException {
    fn from(err: ScopedDiError) -> Self {
        match err {
            ScopedDiError::Message(msg) => {
                HTTPException::new(StatusCode::INTERNAL_SERVER_ERROR, msg)
            }
            ScopedDiError::Http(http_err) => http_err,
        }
    }
}

/// A dependency that has a request-scoped lifecycle.
/// Its `resolve` method returns the injected value along with an asynchronous
/// teardown logic (cleanup future) that will be run in the background after
/// the HTTP response has been sent.
pub trait ScopedDependency<S>: Send + Sync + 'static {
    /// The type of the value produced by this dependency.
    type Output: Send + Sync + 'static;

    /// Resolve the dependency, returning the resolved output along with a teardown future.
    fn resolve<'a>(
        parts: &'a mut Parts,
        state: &'a S,
    ) -> impl Future<Output = Result<(Self::Output, BoxFuture<'static, ()>), ScopedDiError>> + Send + 'a;
}

/// An extractor that resolves a scoped dependency, registering its teardown
/// logic into `BackgroundTasks` automatically.
pub struct ScopedDepends<D: ScopedDependency<S>, S = ()>(pub D::Output, pub PhantomData<S>);

impl<D, S> FromRequestParts<S> for ScopedDepends<D, S>
where
    D: ScopedDependency<S>,
    S: Send + Sync + 'static,
{
    type Rejection = HTTPException;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let (resolved_value, teardown_future) = D::resolve(parts, state)
            .await
            .map_err(Into::<HTTPException>::into)?;

        let background_tasks = if let Some(tasks) = parts.extensions.get::<BackgroundTasks>() {
            tasks.clone()
        } else {
            let tasks = BackgroundTasks::default();
            parts.extensions.insert(tasks.clone());
            tasks
        };

        background_tasks.add_task(teardown_future);

        Ok(ScopedDepends(resolved_value, PhantomData))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::background_tasks::BackgroundTasks;
    use crate::extract::FromRequestParts;
    use axum::http::Request;
    use futures::future::BoxFuture;

    struct TestScopedDependency;

    impl ScopedDependency<()> for TestScopedDependency {
        type Output = String;

        async fn resolve(
            _parts: &mut Parts,
            _state: &(),
        ) -> Result<(Self::Output, BoxFuture<'static, ()>), ScopedDiError> {
            let output = "scoped_value".to_string();
            let teardown_future = Box::pin(async {});
            Ok((output, teardown_future))
        }
    }

    #[tokio::test]
    async fn test_scoped_depends_resolution_and_teardown() {
        let req = Request::builder().uri("/").body(()).unwrap();
        let (parts, _) = req.into_parts();

        let mut my_parts = parts;

        let state = ();
        let result =
            ScopedDepends::<TestScopedDependency, ()>::from_request_parts(&mut my_parts, &state)
                .await;

        assert!(result.is_ok());
        let extracted = result.unwrap();
        assert_eq!(extracted.0, "scoped_value");

        let tasks = my_parts.extensions.get::<BackgroundTasks>().unwrap();
        let pending_tasks = tasks.take_tasks();
        assert_eq!(pending_tasks.len(), 1);
    }
}
