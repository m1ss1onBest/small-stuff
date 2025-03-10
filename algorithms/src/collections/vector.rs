use std::{
    alloc::{alloc, dealloc, Layout},
    ptr::{self},
};

pub struct Vector<T> {
    ptr: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> Vector<T> {
    pub fn new() -> Vector<T> {
        Vector {
            ptr: ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> Vector<T> {
        Vector {
            ptr: if capacity > 0 {
                unsafe { alloc(Self::vec_layout(capacity)) as *mut T }
            } else {
                ptr::null_mut()
            },
            len: 0,
            capacity,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn push(&mut self, val: T) {
        if self.len == self.capacity {
            self.realloc();
        }
        unsafe {
            self.ptr.add(self.len).write(val);
        }
        self.len += 1;
    }

    pub fn push_front(&mut self, val: T) {
        if self.len == self.capacity {
            self.realloc();
        }

        if self.len > 0 {
            unsafe {
                ptr::copy(self.ptr, self.ptr.add(1), self.len);
            }
        }

        unsafe {
            self.ptr.write(val)
        }
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            let res = unsafe { self.ptr.add(self.len).read() };
            Some(res)
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            unsafe {
                let res = self.ptr.read();

                if self.len > 1 {
                    ptr::copy(self.ptr.add(1), self.ptr, self.len - 1); 
                }
                
                self.len -= 1;
                Some(res)
            }
        }
    }

    pub fn iter(&self) -> VectorIter<T> {
        VectorIter {
            vector: self,
            index: 0,
        }
    }

    fn realloc(&mut self) {
        let new_cap = if self.capacity == 0 { 1 } else { self.capacity * 2 };
        let new_layout = Self::vec_layout(new_cap);

        let new_ptr = unsafe { alloc(new_layout) as *mut T };
        unsafe {
            ptr::copy_nonoverlapping(self.ptr, new_ptr, self.len);
        }

        if !self.ptr.is_null() {
            let old_layout = Self::vec_layout(self.capacity);
            unsafe { dealloc(self.ptr as *mut u8, old_layout);}
        }

        self.ptr = new_ptr;
        self.capacity = new_cap;
    }

    fn vec_layout(capacity: usize) -> Layout {
        // it's ok to use expect here because it won't panic "unexpectedly"
        Layout::array::<T>(capacity).expect("failed to create layout")
    }
}

impl<T> Drop for Vector<T> {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                let layout = Self::vec_layout(self.capacity);
                dealloc(self.ptr as *mut u8, layout);
            }
        }
    }
}

pub struct VectorIter<'a, T> {
    vector: &'a Vector<T>,
    index: usize,
}

impl<'a, T> Iterator for VectorIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vector.len {
            let item = unsafe { &*self.vector.ptr.add(self.index) };
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}