use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let resource1 = Arc::new(Mutex::new(()));
    let resource2 = Arc::new(Mutex::new(()));

    let r1 = Arc::clone(&resource1);
    let r2 = Arc::clone(&resource2);

    let handle1 = thread::spawn(move || {
        let _lock1 = r1.lock().unwrap();
        thread::sleep(Duration::from_millis(100));
        let _lock2 = r2.lock().unwrap();
        println!("Thread 1 acquired both locks");
    });

    let r1 = Arc::clone(&resource1);
    let r2 = Arc::clone(&resource2);

    let handle2 = thread::spawn(move || {
        let _lock1 = r1.lock().unwrap();
        thread::sleep(Duration::from_millis(100));
        let _lock2 = r2.lock().unwrap();
        println!("Thread 2 acquired both locks");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}
