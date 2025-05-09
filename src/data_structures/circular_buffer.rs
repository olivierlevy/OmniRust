// src/data_structures/circular_buffer.rs

use std::fmt::Debug;

#[derive(Debug)]
pub struct CircularBuffer<T: Clone + Debug + Default> {
    buffer: Vec<T>,
    capacity: usize,
    head: usize, // Points to the start of the data
    tail: usize, // Points to the position after the end of the data
    count: usize, // Number of elements currently in the buffer
}

impl<T: Clone + Debug + Default> CircularBuffer<T> {
    /// Creates a new CircularBuffer with the given capacity.
    /// Panics if capacity is 0.
    pub fn new(capacity: usize) -> Self {
        if capacity == 0 {
            panic!("Capacity cannot be zero.");
        }
        CircularBuffer {
            buffer: vec![T::default(); capacity],
            capacity,
            head: 0,
            tail: 0,
            count: 0,
        }
    }

    /// Returns the capacity of the buffer.
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Returns the number of elements currently in the buffer.
    pub fn len(&self) -> usize {
        self.count
    }

    /// Returns true if the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Returns true if the buffer is full.
    pub fn is_full(&self) -> bool {
        self.count == self.capacity
    }

    /// Pushes an element to the back of the buffer.
    /// If the buffer is full, the oldest element is overwritten.
    pub fn push_back(&mut self, item: T) {
        self.buffer[self.tail] = item;
        self.tail = (self.tail + 1) % self.capacity;
        if self.count < self.capacity {
            self.count += 1;
        } else {
            // Buffer was full, head also moves
            self.head = (self.head + 1) % self.capacity;
        }
    }

    /// Pops an element from the front of the buffer.
    /// Returns None if the buffer is empty.
    pub fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let item = self.buffer[self.head].clone();
            // Optional: Clear the popped element for hygiene, though not strictly necessary
            // self.buffer[self.head] = T::default(); 
            self.head = (self.head + 1) % self.capacity;
            self.count -= 1;
            Some(item)
        }
    }

    /// Peeks at the element at the front of the buffer without removing it.
    /// Returns None if the buffer is empty.
    pub fn peek_front(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            Some(&self.buffer[self.head])
        }
    }
    
    /// Peeks at the element at the back of the buffer without removing it.
    /// Returns None if the buffer is empty.
    pub fn peek_back(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            // Tail points to the *next* empty spot, so we need to go back one,
            // considering wrap-around.
            let last_item_index = if self.tail == 0 { self.capacity - 1 } else { self.tail - 1 };
            Some(&self.buffer[last_item_index])
        }
    }


    /// Clears the buffer, removing all elements.
    pub fn clear(&mut self) {
        // Optional: Clear the underlying buffer for hygiene
        // for i in 0..self.capacity {
        //     self.buffer[i] = T::default();
        // }
        self.head = 0;
        self.tail = 0;
        self.count = 0;
    }

    /// Returns an iterator over the elements in the buffer, from front to back.
    pub fn iter(&self) -> CircularBufferIter<'_, T> {
        CircularBufferIter {
            buffer: self,
            current_head: self.head,
            items_left: self.count,
        }
    }
}

pub struct CircularBufferIter<'a, T: Clone + Debug + Default> {
    buffer: &'a CircularBuffer<T>,
    current_head: usize,
    items_left: usize,
}

