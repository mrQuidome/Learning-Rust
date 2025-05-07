use std::ptr;

fn main() {
    let ptr: *const i32 = ptr::null();

    unsafe {
        if ptr.is_null() {
            println!("Pointer is null, cannot dereference.");
        } else {
            println!("Value at pointer: {}", *ptr);
        }
    }
}
