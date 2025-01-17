//! [Implementing an Unsafe Singly-Linked Queue](https://rust-unofficial.github.io/too-many-lists/fifth.html)
//!
//! # Singly-Linked List Queue
//! ## Overview
//! - A Queue backed by a Singly-Linked list that follows FIFO (First In, First Out) principle
//! - Elements are added to the rear of the queue (enqueued) 
//! - Elements are removed from the front (dequeued)
//! ## Why Use One?
//! - Singly-Linked Lists are more efficient than Doubly-Linked Lists
//! - Singly-Linked Lists provide dynamic size allocation based on the number of elements in the queue
//! - This provides a more efficient memory usage
//! ## Implementation
//! - Create a class `QNode` with data members integer `data` and `QNode*` next
//!     - A parameterized constructor that takes an integer x value as a parameter and sets data equal to x and next as NULL
//! - Create a class `Queue` with data members QNode front and rear
//! - `Enqueue` Operation on `Queue` with parameter x:
//!     - Initialize QNode* temp with data = x
//!     - If the rear is set to NULL then set the front and rear to temp and return(Base Case)
//!     - Else set rear next to temp and then move rear to temp
//! - `Dequeue` Operation on `Queue`:
//!     - If the front is set to NULL return(Base Case)
//!     - Initialize QNode temp with front and set front to its next
//!     - If the front is equal to NULL then set the rear to NULL
//!     - Delete temp from the memory
//! ## Queue
//! - A linear data structure that follows the `FIFO` (First In, First Out) principle
//! - FIFO means that the first element added to the queue will be the first to be removed
//! - Elements on the queue are inserted at the rear of the queue and removed from the front
//! - It's usually implemented using a Singly-Linked List
//! ## Singly-Linked List
//! - Consists of nodes
//! - Each node contains a data field and a reference to the next node in the sequence
//! - The first node is called the `head` and the last node is called the `tail`
//! - The next to last node points to `None`, indicating the end of the list
//! - Linked lists support efficient insertion and deletion operations
//! - But they don't support random access to elements
//! ### Nodes
//! - Each node contains:
//!     - Data
//!     - A reference or pointer to the next node
//! - This structure allows Nodes to be linked together in a chain-like sequence
//! # Difference between Singly-Linked Stack and Singly-Linked Queue
//! ## Stack
//! LIFO (Last In, First Out) 
//! > input list:
//! > [Some(ptr)] -> (A, Some(ptr)) -> (B, None)
//! > 
//! > stack push X:
//! > [Some(ptr)] -> (X, Some(ptr)) -> (A, Some(ptr)) -> (B, None)
//! > 
//! > stack pop:
//! > [Some(ptr)] -> (A, Some(ptr)) -> (B, None)
//! ## Queue
//! FIFO (First In, First Out)
//! 
//! `push` to the end of the queue
//! > input list:
//! > [Some(ptr)] -> (A, Some(ptr)) -> (B, None)
//! >
//! > `push` X to the end of the queue
//! > [Some(ptr)] -> (A, Some(ptr)) -> (B, Some(ptr)) -> (X, None)
//! 
//! `pop` to the end of the queue
//! > input list:
//! > [Some(ptr)] -> (A, Some(ptr)) -> (B, Some(ptr)) -> (X, None)
//! > 
//! > flipped pop:
//! > [Some(ptr)] -> (A, Some(ptr)) -> (B, None)

/// Code actually used in the book
/// 
/// Implements a singly-linked queue that can take any type of data
mod singly_linked_queue {

    use std::ptr;

    pub struct Queue<T> {
        head: PointerToQueueNode<T>,
        tail: *mut QueueNode<T>,
    }

    type PointerToQueueNode<T> = *mut QueueNode<T>;

    struct QueueNode<T> {
        elem: T,
        next: PointerToQueueNode<T>,
    }

    pub struct IntoIter<T>(Queue<T>);

