use crate::routing::{Routable, Ukidama};

pub struct Mount<S> {
    pub path: String,
    pub directory: String,
    pub _phantom: std::marker::PhantomData<S>,
}

impl<S> Routable<S> for Mount<S> {
    fn add_to_api(self, api: &mut Ukidama<S>) {
        api.mounts.push(self);
    }
}
