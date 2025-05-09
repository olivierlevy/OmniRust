// src/concurrency/scheduler.rs

use async_trait::async_trait;
use std::future::Future;
// use std::pin::Pin; // Unused
use tokio::task::JoinHandle;

/// A trait for scheduling asynchronous tasks.
#[async_trait]
pub trait TaskScheduler: Send + Sync {
    /// The type of handle returned when a task is scheduled, allowing for control over the task.
    type TaskHandle<T: Send + 'static>: Future<Output = Result<T, tokio::task::JoinError>> + Send;

    /// Schedules a task to be run asynchronously.
    /// The task is a future that resolves to a value of type `T`.
    fn schedule<F, T>(&self, task: F) -> Self::TaskHandle<T>
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static;
    
    // Optionally, add methods for scheduled/delayed tasks or recurring tasks if needed.
    // fn schedule_delayed<F, T>(&self, task: F, delay: std::time::Duration) -> Self::TaskHandle<T>
    // where
    //     F: Future<Output = T> + Send + 'static,
    //     T: Send + 'static;
}

/// A simple task scheduler that uses `tokio::spawn`.
#[derive(Debug, Default, Clone, Copy)]
pub struct TokioTaskScheduler;

impl TokioTaskScheduler {
    pub fn new() -> Self {
        TokioTaskScheduler
    }
}

#[async_trait]
impl TaskScheduler for TokioTaskScheduler {
    type TaskHandle<T: Send + 'static> = JoinHandle<T>;

    fn schedule<F, T>(&self, task: F) -> Self::TaskHandle<T>
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        tokio::spawn(task)
    }

    // Example for a delayed task if the trait were extended:
    // fn schedule_delayed<F, T>(&self, task: F, delay: std::time::Duration) -> Self::TaskHandle<T>
    // where
    //     F: Future<Output = T> + Send + 'static,
    //     T: Send + 'static,
    // {
    //     tokio::spawn(async move {
    //         tokio::time::sleep(delay).await;
    //         task.await
    //     })
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::time::Duration;

    #[tokio::test]
    async fn test_tokio_task_scheduler_schedule() {
        let scheduler = TokioTaskScheduler::new();
        let data = Arc::new(Mutex::new(0));
        let data_clone = Arc::clone(&data);

        let task_future = async move {
            // Simulate some work
            tokio::time::sleep(Duration::from_millis(10)).await;
            let mut num = data_clone.lock().unwrap();
            *num = 42;
            *num // Return the value
        };

        let handle = scheduler.schedule(task_future);
        let result = handle.await.unwrap();

        assert_eq!(result, 42);
        assert_eq!(*data.lock().unwrap(), 42);
    }

    #[tokio::test]
    async fn test_tokio_task_scheduler_multiple_tasks() {
        let scheduler = TokioTaskScheduler::new();
        let counter = Arc::new(Mutex::new(0i32));

        let mut handles = Vec::new();
        for i in 0..5 {
            let counter_clone = Arc::clone(&counter);
            let handle = scheduler.schedule(async move {
                tokio::time::sleep(Duration::from_millis(10 * i as u64)).await; // Stagger tasks
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
                *num // Return current count for this task
            });
            handles.push(handle);
        }

        let mut results = Vec::new();
        for handle in handles {
            results.push(handle.await.unwrap());
        }
        
        // Results might be e.g. [1, 2, 3, 4, 5] or other order depending on execution.
        // The important part is that all tasks ran.
        assert_eq!(*counter.lock().unwrap(), 5);
        assert_eq!(results.len(), 5);
        // Check if results contains 1 through 5 (order doesn't matter for this check)
        results.sort();
        assert_eq!(results, vec![1,2,3,4,5]);
    }

    // Example test for a delayed task if it were implemented:
    // #[tokio::test]
    // async fn test_tokio_task_scheduler_schedule_delayed() {
    //     let scheduler = TokioTaskScheduler::new();
    //     let start_time = tokio::time::Instant::now();
    //     let delay = Duration::from_millis(100);

    //     let handle = scheduler.schedule_delayed(async {
    //         // This task will run after the delay
    //         "done".to_string()
    //     }, delay);

    //     let result = handle.await.unwrap();
    //     let elapsed = start_time.elapsed();

    //     assert_eq!(result, "done");
    //     assert!(elapsed >= delay, "Task did not wait for the delay. Elapsed: {:?}", elapsed);
    // }
}
