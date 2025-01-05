use tokio::runtime::Runtime;
use std::sync::Arc;

pub struct TaskScheduler {
    runtime: Arc<Runtime>,
}

impl TaskScheduler {
    pub fn new() -> Self {
        let runtime = Arc::new(Runtime::new().expect("Failed to create runtime"));
        TaskScheduler { runtime }
    }

    pub fn spawn<F>(&self, task: F)
    where
        F: std::future::Future<Output = ()> + Send + 'static,
    {
        self.runtime.spawn(task);
    }
}