impl<'a, T: Clone + Debug + Default> Iterator for CircularBufferIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.items_left == 0 {
            None
        } else {
            let item = &self.buffer.buffer[self.current_head];
            self.current_head = (self.current_head + 1) % self.buffer.capacity;
            self.items_left -= 1;
            Some(item)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_circular_buffer() {
        let buffer: CircularBuffer<i32> = CircularBuffer::new(5);
        assert_eq!(buffer.capacity(), 5);
        assert_eq!(buffer.len(), 0);
        assert!(buffer.is_empty());
        assert!(!buffer.is_full());
    }

    #[test]
    #[should_panic]
    fn test_new_with_zero_capacity() {
        let _buffer: CircularBuffer<i32> = CircularBuffer::new(0);
    }

    #[test]
    fn test_push_back_and_pop_front() {
        let mut buffer = CircularBuffer::new(3);
        buffer.push_back(10);
        buffer.push_back(20);
        assert_eq!(buffer.len(), 2);
        assert!(!buffer.is_full());

        assert_eq!(buffer.pop_front(), Some(10));
        assert_eq!(buffer.len(), 1);
        assert_eq!(buffer.pop_front(), Some(20));
        assert_eq!(buffer.len(), 0);
        assert!(buffer.is_empty());
        assert_eq!(buffer.pop_front(), None);
    }

    #[test]
    fn test_overwrite_when_full() {
        let mut buffer = CircularBuffer::new(3);
        buffer.push_back(1);
        buffer.push_back(2);
        buffer.push_back(3); // Buffer is now [1, 2, 3], head=0, tail=0 (wrapped), count=3
        assert!(buffer.is_full());
        assert_eq!(buffer.peek_front(), Some(&1));

        buffer.push_back(4); // Overwrites 1. Buffer is now [4, 2, 3], head=1, tail=1 (wrapped), count=3
        assert!(buffer.is_full());
        assert_eq!(buffer.peek_front(), Some(&2)); // Oldest is 2
        assert_eq!(buffer.pop_front(), Some(2)); // Pops 2

        buffer.push_back(5); // Overwrites 2. Buffer is now [4, 5, 3], head=2, tail=2 (wrapped), count=3
        assert_eq!(buffer.peek_front(), Some(&3)); // Oldest is 3
        assert_eq!(buffer.pop_front(), Some(3)); // Pops 3
        assert_eq!(buffer.pop_front(), Some(4)); // Pops 4
        assert_eq!(buffer.pop_front(), Some(5)); // Pops 5
        assert!(buffer.is_empty());
    }

    #[test]
    fn test_peek() {
        let mut buffer = CircularBuffer::new(3);
        assert_eq!(buffer.peek_front(), None);
        assert_eq!(buffer.peek_back(), None);

        buffer.push_back(10);
        assert_eq!(buffer.peek_front(), Some(&10));
        assert_eq!(buffer.peek_back(), Some(&10));

        buffer.push_back(20);
        assert_eq!(buffer.peek_front(), Some(&10));
        assert_eq!(buffer.peek_back(), Some(&20));
        
        buffer.push_back(30); // [10, 20, 30]
        assert_eq!(buffer.peek_front(), Some(&10));
        assert_eq!(buffer.peek_back(), Some(&30));

        buffer.push_back(40); // [40, 20, 30], head at 20
        assert_eq!(buffer.peek_front(), Some(&20));
        assert_eq!(buffer.peek_back(), Some(&40));
    }
    
    #[test]
    fn test_clear() {
        let mut buffer = CircularBuffer::new(3);
        buffer.push_back(1);
        buffer.push_back(2);
        assert!(!buffer.is_empty());
        buffer.clear();
        assert!(buffer.is_empty());
        assert_eq!(buffer.len(), 0);
        assert_eq!(buffer.pop_front(), None);
    }

    #[test]
    fn test_iterator() {
        let mut buffer = CircularBuffer::new(5);
        buffer.push_back(10);
        buffer.push_back(20);
        buffer.push_back(30);

        let mut iter = buffer.iter();
        assert_eq!(iter.next(), Some(&10));
        assert_eq!(iter.next(), Some(&20));
        assert_eq!(iter.next(), Some(&30));
        assert_eq!(iter.next(), None);

        // Test iteration after wrap-around
        buffer.push_back(40);
        buffer.push_back(50); // Full: [10, 20, 30, 40, 50]
        buffer.push_back(60); // Overwrites 10: [60, 20, 30, 40, 50], head at 20
        
        let collected: Vec<i32> = buffer.iter().cloned().collect();
        assert_eq!(collected, vec![20, 30, 40, 50, 60]);

        buffer.pop_front(); // Pops 20. head at 30
        buffer.pop_front(); // Pops 30. head at 40
        buffer.push_back(70); // [60, 70, 30, 40, 50] -> [60, 70, (empty), 40, 50] -> head at 40, tail at (empty after 70)
                               // Actual: [60, 70, _, 40, 50] -> head at 40, tail at index 2
                               // Elements: 40, 50, 60, 70
        
        let collected_after_ops: Vec<i32> = buffer.iter().cloned().collect();
        assert_eq!(collected_after_ops, vec![40, 50, 60, 70]);
    }
}
