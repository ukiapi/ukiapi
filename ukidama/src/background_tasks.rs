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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::request::Parts;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    fn create_test_parts() -> Parts {
        let (parts, _) = axum::http::Request::builder()
            .body(())
            .unwrap()
            .into_parts();
        parts
    }

    #[test]
    fn test_background_tasks_default() {
        let tasks = BackgroundTasks::default();
        let taken = tasks.take_tasks();
        assert!(taken.is_empty());
    }

    #[test]
    fn test_background_tasks_add_task() {
        let tasks = BackgroundTasks::default();
        tasks.add_task(async {});
        let taken = tasks.take_tasks();
        assert_eq!(taken.len(), 1);
    }

    #[test]
    fn test_background_tasks_add_multiple_tasks() {
        let tasks = BackgroundTasks::default();
        tasks.add_task(async {});
        tasks.add_task(async {});
        tasks.add_task(async {});
        let taken = tasks.take_tasks();
        assert_eq!(taken.len(), 3);
    }

    #[test]
    fn test_background_tasks_take_drains() {
        let tasks = BackgroundTasks::default();
        tasks.add_task(async {});
        let _ = tasks.take_tasks();
        let taken = tasks.take_tasks();
        assert!(taken.is_empty());
    }

    #[test]
    fn test_background_tasks_clone() {
        let tasks1 = BackgroundTasks::default();
        let tasks2 = tasks1.clone();
        tasks1.add_task(async {});
        let taken = tasks2.take_tasks();
        assert_eq!(taken.len(), 1);
    }

    #[tokio::test]
    async fn test_background_tasks_extract_from_parts() {
        let mut parts = create_test_parts();
        let result = BackgroundTasks::from_request_parts(&mut parts, &()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_background_tasks_extract_caches() {
        let mut parts = create_test_parts();
        let result1 = BackgroundTasks::from_request_parts(&mut parts, &())
            .await
            .unwrap();
        let result2 = BackgroundTasks::from_request_parts(&mut parts, &())
            .await
            .unwrap();
        result1.add_task(async {});
        let taken = result2.take_tasks();
        assert_eq!(taken.len(), 1);
    }

    #[tokio::test]
    async fn test_background_tasks_execute_tasks() {
        let tasks = BackgroundTasks::default();
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        tasks.add_task(async move {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });

        let mut taken = tasks.take_tasks();
        for task in taken.drain(..) {
            task.await;
        }

        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }
}
