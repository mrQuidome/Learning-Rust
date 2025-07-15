use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    let mut handles = vec![];

    for i in 1..=5 {
        let tx = tx.clone();
        let handle = thread::spawn(move || {
            tx.send(format!("Message from thread {}", i)).unwrap();
        });
        handles.push(handle);
    }

    drop(tx);

    for handle in handles {
        handle.join().unwrap();
    }

    for received in rx {
        println!("Received: {}", received);
    }
}
