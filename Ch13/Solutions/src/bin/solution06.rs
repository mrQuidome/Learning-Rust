use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let resource1 = Arc::new(Mutex::new(()));
    let resource2 = Arc::new(Mutex::new(()));

    let r1 = Arc::clone(&resource1);
    let r2 = Arc::clone(&resource2);
    let t1 = thread::spawn(move || {
        let _lock1 = r1.lock().unwrap();
        thread::sleep(Duration::from_millis(50)); // Simulate work
        let _lock2 = r2.lock().unwrap();
        println!("Thread 1 acquired both locks.");
    });

    let r1 = Arc::clone(&resource1);
    let r2 = Arc::clone(&resource2);
    let t2 = thread::spawn(move || {
        // switch these two locks to prevent a deadlock
        let _lock2 = r2.lock().unwrap();
        thread::sleep(Duration::from_millis(50)); // Simulate work
        let _lock1 = r1.lock().unwrap();
        println!("Thread 2 acquired both locks.");
    });

    t1.join().unwrap();
    t2.join().unwrap();
}
