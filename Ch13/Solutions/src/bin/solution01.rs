use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("Executing the thread...!");
    });

    handle.join().ok();
}
