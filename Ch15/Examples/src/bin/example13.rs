fn main() {
    let null_ptr: *const i32 = std::ptr::null();
    println!("{}", null_ptr.is_null());
}