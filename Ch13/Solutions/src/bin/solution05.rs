use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

fn main() {
    let catalog = Arc::new(RwLock::new(vec![
        String::from("The Rust Programming Language"),
        String::from("Programming in Rust"),
    ]));

    let mut handles = vec![];

    // Spawn multiple reader threads
    for i in 0..3 {
        let catalog_clone = Arc::clone(&catalog);
        let handle = thread::spawn(move || {
            let books = catalog_clone.read().expect("Failed to acquire read lock");
            println!("Reader {} sees catalog: {:?}", i, *books);
        });
        handles.push(handle);
    }

    // Writer thread: adds a new book
    {
        let catalog_clone = Arc::clone(&catalog);
        let handle = thread::spawn(move || {
            // Give readers a chance to start
            thread::sleep(Duration::from_millis(50));
            let mut books = catalog_clone.write().expect("Failed to acquire write lock");
            books.push(String::from("Rust Concurrency Explained"));
            println!("Writer added a new book.");
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Thread panicked");
    }

    // Final state of the catalog
    let final_catalog = catalog.read().expect("Failed to acquire final read lock");
    println!("Final catalog: {:?}", *final_catalog);
}
