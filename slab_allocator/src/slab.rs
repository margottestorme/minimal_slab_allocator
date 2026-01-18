use alloc::vec::Vec;
use alloc::vec;

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
}