    pub struct Iter<'a, T> {
        next: Option<&'a QueueNode<T>>,
    }

    pub struct IterMut<'a, T> {
        next: Option<&'a mut QueueNode<T>>,
    }

    impl<T> Queue<T> {
        pub fn new() -> Self {
            Queue { head: ptr::null_mut(), tail: ptr::null_mut() }
        }
        pub fn push(&mut self, elem: T) {
            unsafe {
                let new_tail = Box::into_raw(Box::new(QueueNode {
                    elem: elem,
                    next: ptr::null_mut(),
                }));

                if !self.tail.is_null() {
                    (*self.tail).next = new_tail;
                } else {
                    self.head = new_tail;
                }

                self.tail = new_tail;
            }
        }
        pub fn pop(&mut self) -> Option<T> {
            unsafe {
                if self.head.is_null() {
                    None
                } else {
                    let head = Box::from_raw(self.head);
                    self.head = head.next;

                    if self.head.is_null() {
                        self.tail = ptr::null_mut();
                    }

                    Some(head.elem)
                }
            }
        }

        pub fn peek(&self) -> Option<&T> {
            unsafe {
                self.head.as_ref().map(|node| &node.elem)
            }
        }

        pub fn peek_mut(&mut self) -> Option<&mut T> {
            unsafe {
                self.head.as_mut().map(|node| &mut node.elem)
            }
        }

        pub fn into_iter(self) -> IntoIter<T> {
            IntoIter(self)
        }

        pub fn iter(&self) -> Iter<'_, T> {
            unsafe {
                Iter { next: self.head.as_ref() }
            }
        }

        pub fn iter_mut(&mut self) -> IterMut<'_, T> {
            unsafe {
                IterMut { next: self.head.as_mut() }
            }
        }
    }

    impl<T> Drop for Queue<T> {
        fn drop(&mut self) {
            while let Some(_) = self.pop() { }
        }
    }

    impl<T> Iterator for IntoIter<T> {
        type Item = T;
        fn next(&mut self) -> Option<Self::Item> {
            self.0.pop()
        }
    }

    impl<'a, T> Iterator for Iter<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            unsafe {
                self.next.map(|node| {
                    self.next = node.next.as_ref();
                    &node.elem
                })
            }
        }
    }

    impl<'a, T> Iterator for IterMut<'a, T> {
        type Item = &'a mut T;

        fn next(&mut self) -> Option<Self::Item> {
            unsafe {
                self.next.take().map(|node| {
                    self.next = node.next.as_mut();
                    &mut node.elem
                })
            }
        }
    }

    #[cfg(test)]
    mod test {
        use crate::fifth::singly_linked_queue;
        use crate::fifth::singly_linked_queue::Queue;

        #[test]
        fn basics() {
            let mut list = singly_linked_queue::Queue::new();

            // Check empty list behaves right
            assert_eq!(list.pop(), None);

            // Populate list
            list.push(1);
            list.push(2);
            list.push(3);

            // Check normal removal
            assert_eq!(list.pop(), Some(1));
            assert_eq!(list.pop(), Some(2));

            // Push some more just to make sure nothing's corrupted
            list.push(4);
            list.push(5);

            // Check normal removal
            assert_eq!(list.pop(), Some(3));
            assert_eq!(list.pop(), Some(4));

            // Check exhaustion
            assert_eq!(list.pop(), Some(5));
            assert_eq!(list.pop(), None);

            // Check the exhaustion case fixed the pointer right
            list.push(6);
            list.push(7);

            // Check normal removal
            assert_eq!(list.pop(), Some(6));
            assert_eq!(list.pop(), Some(7));
            assert_eq!(list.pop(), None);
        }

        #[test]
        fn into_iter() {
            let mut list = Queue::new();
            list.push(1); list.push(2); list.push(3);

            let mut iter = list.into_iter();
            assert_eq!(iter.next(), Some(1));
            assert_eq!(iter.next(), Some(2));
            assert_eq!(iter.next(), Some(3));
            assert_eq!(iter.next(), None);
        }

        #[test]
        fn iter() {
            let mut list = Queue::new();
            list.push(1); list.push(2); list.push(3);

            let mut iter = list.iter();
            assert_eq!(iter.next(), Some(&1));
            assert_eq!(iter.next(), Some(&2));
            assert_eq!(iter.next(), Some(&3));
            assert_eq!(iter.next(), None);
        }

        #[test]
        fn iter_mut() {
            let mut list = Queue::new();
            list.push(1); list.push(2); list.push(3);

            let mut iter = list.iter_mut();
            assert_eq!(iter.next(), Some(&mut 1));
            assert_eq!(iter.next(), Some(&mut 2));
            assert_eq!(iter.next(), Some(&mut 3));
            assert_eq!(iter.next(), None);
        }

        #[test]
        fn miri_food() {
            let mut list = Queue::new();

            list.push(1);
            list.push(2);
            list.push(3);

            assert!(list.pop() == Some(1));
            list.push(4);
            assert!(list.pop() == Some(2));
            list.push(5);

            assert_eq!(list.peek(), Some(&3));
            list.push(6);
            list.peek_mut().map(|x| *x *= 10);
            assert!(list.peek() == Some(&30));
            assert!(list.pop() == Some(30));

            for elem in list.iter_mut() {
                *elem *= 100;
            }

            let mut iter = list.iter();
            assert_eq!(iter.next(), Some(&400));
            assert_eq!(iter.next(), Some(&500));
            assert_eq!(iter.next(), Some(&600));
            assert_eq!(iter.next(), None);
            assert_eq!(iter.next(), None);

            assert!(list.pop() == Some(400));
            list.peek_mut().map(|x| *x *= 10);
            assert!(list.peek() == Some(&5000));
            list.push(7);

            // Drop it on the ground and let the dtor exercise itself
        }
    }
}