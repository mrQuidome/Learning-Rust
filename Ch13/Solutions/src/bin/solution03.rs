use std::sync::Arc;
use std::thread;

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let shared_numbers = Arc::new(numbers);

    let mut handles = vec![];

    for i in 0..3 {
        let shared_clone = Arc::clone(&shared_numbers);
        let handle = thread::spawn(move || {
            println!("Thread {}: {:?}", i, shared_clone);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Thread panicked");
    }
}
