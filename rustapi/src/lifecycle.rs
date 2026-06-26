use std::future::Future;
use std::pin::Pin;

pub type LifecycleHandler<S> =
    Box<dyn FnOnce(S) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send>;
