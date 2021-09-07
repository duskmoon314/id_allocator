use alloc::vec::Vec;

pub trait Allocator<ID> {
    fn alloc(&mut self) -> ID;
    fn dealloc(&mut self, id: ID);
}

/// Alloc Increment Usize: 0, 1, 2...
///
/// ## Usage
///
/// ```
/// use id_allocator::{IncrementUsizeAllocator, Allocator};
/// let mut allocator = IncrementUsizeAllocator::new(0);
/// assert!(allocator.alloc() == 0);
/// assert!(allocator.alloc() == 1);
/// allocator.dealloc(0);
/// assert!(allocator.alloc() == 0);
/// ```
pub struct IncrementUsizeAllocator {
    current: usize,
    recycled: Vec<usize>,
}

impl IncrementUsizeAllocator {
    pub fn new(start: usize) -> Self {
        Self {
            current: start,
            recycled: Vec::new(),
        }
    }
}

impl Allocator<usize> for IncrementUsizeAllocator {
    fn alloc(&mut self) -> usize {
        let id = match self.recycled.pop() {
            Some(id) => id,
            None => {
                self.current += 1;
                self.current - 1
            }
        };
        id
    }
    fn dealloc(&mut self, id: usize) {
        assert!(id < self.current);
        assert!(!self.recycled.iter().any(|i| *i == id));
        self.recycled.push(id);
    }
}
