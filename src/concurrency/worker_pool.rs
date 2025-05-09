// src/concurrency/worker_pool.rs

use std::sync::{mpsc, Arc, Mutex};
use std::thread;

// Type alias for a job the worker pool can execute.
// Jobs are closures that are Send + 'static.
type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

#[derive(Debug)] // Added Debug derive
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap_or_else(|e| {
                // If recv() fails, it usually means the sender has disconnected.
                // This can happen if the WorkerPool is dropped.
                // We can treat this as a signal to terminate the worker thread.
                eprintln!("Worker {} disconnected; terminating. Error: {}", id, e);
                Message::Terminate
            });

            match message {
                Message::NewJob(job) => {
                    // println!("Worker {} got a job; executing.", id);
                    job();
                    // println!("Worker {} finished job.", id);
                }
                Message::Terminate => {
                    // println!("Worker {} was told to terminate.", id);
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

/// A simple thread pool for executing jobs.
#[derive(Debug)]
pub struct WorkerPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Message>>, // Option to allow for graceful shutdown
}

impl WorkerPool {
    /// Creates a new WorkerPool with `size` threads.
    /// Panics if `size` is 0.
    pub fn new(size: usize) -> WorkerPool {
        assert!(size > 0, "WorkerPool size must be greater than 0");

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        WorkerPool { workers, sender: Some(sender) }
    }

    /// Executes a job in the worker pool.
    /// The job `f` must be a closure that is `FnOnce() + Send + 'static`.
    ///
    /// # Panics
    /// Panics if the pool has been shut down (sender is None).
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        if let Some(sender) = self.sender.as_ref() {
            sender.send(Message::NewJob(job)).unwrap_or_else(|e| {
                // This might happen if all worker threads have panicked and exited.
                eprintln!("Failed to send job to worker pool: {}. Pool might be broken.", e);
            });
        } else {
            // This case should ideally not be reached if Drop is implemented correctly
            // and execute is not called after shutdown.
            panic!("WorkerPool has been shut down, cannot execute new jobs.");
        }
    }
}

impl Drop for WorkerPool {
    fn drop(&mut self) {
        // println!("Sending terminate message to all workers.");
        if let Some(sender) = self.sender.take() { // Take ownership of sender
            for _ in &self.workers {
                if sender.send(Message::Terminate).is_err() {
                    // This means a receiver has already hung up, possibly due to panic.
                    // println!("Failed to send terminate to a worker; it might have already exited.");
                }
            }
        }
        
        // println!("Shutting down all workers.");
        for worker in &mut self.workers {
            // println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap_or_else(|e| {
                     eprintln!("Worker {} panicked during shutdown: {:?}", worker.id, e);
                });
                // println!("Worker {} shut down.", worker.id);
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::Duration;

    #[test]
    fn test_worker_pool_executes_jobs() {
        let pool = WorkerPool::new(4);
        let counter = Arc::new(AtomicUsize::new(0));

        for _ in 0..8 {
            let counter_clone = Arc::clone(&counter);
            pool.execute(move || {
                // Simulate work
                thread::sleep(Duration::from_millis(50));
                counter_clone.fetch_add(1, Ordering::SeqCst);
            });
        }

        // Wait for jobs to complete by dropping the pool (which joins threads)
        // Or, for testing, we might need a more robust way to know jobs are done.
        // For this simple test, we'll drop and then check.
        drop(pool); 
        
        // Add a small delay to ensure threads have time to finish after drop starts.
        // This is a bit of a hack for testing; proper synchronization would be better.
        thread::sleep(Duration::from_millis(500));


        assert_eq!(counter.load(Ordering::SeqCst), 8);
    }

    #[test]
    fn test_worker_pool_panic_in_job() {
        let pool = WorkerPool::new(2);
        let job_completed_normally = Arc::new(AtomicUsize::new(0));
        
        let completed_clone = Arc::clone(&job_completed_normally);
        pool.execute(move || {
            thread::sleep(Duration::from_millis(10));
            completed_clone.fetch_add(1, Ordering::SeqCst);
        });

        // This job will panic
        pool.execute(|| {
            thread::sleep(Duration::from_millis(10));
            panic!("Test panic in a worker job");
        });
        
        let completed_clone2 = Arc::clone(&job_completed_normally);
         pool.execute(move || {
            thread::sleep(Duration::from_millis(10));
            completed_clone2.fetch_add(1, Ordering::SeqCst);
        });

        drop(pool);
        thread::sleep(Duration::from_millis(200)); // Give time for threads to process

        // Check that non-panicking jobs still completed.
        // One worker thread might die due to panic, but others should continue.
        // The exact number here depends on how quickly the pool reassigns or if a worker is lost.
        // For this simple pool, a panic in a worker's job will cause that worker's thread to terminate.
        // The other workers should still process jobs.
        assert_eq!(job_completed_normally.load(Ordering::SeqCst), 2, "Expected 2 jobs to complete normally");
    }
    
    #[test]
    #[should_panic]
    fn test_worker_pool_new_with_zero_size() {
        let _pool = WorkerPool::new(0);
    }
}
