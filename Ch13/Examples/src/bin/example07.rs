use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        tx.send("Hello from the thread!").unwrap();
    });

    let message = rx.recv().unwrap();
    println!("Main thread received: {}", message);

    handle.join().unwrap();
}
