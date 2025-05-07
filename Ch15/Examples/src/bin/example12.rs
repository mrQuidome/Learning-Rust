fn main() {
    let mut value = 10;
    let ptr: *mut i32 = &mut value;

    let r = unsafe {
        *ptr = 20;
        &mut *ptr
    };

    *r += 1;
    println!("value: {}", r);
}
