use std::sync::Arc;
use std::thread;

fn main() {
    let data = Arc::new(String::from("Hello, threads!"));

    let mut handles = vec![];
    for _ in 0..5 {
        let data = Arc::clone(&data);

        // Arc<T> is Sync, so it can be shared between threads safely.
        let handle = thread::spawn(move || {
            println!("{}", data);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
