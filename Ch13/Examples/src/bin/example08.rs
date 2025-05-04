use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel::<i32>();

    drop(tx); // Simulate the sender being dropped

    match rx.recv() {
        Ok(msg) => println!("Received: {}", msg),
        Err(e) => println!("Error receiving message: {}", e),
    }
}
