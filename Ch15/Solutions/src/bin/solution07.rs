struct SimpleBox {
    ptr: *mut i32,
}

impl SimpleBox {
    fn new(value: i32) -> Self {
        // Allocate memory on the heap and take ownership of the raw pointer
        let boxed = Box::new(value);
        Self {
            ptr: Box::into_raw(boxed),
        }
    }

    fn get(&self) -> i32 {
        unsafe {
            // SAFETY: We own the pointer and it's valid as long as `self` is alive
            *self.ptr
        }
    }

    fn set(&mut self, value: i32) {
        unsafe {
            // SAFETY: We have unique access to the data
            *self.ptr = value;
        }
    }
}

impl Drop for SimpleBox {
    fn drop(&mut self) {
        unsafe {
            // SAFETY: We are the sole owner, and we turn the raw pointer back into a Box to drop it
            drop(Box::from_raw(self.ptr));
        }
    }
}

fn main() {
    let mut smart = SimpleBox::new(123);
    println!("Initial value: {}", smart.get());

    smart.set(456);
    println!("Updated value: {}", smart.get());

    // When `smart` goes out of scope, memory is automatically freed
}
