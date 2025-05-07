fn main() {
    let raw: * const i32 = std::ptr::null();

    unsafe {
        if !raw.is_null() {
            println!("{}", *raw);
        } else {
            println!("Pointer is null, cannot dereference.");
        }
    }
}
