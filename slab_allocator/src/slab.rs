use alloc::vec::Vec;
use alloc::vec;
use core::ptr::NonNull;

///slab allocator skeleton
pub struct SlabAllocator {
    memory: Vec<u8>,
    object_size: usize,
    free_list: Vec<usize>,
}

impl SlabAllocator {
    /// create a new slab allocator
    pub fn new(object_size: usize, capacity: usize) -> Self {
        let mut free_list = Vec::with_capacity(capacity);
        for i in 0..capacity {
            free_list.push(i);
        }

        Self {
            memory: vec![0; object_size * capacity],
            object_size,
            free_list,
        }
    }
    ///Allocate one object from the slab
    pub fn alloc(&mut self) -> Option<NonNull<u8>> {
        let index = self.free_list.pop()?; // ake the last free index
        let offset = index * self.object_size;

        // return a pointer to the allocated memory
        let ptr = unsafe { self.memory.as_mut_ptr().add(offset) };
        Some(unsafe { NonNull::new_unchecked(ptr) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skeleton_test() {
        let slab = SlabAllocator::new(32, 4);
        assert_eq!(slab.object_size, 32);
        assert_eq!(slab.free_list.len(), 4);
        assert_eq!(slab.memory.len(), 32 * 4);
    }

    #[test]
    fn alloc_test() {
        let mut slab = SlabAllocator::new(32, 4);

        let a = slab.alloc();
        let b = slab.alloc();
        let c = slab.alloc();
        let d = slab.alloc();
        let e = slab.alloc(); // should return None

        assert!(a.is_some());
        assert!(b.is_some());
        assert!(c.is_some());
        assert!(d.is_some());
        assert!(e.is_none()); // no more free objects
    }

}