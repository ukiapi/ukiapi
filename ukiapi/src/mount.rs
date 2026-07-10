use crate::routing::{Routable, UkiApi};

pub struct Mount<S> {
    pub path: String,
    pub directory: String,
    pub _phantom: std::marker::PhantomData<S>,
}

impl<S> Routable<S> for Mount<S> {
    fn add_to_api(self, api: &mut UkiApi<S>) {
        api.mounts.push(self);
    }
}
