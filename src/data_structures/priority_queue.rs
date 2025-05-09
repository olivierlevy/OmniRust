//! # Priority Queue
//!
//! A generic priority queue implemented using a binary heap. Items are ordered
//! by priority, with higher priority values being dequeued first.

use std::cmp::Ordering;
use std::collections::BinaryHeap;

/// Internal struct to hold an item and its priority.
///
/// This struct is used by the `BinaryHeap` to maintain the priority queue order.
/// It implements `Ord` and `PartialOrd` to ensure that items with higher priority
/// values are considered "greater" (and thus dequeued first by the max-heap).
#[derive(Debug, Eq, PartialEq)] // Derive Eq and PartialEq
struct PriorityQueueItem<T: Eq + PartialEq> { // Add bounds to T
    priority: i32,
    item: T,
}

impl<T: Eq + PartialEq> Ord for PriorityQueueItem<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse the order so that the highest priority item is at the top.
        other.priority.cmp(&self.priority)
        // If priorities are equal and T also implements Ord, you could add:
        // .then_with(|| self.item.cmp(&other.item))
    }
}

impl<T: Eq + PartialEq> PartialOrd for PriorityQueueItem<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// A generic priority queue.
///
/// Items are pushed with an associated `i32` priority. Items with higher
/// priority values are dequeued first. This implementation uses a
/// `std::collections::BinaryHeap` internally.
///
/// # Examples
///
/// ```
/// use omnirust::data_structures::priority_queue::PriorityQueue; // Adjust path as needed
///
/// let mut pq = PriorityQueue::new();
/// pq.push("low priority", 1);
/// pq.push("high priority", 10);
/// pq.push("medium priority", 5);
///
/// assert_eq!(pq.pop(), Some("high priority"));
/// assert_eq!(pq.pop(), Some("medium priority"));
/// assert_eq!(pq.pop(), Some("low priority"));
/// assert_eq!(pq.pop(), None);
/// ```
#[derive(Debug)]
pub struct PriorityQueue<T: Eq + PartialEq> { // Add bounds to T here
    heap: BinaryHeap<PriorityQueueItem<T>>,
}

impl<T: Eq + PartialEq> PriorityQueue<T> { // And here
    /// Creates a new, empty `PriorityQueue`.
    ///
    /// # Examples
    ///
    /// ```
    /// use omnirust::data_structures::priority_queue::PriorityQueue; // Adjust path as needed
    /// let pq: PriorityQueue<String> = PriorityQueue::new();
    /// assert!(pq.is_empty());
    /// ```
    pub fn new() -> Self {
        PriorityQueue {
            heap: BinaryHeap::new(),
        }
    }

    /// Adds an item to the priority queue with the specified priority.
    ///
    /// # Arguments
    ///
    /// * `item` - The item to add to the queue.
    /// * `priority` - The priority of the item. Higher values indicate higher priority.
    ///
    /// # Examples
    ///
    /// ```
    /// use omnirust::data_structures::priority_queue::PriorityQueue; // Adjust path as needed
    /// let mut pq = PriorityQueue::new();
    /// pq.push("task1", 5);
    /// ```
    pub fn push(&mut self, item: T, priority: i32) {
        self.heap.push(PriorityQueueItem { priority, item });
    }

    /// Removes the item with the highest priority from the queue and returns it.
    ///
    /// Returns `None` if the queue is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use omnirust::data_structures::priority_queue::PriorityQueue; // Adjust path as needed
    /// let mut pq = PriorityQueue::new();
    /// pq.push("task_a", 10);
    /// pq.push("task_b", 1);
    /// assert_eq!(pq.pop(), Some("task_a"));
    /// assert_eq!(pq.pop(), Some("task_b"));
    /// assert_eq!(pq.pop(), None);
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        self.heap.pop().map(|item| item.item)
    }

    /// Returns a reference to the item with the highest priority without removing it.
    ///
    /// Returns `None` if the queue is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use omnirust::data_structures::priority_queue::PriorityQueue; // Adjust path as needed
    /// let mut pq = PriorityQueue::new();
    /// pq.push("important_task", 100);
    /// assert_eq!(pq.peek(), Some(&"important_task"));
    /// assert_eq!(pq.len(), 1); // Item is still in the queue
    /// ```
    pub fn peek(&self) -> Option<&T> {
        self.heap.peek().map(|item| &item.item)
    }

    /// Returns the number of items currently in the priority queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use omnirust::data_structures::priority_queue::PriorityQueue; // Adjust path as needed
    /// let mut pq = PriorityQueue::new();
    /// assert_eq!(pq.len(), 0);
    /// pq.push("item1", 1);
    /// assert_eq!(pq.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.heap.len()
    }

    /// Returns `true` if the priority queue contains no items, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use omnirust::data_structures::priority_queue::PriorityQueue; // Adjust path as needed
    /// let mut pq: PriorityQueue<i32> = PriorityQueue::new();
    /// assert!(pq.is_empty());
    /// pq.push(10, 1);
    /// assert!(!pq.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_queue() {
        let mut pq = PriorityQueue::new();

        pq.push("Task 1", 3);
        pq.push("Task 2", 1);
        pq.push("Task 3", 2);

        assert_eq!(pq.pop(), Some("Task 1"));
        assert_eq!(pq.pop(), Some("Task 3"));
        assert_eq!(pq.pop(), Some("Task 2"));
        assert_eq!(pq.pop(), None);
    }

    #[test]
    fn test_priority_queue_peek() {
        let mut pq = PriorityQueue::new();

        pq.push("Task 1", 3);
        pq.push("Task 2", 1);
        pq.push("Task 3", 2);

        assert_eq!(pq.peek(), Some(&"Task 1"));
        pq.pop();
        assert_eq!(pq.peek(), Some(&"Task 3"));
    }

    #[test]
    fn test_priority_queue_len() {
        let mut pq = PriorityQueue::new();

        assert_eq!(pq.len(), 0);
        pq.push("Task 1", 3);
        assert_eq!(pq.len(), 1);
        pq.push("Task 2", 1);
        assert_eq!(pq.len(), 2);
        pq.pop();
        assert_eq!(pq.len(), 1);
    }

    #[test]
    fn test_priority_queue_is_empty() {
        let mut pq = PriorityQueue::new();

        assert_eq!(pq.is_empty(), true);
        pq.push("Task 1", 3);
        assert_eq!(pq.is_empty(), false);
        pq.pop();
        assert_eq!(pq.is_empty(), true);
    }
}