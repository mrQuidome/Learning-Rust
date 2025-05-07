// Unsafe function that modifies the value at a raw pointer
unsafe fn modify_value(ptr: *mut i32) {
    if !ptr.is_null() {
        unsafe {
            *ptr += 1;
        }
    }
}

fn main() {
    let mut value = 10;
    let ptr: *mut i32 = &mut value;

    unsafe {
        modify_value(ptr); // call the unsafe function
    }

    println!("Modified value: {}", value); // should print 11
}
