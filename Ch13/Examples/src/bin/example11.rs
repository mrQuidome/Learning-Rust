use std::thread;

fn main() {
    let data = String::from("Hello, threads!");

    // String implements Send, so it can be moved into a thread.
    let handle = thread::spawn(move || {
        println!("{}", data);
    });

    handle.join().unwrap();
}
