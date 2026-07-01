use crate::extract::FromRequestParts;
use crate::http::request::Parts;
use crate::response::HTTPException;
use futures::future::BoxFuture;
use std::sync::{Arc, Mutex};

/// A collection of tasks to be executed in the background after the response is sent.
#[derive(Clone, Default)]
pub struct BackgroundTasks {
    tasks: Arc<Mutex<Vec<BoxFuture<'static, ()>>>>,
}

impl BackgroundTasks {
    /// Add a task to be executed in the background.
    pub fn add_task<F>(&self, task: F)
    where
        F: std::future::Future<Output = ()> + Send + 'static,
    {
        let mut tasks = self.tasks.lock().unwrap();
        tasks.push(Box::pin(task));
    }

    pub(crate) fn take_tasks(&self) -> Vec<BoxFuture<'static, ()>> {
        let mut tasks = self.tasks.lock().unwrap();
        std::mem::take(&mut *tasks)
    }
}

impl<S> FromRequestParts<S> for BackgroundTasks
where
    S: Send + Sync,
{
    type Rejection = HTTPException;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        if let Some(tasks) = parts.extensions.get::<BackgroundTasks>() {
            return Ok(tasks.clone());
        }

        // This should not happen if the middleware is correctly applied.
        let tasks = BackgroundTasks::default();
        parts.extensions.insert(tasks.clone());
        Ok(tasks)
    }
}
