use crate::extract::FromRequestParts;
use crate::http::request::Parts;
use crate::response::HTTPException;
use std::future::Future;
use std::marker::PhantomData;

/// A trait for dependencies that can be resolved from request parts and state.
///
/// Dependencies are cached by default within a single request.
pub trait Dependency<S>: Send + Sync + 'static {
    /// The type of the value produced by this dependency.
    type Output: Clone + Send + Sync + 'static;

    /// Resolve the dependency.
    fn resolve(
        parts: &mut Parts,
        state: &S,
    ) -> impl Future<Output = Result<Self::Output, HTTPException>> + Send;
}

/// An extractor that resolves a dependency and caches the result.
#[derive(Debug)]
pub struct Depends<D: Dependency<S>, S = ()>(pub D::Output, pub PhantomData<S>);

/// Internal wrapper for caching resolved dependencies in request extensions.
struct CachedDependency<D, S>(D::Output, PhantomData<fn() -> (D, S)>)
where
    D: Dependency<S>;

impl<D, S> Clone for CachedDependency<D, S>
where
    D: Dependency<S>,
{
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData)
    }
}

impl<D, S> FromRequestParts<S> for Depends<D, S>
where
    D: Dependency<S>,
    S: Send + Sync + 'static,
{
    type Rejection = HTTPException;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Check if the dependency has already been resolved and cached in request extensions.
        if let Some(cached) = parts.extensions.get::<CachedDependency<D, S>>() {
            return Ok(Depends(cached.0.clone(), PhantomData));
        }

        // Resolve the dependency.
        let resolved = D::resolve(parts, state).await?;

        // Cache the resolved value in request extensions.
        parts
            .extensions
            .insert(CachedDependency::<D, S>(resolved.clone(), PhantomData));

        Ok(Depends(resolved, PhantomData))
    }
}

/// A wrapper for dependencies that should be executed but their return value is ignored.
/// Useful for dependencies that perform validation or authentication but don't return a value
/// needed by the handler.
pub struct Security<D: Dependency<S>, S = ()>(pub PhantomData<(D, S)>);

impl<D, S> FromRequestParts<S> for Security<D, S>
where
    D: Dependency<S>,
    S: Send + Sync + 'static,
{
    type Rejection = HTTPException;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let _ = Depends::<D, S>::from_request_parts(parts, state).await?;
        Ok(Security(PhantomData))
    }
}

/// Helper to create a Security extractor.
pub fn security<D: Dependency<S>, S>() -> Security<D, S> {
    Security(PhantomData)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::request::Parts;

    struct TestDependency;

    impl Dependency<()> for TestDependency {
        type Output = String;

        async fn resolve(_parts: &mut Parts, _state: &()) -> Result<String, HTTPException> {
            Ok("resolved_value".to_string())
        }
    }

    #[derive(Debug)]
    struct FailingDependency;

    impl Dependency<()> for FailingDependency {
        type Output = String;

        async fn resolve(_parts: &mut Parts, _state: &()) -> Result<String, HTTPException> {
            Err(HTTPException::new(
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Dependency failed",
            ))
        }
    }

    fn create_test_parts() -> Parts {
        let (parts, _) = axum::http::Request::builder()
            .body(())
            .unwrap()
            .into_parts();
        parts
    }

    #[tokio::test]
    async fn test_depends_resolves_dependency() {
        let mut parts = create_test_parts();
        let result = Depends::<TestDependency, ()>::from_request_parts(&mut parts, &()).await;
        assert!(result.is_ok());
        let Depends(value, _) = result.unwrap();
        assert_eq!(value, "resolved_value");
    }

    #[tokio::test]
    async fn test_depends_caches_result() {
        let mut parts = create_test_parts();
        let result1 = Depends::<TestDependency, ()>::from_request_parts(&mut parts, &()).await;
        assert!(result1.is_ok());

        let result2 = Depends::<TestDependency, ()>::from_request_parts(&mut parts, &()).await;
        assert!(result2.is_ok());

        let Depends(value1, _) = result1.unwrap();
        let Depends(value2, _) = result2.unwrap();
        assert_eq!(value1, value2);
    }

    #[tokio::test]
    async fn test_depends_propagates_error() {
        let mut parts = create_test_parts();
        let result = Depends::<FailingDependency, ()>::from_request_parts(&mut parts, &()).await;
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().status_code,
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[tokio::test]
    async fn test_security_wraps_depends() {
        let mut parts = create_test_parts();
        let result = Security::<TestDependency, ()>::from_request_parts(&mut parts, &()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_security_propagates_error() {
        let mut parts = create_test_parts();
        let result = Security::<FailingDependency, ()>::from_request_parts(&mut parts, &()).await;
        assert!(result.is_err());
    }

    #[test]
    fn test_security_helper() {
        let _security = security::<TestDependency, ()>();
    }
}
