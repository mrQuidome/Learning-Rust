struct IntPointer {
    ptr: *mut i32,
}

impl IntPointer {
    fn new(value: i32) -> Self {
        // Allocate on the heap using Box and convert to raw pointer
        let boxed = Box::new(value);
        IntPointer {
            ptr: Box::into_raw(boxed),
        }
    }

    fn get(&self) -> i32 {
        unsafe {
            // SAFETY: We own the memory and it is valid
            *self.ptr
        }
    }

    fn set(&mut self, value: i32) {
        unsafe {
            // SAFETY: We have exclusive access to the pointer
            *self.ptr = value;
        }
    }
}

impl Drop for IntPointer {
    fn drop(&mut self) {
        unsafe {
            // Reconstruct Box to deallocate the memory safely
            drop(Box::from_raw(self.ptr));
        }
    }
}

fn main() {
    let mut ip = IntPointer::new(10);
    println!("Initial value: {}", ip.get());

    ip.set(42);
    println!("Updated value: {}", ip.get());
}
