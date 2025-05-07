fn main() {
    let mut x = 42;
    let raw_const: *const i32 = &x;
    let raw_mut: *mut i32 = &mut x;

    unsafe {
        *raw_mut += 1;
        println!("raw_mut: {}", *raw_mut);
        println!("raw_const: {}", *raw_const);
    }
}

