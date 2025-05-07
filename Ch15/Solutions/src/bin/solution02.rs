fn main() {
    let mut x = 42;

    // Create immutable and mutable raw pointers
    let ptr_const: *const i32 = &x;
    let ptr_mut: *mut i32 = &mut x;

    unsafe {
        // Print the original value using the const pointer
        println!("Original value: {}", *ptr_const);

        // Modify the value through the mutable raw pointer
        *ptr_mut = 100;

        // Print the modified value using both pointers
        println!("Modified value via ptr_mut: {}", *ptr_mut);
        println!("Modified value via ptr_const: {}", *ptr_const);
    }
}
