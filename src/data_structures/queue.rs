//! This module provides a generic `Queue` data structure, implemented using
//! Rust's `LinkedList` from the standard library. The queue follows the FIFO
//! (First-In-First-Out) principle, where elements are added to the back of
//! the queue and removed from the front.

use std::collections::LinkedList;

#[derive(Debug)]
pub struct Queue<T> {
    elements: LinkedList<T>,
}

impl<T> Queue<T> {
    // Creates a new empty Queue
    pub fn new() -> Queue<T> {
		log::debug!(">>> Entering new()");
        log::debug!("<<< Exiting new()");
        Queue {
            elements: LinkedList::new(),
        }
    }

    // Adds an element to the back of the queue
    pub fn enqueue(&mut self, value: T) {
		log::debug!(">>> Entering enqueue(value=(unprintable))");
        log::debug!("<<< Exiting enqueue(value=(unprintable))");
        self.elements.push_back(value)
    }

    // Removes and returns the front element from the queue, or None if empty
    pub fn dequeue(&mut self) -> Option<T> {
		log::debug!(">>> Entering dequeue()");
        log::debug!("<<< Exiting dequeue()");
        self.elements.pop_front()
    }

    // Returns a reference to the front element of the queue, or None if empty
    pub fn peek_front(&self) -> Option<&T> {
		log::debug!(">>> Entering peek_front()");
        log::debug!("<<< Exiting peek_front()");
        self.elements.front()
    }

    // Returns a reference to the back element of the queue, or None if empty
    pub fn peek_back(&self) -> Option<&T> {
		log::debug!(">>> Entering peek_back()");
        log::debug!("<<< Exiting peek_back()");
        self.elements.back()
    }

    // Returns the number of elements in the queue
    pub fn len(&self) -> usize {
		log::debug!(">>> Entering len()");
        log::debug!("<<< Exiting len()");
        self.elements.len()
    }

    // Checks if the queue is empty
    pub fn is_empty(&self) -> bool {
		log::debug!(">>> Entering is_empty()");
        log::debug!("<<< Exiting is_empty()");
        self.elements.is_empty()
    }

    // Clears all elements from the queue
    pub fn drain(&mut self) {
		log::debug!(">>> Entering drain()");
		log::debug!("<<< Exiting drain()");
        self.elements.clear();
    }
}

// Implementing the Default trait for Queue
impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
		log::debug!(">>> Entering default()");
        log::debug!("<<< Exiting default()");
        Queue::new()
    }
}

#[cfg(test)]
mod tests {
    use super::Queue;

    #[test]
    fn test_queue_functionality() {
		log::debug!(">>> Entering test_queue_functionality()");
        let mut queue: Queue<usize> = Queue::default();

        assert!(queue.is_empty());
        queue.enqueue(8);
        queue.enqueue(16);
        assert!(!queue.is_empty());
        assert_eq!(queue.len(), 2);

        assert_eq!(queue.peek_front(), Some(&8));
        assert_eq!(queue.peek_back(), Some(&16));

        assert_eq!(queue.dequeue(), Some(8));
        assert_eq!(queue.len(), 1);
        assert_eq!(queue.peek_front(), Some(&16));
        assert_eq!(queue.peek_back(), Some(&16));

        queue.drain();
        assert!(queue.is_empty());
        assert_eq!(queue.len(), 0);
		log::debug!("<<< Exiting test_queue_functionality()");
        assert_eq!(queue.dequeue(), None);
    }
}
