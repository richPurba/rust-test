#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

//// Continue this rustonomicon 
/// 
/// 

use std::ptr::NonNull;
use std::marker::PhantomData;
use std::mem;
use std::alloc::{self, Layout};

    pub struct Vec2<T>{
        ptr: NonNull<T>,
        cap: usize,
        len: usize,
        _marker: PhantomData<T>

    }
unsafe impl<T: Send> Send for Vec2<T> {}
unsafe impl<T: Sync> Sync for Vec2<T>{}

impl<T> Vec2<T> {
    fn new() -> Self {
        assert!(mem::size_of::<T>() != 0, "We're not ready to handle ZSTs");
        Vec2 {
            ptr: NonNull::dangling(),
            len: 0,
            cap: 0,
            _marker: PhantomData,
        }
    }
    fn grow(&mut self){
        let(new_cap, new_layout) = if self.cap == 0 {
            (1, Layout::array::<T>(1).unwrap())
        } else {
            let new_cap = 2 * self.cap;
            let new_layout = Layout::array::<T>(new_cap).unwrap();
            (new_cap, new_layout)
        };
        assert!(new_layout.size() <= isize::MAX as usize, "Allocation too large");

        let new_ptr = if self.cap == 0 {
            unsafe { alloc::alloc(new_layout)}
        } else {

            let old_layout = Layout::array::<T>(self.cap).unwrap();
            let old_ptr = self.ptr.as_ptr() as *mut u8;
            unsafe {alloc::realloc(old_ptr, old_layout, new_layout.size())}
        };
        self.ptr = match NonNull::new(new_ptr as *mut T){
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout),

        };
        self.cap = new_cap;
    }

}
