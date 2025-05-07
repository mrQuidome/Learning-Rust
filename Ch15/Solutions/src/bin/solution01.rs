fn main() {
    let x = 42;
    let ptr: *const i32 = &x;

    unsafe {
        println!("Value pointed to by ptr: {}", *ptr);
    }
}
