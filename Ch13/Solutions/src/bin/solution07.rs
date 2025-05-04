use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (sender, receiver) = mpsc::channel();

    // Producer thread
    let producer = thread::spawn(move || {
        for i in 1..=5 {
            println!("Producing {}", i);
            sender.send(i).expect("Failed to send");
            thread::sleep(Duration::from_millis(100)); // Simulate work
        }
    });

    // Consumer thread
    let consumer = thread::spawn(move || {
        while let Ok(num) = receiver.recv() {
            println!("Consumed {}", num);
        }
    });

    producer.join().expect("Producer thread panicked");
    consumer.join().expect("Consumer thread panicked");
}
