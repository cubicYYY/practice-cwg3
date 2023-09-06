use std::{
    alloc::{alloc, dealloc, handle_alloc_error, Layout},
    ops::Deref,
};

// Encapsulated data container
pub struct MyRcBox<T> {
    rc: u32,
    value: T,
}

impl<T> MyRcBox<T> {
    pub fn new(value: T) -> MyRcBox<T> {
        MyRcBox::<T> { rc: 1, value }
    }
    // Not atomic, not thread-safe.
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
    ptr: *mut MyRcBox<T>, // a have-to
}

impl<T> MyRc<T> {
    pub fn new(value: T) -> MyRc<T> {
        // We have to use unsafe, see README
        unsafe {
            let layout = Layout::new::<MyRcBox<T>>();
            let ptr = alloc(layout);
            if ptr.is_null() { // alloc failed!
                handle_alloc_error(layout);
            }
            *(ptr as *mut MyRcBox<T>) = MyRcBox::<T>::new(value);
            MyRc::<T> {
                ptr: ptr as *mut MyRcBox<T>,
            }
        }
    }

    pub fn ref_count(&self) -> u32 {
        unsafe{(*self.ptr).ref_count()}
    }
}

impl<T> Clone for MyRc<T> {
    fn clone(&self) -> MyRc<T> {
        unsafe {
            (*self.ptr).increment();
            MyRc::<T> { ptr: self.ptr }
        }
    }
}

impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        unsafe {
            (*self.ptr).decrement();
            if (*self.ptr).ref_count() == 0 {
                let layout = Layout::new::<MyRcBox<T>>();
                dealloc(self.ptr as *mut u8, layout);
                print!("GC: Deallocated!");
            }
        }
    }
}

impl<T> Deref for MyRc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &(*self.ptr).value }
    }
}
