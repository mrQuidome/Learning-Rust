use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let catalog = Arc::new(RwLock::new(vec![1, 2, 3]));

    let mut handles = vec![];

    // Readers
    for _ in 0..5 {
        let catalog = Arc::clone(&catalog);
        let handle = thread::spawn(move || {
            let read_lock = catalog.read().unwrap();
            println!("Thread {:?} read: {:?}", thread::current().id(), *read_lock);
        });
        handles.push(handle);
    }

    // Writer
    let catalog_writer = Arc::clone(&catalog);
    let writer = thread::spawn(move || {
        let mut write_lock = catalog_writer.write().unwrap();
        write_lock.push(4);
        println!("Thread {:?} wrote to the catalog.", thread::current().id());
    });

    handles.push(writer);

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final catalog: {:?}", *catalog.read().unwrap());
}
