use axum::{
    extract::FromRequestParts,
    http::request::Parts,
};
use std::marker::PhantomData;
use std::future::Future;
use crate::HTTPException;

/// A trait for dependencies that can be resolved from request parts and state.
///
/// Dependencies are cached by default within a single request.
pub trait Dependency<S>: Send + Sync + 'static {
    /// The type of the value produced by this dependency.
    type Output: Clone + Send + Sync + 'static;

    /// Resolve the dependency.
    fn resolve(parts: &mut Parts, state: &S) -> impl Future<Output = Result<Self::Output, HTTPException>> + Send;
}

/// An extractor that resolves a dependency and caches the result.
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
        parts.extensions.insert(CachedDependency::<D, S>(resolved.clone(), PhantomData));

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
