use crate::http::request::Parts;
use axum::extract::{ConnectInfo, FromRequestParts};
use std::net::SocketAddr;

/// Details about the current HTTP connection.
#[derive(Debug, Clone, Copy)]
pub struct HTTPConnection {
    /// The address of the client that initiated the connection.
    pub client_addr: SocketAddr,
}

impl<S> FromRequestParts<S> for HTTPConnection
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let addr = parts
            .extensions
            .get::<ConnectInfo<SocketAddr>>()
            .map(|ConnectInfo(addr)| *addr)
            .unwrap_or_else(|| SocketAddr::from(([0, 0, 0, 0], 0)));

        Ok(HTTPConnection { client_addr: addr })
    }
}
