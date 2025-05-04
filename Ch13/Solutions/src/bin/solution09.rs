use std::sync::{Arc, Barrier};
use std::thread;
use std::time::Duration;

fn main() {
    const WORKER_COUNT: usize = 5;
    let barrier = Arc::new(Barrier::new(WORKER_COUNT));
    let mut handles = vec![];

    for i in 0..WORKER_COUNT {
        let barrier_clone = Arc::clone(&barrier);
        let handle = thread::spawn(move || {
            println!("Worker {} is preparing...", i);
            thread::sleep(Duration::from_millis(100 * (i as u64 + 1))); // Simulate varied setup time
            println!("Worker {} is ready, waiting at the barrier.", i);

            barrier_clone.wait(); // Wait for all workers

            println!("Worker {} is starting the task!", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Worker thread panicked");
    }
}
