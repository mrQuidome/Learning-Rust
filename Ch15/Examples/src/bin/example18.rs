static mut COUNTER: i32 = 0;

fn main() {
    unsafe {
        COUNTER += 1;
        let value = COUNTER;
        println!("Counter: {}", value);
    }
}
