fn to_raw_pointer(x: &i32) -> *const i32 {
    x as *const i32
}

fn main() {
    let value = 123;
    let ptr = to_raw_pointer(&value);

    unsafe {
        println!("Value at pointer: {}", *ptr);
    }
}
