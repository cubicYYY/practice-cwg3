use std::{ops::Deref, ptr::NonNull};

// Encapsulated data container
struct MyRcBox<T> {
    rc: u32,
    value: Box<T>,
}
impl<T> Drop for MyRcBox<T> {
    fn drop(&mut self) {
        println!("[MyGC] Actual data dropped.");
    }
}

impl<T> MyRcBox<T> {
    pub fn new(value: T) -> MyRcBox<T> {
        MyRcBox::<T> {
            rc: 1,
            value: Box::new(value),
        }
    }
    // Not atomic, not thread-safe.
    // You can make them atomic to obtain thread-safety
    pub fn increment(&mut self) {
        self.rc += 1;
    }
    pub fn decrement(&mut self) {
        self.rc -= 1;
    }
    pub fn ref_count(&self) -> u32 {
        self.rc
    }
}

pub struct MyRc<T> {
    boxed_ptr: NonNull<MyRcBox<T>>,
}

impl<T> MyRc<T> {
    pub fn new(value: T) -> MyRc<T> {
        let ptr = NonNull::new(Box::into_raw(Box::new(MyRcBox::new(value))))
            .expect("Allocation for boxed data failed!");
        MyRc { boxed_ptr: ptr }
    }

    pub fn strong_count(&self) -> u32 {
        unsafe { (*self.boxed_ptr.as_ptr()).ref_count() }
    }
}

impl<T> Clone for MyRc<T> {
    fn clone(&self) -> MyRc<T> {
        unsafe {
            (*self.boxed_ptr.as_ptr()).increment();
        }
        MyRc {
            boxed_ptr: self.boxed_ptr,
        }
    }
}

impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        unsafe {
            (*self.boxed_ptr.as_ptr()).decrement();
            if (*self.boxed_ptr.as_ptr()).ref_count() == 0 {
                // Need GC

                drop(Box::from_raw(self.boxed_ptr.as_ptr()));
            }
        }
    }
}

impl<T> Deref for MyRc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { (*self.boxed_ptr.as_ptr()).value.as_ref() }
    }
}
