fn fill_buffer(ptr: *mut u8, len: usize) {
    unsafe {
        for i in 0..len {
            *ptr.add(i) = 0xAA;
        }
    }
}

fn main() {
    // Create a buffer on the stack
    let mut buffer = [0u8; 8];

    // Get a raw pointer to the buffer
    let ptr = buffer.as_mut_ptr();
    let len = buffer.len();

    // Call the unsafe buffer-filling function
    fill_buffer(ptr, len);

    // Print the contents to verify the pattern
    println!("Buffer contents:");
    for byte in &buffer {
        println!("{:02X}", byte);
    }
}
