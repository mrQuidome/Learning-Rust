use std::sync::{Arc, Barrier};
use std::thread;

fn main() {
    let num_threads = 5;
    let barrier = Arc::new(Barrier::new(num_threads));
    let mut handles = vec![];

    for i in 0..num_threads {
        let barrier_clone = Arc::clone(&barrier);
        let handle = thread::spawn(move || {
            println!("Thread {}: Performing initial work...", i);
            // Simulate some work before the barrier
            std::thread::sleep(std::time::Duration::from_millis(100 * i as u64));

            println!("Thread {}: Waiting at the barrier.", i);
            barrier_clone.wait(); // Synchronize here
            println!("Thread {}: Proceeding after the barrier.", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
