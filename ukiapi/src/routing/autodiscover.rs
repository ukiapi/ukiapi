use crate::routing::route::Route;

/// Trait implemented by route registry entries.
pub trait RegistryEntry<S>: ::inventory::Collect {
    /// Return the registered route.
    fn get_route(&self) -> Route<S>;
}

/// A default stateless route container for autodiscovery.
pub struct DefaultRoute {
    /// The route companion function.
    pub route_fn: fn() -> Route<()>,
}

inventory::collect!(DefaultRoute);

impl RegistryEntry<()> for DefaultRoute {
    fn get_route(&self) -> Route<()> {
        (self.route_fn)()
    }
}

/// Wrapper for route function pointers of stateless and stateful routes.
#[derive(Copy, Clone)]
pub enum RouteWrapper<S> {
    Stateless(fn() -> Route<()>),
    Stateful(fn() -> Route<S>),
}

/// Submit a route to a registry.
///
/// Usage:
/// - `submit_route!(route_fn)` submits a stateless route.
/// - `submit_route!(route_fn, RegistryName, stateless)` submits a stateless route to a custom registry.
/// - `submit_route!(route_fn, RegistryName, stateful)` submits a stateful route to a custom registry.
#[macro_export]
macro_rules! submit_route {
    ($route_fn:expr) => {
        $crate::inventory::submit! {
            $crate::routing::autodiscover::DefaultRoute { route_fn: $route_fn }
        }
    };
    ($route_fn:expr, $registry_name:path, stateless) => {
        $crate::inventory::submit! {
            $registry_name {
                wrapper: $crate::routing::autodiscover::RouteWrapper::Stateless($route_fn)
            }
        }
    };
    ($route_fn:expr, $registry_name:path, stateful) => {
        $crate::inventory::submit! {
            $registry_name {
                wrapper: $crate::routing::autodiscover::RouteWrapper::Stateful($route_fn)
            }
        }
    };
}

/// Declare a custom route registry for stateful routing.
///
/// Usage: `declare_registry!(AppState, AppStateRoute)`
#[macro_export]
macro_rules! declare_registry {
    ($state:ty, $registry_name:ident) => {
        pub struct $registry_name {
            pub wrapper: $crate::routing::autodiscover::RouteWrapper<$state>,
        }
        $crate::inventory::collect!($registry_name);

        impl $crate::routing::autodiscover::RegistryEntry<$state> for $registry_name {
            fn get_route(&self) -> $crate::Route<$state> {
                match self.wrapper {
                    $crate::routing::autodiscover::RouteWrapper::Stateless(f) => {
                        f().with_state::<$state>()
                    }
                    $crate::routing::autodiscover::RouteWrapper::Stateful(f) => f(),
                }
            }
        }
    };
}
