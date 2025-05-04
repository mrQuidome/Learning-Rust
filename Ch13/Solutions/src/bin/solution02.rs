use std::thread;

fn main() {
    let number = 6;

    // Spawn a thread to compute the square
    let handle = thread::spawn(move || {
        number * number
    });

    // Wait for the result and print it
    let result = handle.join().expect("Thread panicked");
    println!("The square is: {}", result);
}
