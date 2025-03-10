pub struct Vector<T> {
    ptr: *mut T,
    length: usize,
    capacity: usize,
}

impl<T> Vector<T> {
    pub fn new() -> Vector<T> {
        Vector {
            ptr: std::ptr::null_mut(),
            capacity: 0,
            length: 0,
        }
    }
}
