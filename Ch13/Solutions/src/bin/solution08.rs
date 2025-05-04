use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (sender, receiver) = mpsc::channel();

    let mut handles = vec![];

    // First producer sends 1 to 5
    let tx1 = sender.clone();
    let handle1 = thread::spawn(move || {
        for i in 1..=5 {
            println!("Producer 1 sending {}", i);
            tx1.send(i).expect("Failed to send from producer 1");
            thread::sleep(Duration::from_millis(100));
        }
    });
    handles.push(handle1);

    // Second producer sends 100 to 105
    let tx2 = sender.clone();
    let handle2 = thread::spawn(move || {
        for i in 100..=105 {
            println!("Producer 2 sending {}", i);
            tx2.send(i).expect("Failed to send from producer 2");
            thread::sleep(Duration::from_millis(150));
        }
    });
    handles.push(handle2);

    // Drop the original sender so the consumer stops when all producers are done
    drop(sender);

    // Consumer thread
    let consumer = thread::spawn(move || {
        while let Ok(num) = receiver.recv() {
            println!("Consumer received {}", num);
        }
        println!("Consumer: channel closed, exiting.");
    });

    for handle in handles {
        handle.join().expect("Producer thread panicked");
    }

    consumer.join().expect("Consumer thread panicked");
}
