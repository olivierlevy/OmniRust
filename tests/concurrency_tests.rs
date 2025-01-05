use omnirust::concurrency::thread_pool::ThreadPool;

#[test]
fn test_thread_pool() {
    let pool = ThreadPool::new(4);

    for i in 0..8 {
        pool.execute(move || {
            println!("Processing task {}", i);
        });
    }
}
