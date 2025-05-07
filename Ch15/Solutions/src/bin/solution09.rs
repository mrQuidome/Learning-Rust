use std::ptr;
use std::alloc::{alloc, dealloc, Layout};

struct SafePtr {
    ptr: *mut i32,
}

impl SafePtr {
    fn new(value: i32) -> Self {
        let layout = Layout::new::<i32>();
        let ptr = unsafe { alloc(layout) as *mut i32 };

        if ptr.is_null() {
            panic!("Failed to allocate memory");
        }

        unsafe {
            *ptr = value;
        }

        Self { ptr }
    }

    fn get(&self) -> Option<i32> {
        if self.ptr.is_null() {
            None
        } else {
            unsafe { Some(*self.ptr) }
        }
    }

    fn set(&mut self, value: i32) {
        if !self.ptr.is_null() {
            unsafe {
                *self.ptr = value;
            }
        }
    }

    fn free(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                let layout = Layout::new::<i32>();
                dealloc(self.ptr as *mut u8, layout);
                self.ptr = ptr::null_mut();
            }
        }
    }
}

fn main() {
    let mut safe = SafePtr::new(10);

    if let Some(value) = safe.get() {
        println!("Initial value: {}", value);
    }

    safe.set(99);

    if let Some(value) = safe.get() {
        println!("Updated value: {}", value);
    }

    safe.free();

    // After freeing, get should return None
    if safe.get().is_none() {
        println!("Memory has been freed; pointer is now null.");
    }
}
