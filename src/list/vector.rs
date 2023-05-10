use std::{
    alloc::Layout,
    ops::{Deref, DerefMut},
    ptr::null_mut,
};

pub struct Vector<T> {
    ptr: *mut T,
    cap: usize,
    len: usize,
}

impl<T> Vector<T> {
    pub fn new() -> Self {
        assert!(
            std::mem::size_of::<T>() != 0,
            "Zero-sized types not supported."
        );
        Vector {
            ptr: null_mut(),
            cap: 0,
            len: 0,
        }
    }

    fn grow(&mut self) {
        unsafe {
            (self.cap, self.ptr) = match self.cap {
                0 => {
                    let new_cap = 1;
                    let new_layout = Layout::array::<T>(new_cap).unwrap();
                    let new_ptr = std::alloc::alloc(new_layout);

                    if new_ptr.is_null() {
                        std::alloc::handle_alloc_error(new_layout);
                    }

                    (new_cap, new_ptr as *mut T)
                }
                _ => {
                    // grow capacity by a factor of 2
                    let new_cap = 2 * self.cap;

                    let old_layout = Layout::array::<T>(self.cap).unwrap();
                    let new_layout = Layout::array::<T>(2 * self.cap).unwrap();

                    // make sure the allocation size never exceeds isize::MAX
                    // as a nice consequence, this also protects against
                    // overflow for 2 * self.cap
                    assert!(
                        new_layout.size() <= isize::MAX as usize,
                        "Allocation too large!"
                    );

                    // reallocate the memory
                    let old_ptr = self.ptr as *mut u8;
                    let new_ptr = std::alloc::realloc(old_ptr, old_layout, new_layout.size());

                    if new_ptr.is_null() {
                        std::alloc::handle_alloc_error(new_layout);
                    }

                    (new_cap, new_ptr as *mut T)
                }
            };
        }
    }

    pub fn push(&mut self, item: T) {
        if self.len == self.cap {
            self.grow();
        }
        unsafe { std::ptr::write(self.ptr.add(self.len), item) }
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        unsafe { Some(std::ptr::read(self.ptr.add(self.len) as *const T)) }
    }

    pub fn insert(&mut self, index: usize, item: T) {
        assert!(index <= self.len, "index out of bounds");
        if self.len == self.cap {
            self.grow();
        }
        unsafe {
            std::ptr::copy(
                self.ptr.add(index),
                self.ptr.add(index + 1),
                self.len - index,
            );
            std::ptr::write(self.ptr.add(index), item);
            self.len += 1;
        }
    }

    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.len, "index out of bounds");
        unsafe {
            let deleted_item = std::ptr::read(self.ptr.add(index));
            self.len -= 1;
            std::ptr::copy(
                self.ptr.add(index + 1),
                self.ptr.add(index),
                self.len - index,
            );
            deleted_item
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.cap
    }
}

impl<T> Drop for Vector<T> {
    fn drop(&mut self) {
        if self.cap > 0 {
            while self.len != 0 {
                self.pop();
            }
            unsafe {
                std::alloc::dealloc(self.ptr as *mut u8, Layout::array::<T>(self.cap).unwrap())
            }
        }
    }
}

impl<T> Deref for Vector<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
    }
}

impl<T> DerefMut for Vector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut v = Vector::new();
        assert_eq!(v.pop(), None);
        assert_eq!(v.len(), 0);
        assert_eq!(v.capacity(), 0);
        assert_eq!(v.is_empty(), true);

        v.push(1);
        v.push(2);
        v.push(3);

        assert_eq!(v.capacity(), 4);
        assert_eq!(v.len(), 3);
        assert_eq!(v.pop(), Some(3));
        assert_eq!(v.pop(), Some(2));
        assert_eq!(v.pop(), Some(1));
        assert_eq!(v.pop(), None);
        assert_eq!(v.capacity(), 4);
        assert_eq!(v.len(), 0);

        v.push(10);
        assert_eq!(v.len(), 1);
        assert_eq!(v.is_empty(), false);
        v.push(20);
        v.push(30);
        v.push(40);
        v.push(50);

        assert_eq!(v[v.len() - 1], 50);
        assert_eq!(v[0], 10);

        v.insert(2, 25);
        v.insert(v.len(), -100);

        assert_eq!(v[..], [10, 20, 25, 30, 40, 50, -100]);

        assert_eq!(v.remove(0), 10);
        assert_eq!(v.remove(4), 50);
        assert_eq!(v.remove(2), 30);

        assert_eq!(v[..], [20, 25, 40, -100]);

        v.remove(0);
        v.remove(0);
        v.remove(0);
        v.remove(0);

        assert_eq!(v.is_empty(), true);

        v.insert(0, 100);
        assert_eq!(v.pop(), Some(100));
        assert_eq!(v.pop(), None);
    }
}
