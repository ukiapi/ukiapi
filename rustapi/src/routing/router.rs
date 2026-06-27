use crate::dependencies::{Dependency, Depends};
use crate::extract::{FromRequestParts, Request, State};
use crate::middleware::{self, Next};
use crate::mount::Mount;
use crate::response::{HTTPException, IntoResponse};
use crate::routing::Router;
use crate::routing::{api::RustAPI, route::Route, Routable, RouterBuilder};
use std::sync::Arc;

/// A router for grouping routes with a common prefix and tags.
pub struct APIRouter<S = ()> {
    /// The prefix for all routes in this router.
    pub prefix: String,
    /// Tags for all routes in this router.
    pub tags: Vec<String>,
    /// The list of routes in this router.
    pub routes: Vec<Route<S>>,
    /// Static file mounts in this router.
    pub mounts: Vec<Mount<S>>,
    /// Layers applied to all routes in this router.
    pub layers: Vec<RouterBuilder<S>>,
}

impl<S> Default for APIRouter<S>
where
    S: Clone + Send + Sync + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<S> APIRouter<S>
where
    S: Clone + Send + Sync + 'static,
{
    /// Create a new APIRouter.
    pub fn new() -> Self {
        Self {
            prefix: String::new(),
            tags: Vec::new(),
            routes: Vec::new(),
            mounts: Vec::new(),
            layers: Vec::new(),
        }
    }

    /// Set a common prefix for all routes in this router.
    pub fn prefix(mut self, prefix: &str) -> Self {
        self.prefix = prefix.to_string();
        self
    }

    /// Add a common tag for all routes in this router.
    pub fn tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }

    /// Add a static file mount to this router.
    pub fn mount(mut self, path: &str, directory: &str) -> Self {
        self.mounts.push(Mount {
            path: format!("{}{}", self.prefix, path),
            directory: directory.to_string(),
            _phantom: std::marker::PhantomData,
        });
        self
    }

    /// Add a route or another router to this router.
    pub fn route<R: Routable<S>>(mut self, routable: R) -> Self {
        let mut temp_api = RustAPI::<S>::new();
        routable.add_to_api(&mut temp_api);
        for mut route in temp_api.routes {
            route.path = format!("{}{}", self.prefix, route.path);
            route.tags.extend(self.tags.clone());
            self.routes.push(route);
        }
        for mut mount in temp_api.mounts {
            mount.path = format!("{}{}", self.prefix, mount.path);
            self.mounts.push(mount);
        }
        self
    }

    /// Add a layer (middleware) to all routes in this router.
    pub fn layer<F>(mut self, f: F) -> Self
    where
        F: Fn(Router<S>) -> Router<S> + Send + Sync + 'static,
    {
        self.layers.push(Arc::new(f));
        self
    }

    /// Add a dependency to all routes in this router.
    pub fn dependency<D>(self, state: S) -> Self
    where
        D: Dependency<S>,
    {
        self.layer(move |router| {
            let state = state.clone();
            router.layer(middleware::from_fn_with_state(
                state,
                |State(state): State<S>, mut req: Request, next: Next| async move {
                    let (mut parts, body) = req.into_parts();
                    match Depends::<D, S>::from_request_parts(&mut parts, &state).await {
                        Ok(_) => {
                            req = Request::from_parts(parts, body);
                            next.run(req).await.into_response()
                        }
                        Err(err) => (err as HTTPException).into_response(),
                    }
                },
            ))
        })
    }

    pub fn autodiscover(mut self) -> Self {
        for entry in ::inventory::iter::<crate::routing::autodiscover::DefaultRoute> {
            let route_stateless = (entry.route_fn)();
            let route_stateful = route_stateless.with_state::<S>();
            self = self.route(route_stateful);
        }
        self
    }

    pub fn autodiscover_with<R>(mut self) -> Self
    where
        R: crate::routing::autodiscover::RegistryEntry<S> + ::inventory::Collect + 'static,
    {
        for entry in ::inventory::iter::<R> {
            self = self.route(entry.get_route());
        }
        self
    }
}

impl<S> Routable<S> for APIRouter<S>
where
    S: Clone + Send + Sync + 'static,
{
    fn add_to_api(self, api: &mut RustAPI<S>) {
        let layers = self.layers;
        for mut route in self.routes {
            let old_adder = route.adder;
            let layers_clone = layers.clone();
            route.adder = Box::new(move |mut router, path| {
                for layer_fn in &layers_clone {
                    router = layer_fn(router);
                }
                old_adder(router, path)
            });
            api.routes.push(route);
        }
        api.mounts.extend(self.mounts);
    }
}

impl<S> crate::routing::middleware::MiddlewareExt<S> for APIRouter<S>
where
    S: Clone + Send + Sync + 'static,
{
    fn layer<F>(self, f: F) -> Self
    where
        F: Fn(Router<S>) -> Router<S> + Send + Sync + 'static,
    {
        self.layer(f)
    }
}
