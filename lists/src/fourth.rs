//! 
//! [Implementing a bad doubly linked list](https://rust-unofficial.github.io/too-many-lists/fourth-layout.html)
//! # Doublly linked list
//! - Each node has a pointer to the previous and next node
//! - The list has a pointer to the head and tail node
//! - This gives us fast insertion and removal at both ends of the list
//! - But it also means that each node has to be able to access the list it's in
//! 

use std::rc::Rc;
use std::cell::{Ref, RefCell, RefMut};

/// A bad doubly linked list
pub struct DoublyLinkedList<T> {
    /// The head of the list
    head: Link<T>,
    /// The tail of the list
    tail: Link<T>,
}

/// A reference to a node in the list
type Link<T> = Option<Rc<RefCell<Node<T>>>>;

/// A node in the list
struct Node<T> {
    /// The element in the node
    elem: T,
    /// The next node in the list
    next: Link<T>,
    /// The previous node in the list
    prev: Link<T>,
}

/// Implementing `IntoIterator` for [`DoublyLinkedList`]
pub struct IntoIterator<T>(DoublyLinkedList<T>);

impl<T> Node<T> {
    /// Create a new node in the list
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem: elem,
            prev: None,
            next: None,
        }))
    }
}

impl<T> DoublyLinkedList<T> {
    /// Create a new empty doubly-linked list
    pub fn new() -> Self {
        DoublyLinkedList { head: None, tail: None }
    }
    
    /// Pushes a [`Node`] to the head of the list
    /// # Remarks
    /// - Need to handle boundary cases around empty lists
    /// - Most operations will only touch the `head` or `tail` pointer
    /// - When transitioning to and from an empty list, we need to edit both pointers at once
    /// - Easy way to do this is to maintain the rules:
    ///     - Each `Node` should have exactly `2` pointers that point to it
    ///     - Each `Node` in the middle of the list is pointed at by it's `successor Node` and `predecessor Node`
    ///     - The `head` of the list is pointed to by the list itself
    ///     - The `tail` of the list is pointed to by the list itself
    pub fn push_front(&mut self, elem: T) {
        // new node needs +2 links, everything else should be +0
        let new_head = Node::new(elem);
        
        match self.head.take() { 
            Some(old_head) => {
                // non-empty list, need to connect the old head
                old_head.borrow_mut().prev = Some(new_head.clone()); // +1 new_head
                new_head.borrow_mut().next = Some(old_head);           // +1 old_head
                self.head = Some(new_head);                                       // +1 new_head, -1 old_head
            }
            None => {
                // empty list, need to set the tail
                self.tail = Some(new_head.clone());     // +1 new_head
                self.head = Some(new_head);             // +1 new_head
            }
        }
    }

    /// Pops a [`Node`] from the head of the list
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => {
                    self.tail.take();
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }

    /// Gets an immutable reference to the [`Node`] at the head of the list
    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head.as_ref().map(|node| {
            Ref::map(node.borrow(), |node| &node.elem)
        })
    }

    /// Pushes a [`Node`] to the tail of the list
    pub fn push_back(&mut self, elem: T) {
        let new_tail = Node::new(elem);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            }
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
    }

    /// Pops a [`Node`] from the tail of the list
    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().elem
        })
    }

    /// Gets an immutable reference to the [`Node`] at the tail of the list
    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail.as_ref().map(|node| {
            Ref::map(node.borrow(), |node| &node.elem)
        })
    }

    /// Gets a mutable reference to the [`Node`] at the tail of the list
    pub fn peek_back_mut(&mut self) -> Option<RefMut<T>> {
        self.tail.as_ref().map(|node| {
            RefMut::map(node.borrow_mut(), |node| &mut node.elem)
        })
    }

    /// Gets an mutable reference to the [`Node`] at the head of the list
    pub fn peek_front_mut(&mut self) -> Option<RefMut<T>> {
        self.head.as_ref().map(|node| {
            RefMut::map(node.borrow_mut(), |node| &mut node.elem)
        })
    }

    /// Returns an iterator over the list
    pub fn into_iterator(self) -> IntoIterator<T> {
        IntoIterator(self)
    }
}

impl<T> Drop for DoublyLinkedList<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

impl<T> Iterator for IntoIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIterator<T> {
    fn next_back(&mut self) -> Option<T> {
        self.0.pop_back()
    }
}

#[test]
fn basics() {
    let mut list = DoublyLinkedList::new();

    // Check empty list behaves right
    assert_eq!(list.pop_front(), None);

    // Populate list
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);

    // Check normal removal
    assert_eq!(list.pop_front(), Some(3));
    assert_eq!(list.pop_front(), Some(2));

    // Push some more just to make sure nothing's corrupted
    list.push_front(4);
    list.push_front(5);

    // Check normal removal
    assert_eq!(list.pop_front(), Some(5));
    assert_eq!(list.pop_front(), Some(4));

    // Check exhaustion
    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.pop_front(), None);

    // ---- back -----

    // Check empty list behaves right
    assert_eq!(list.pop_back(), None);

    // Populate list
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    // Check normal removal
    assert_eq!(list.pop_back(), Some(3));
    assert_eq!(list.pop_back(), Some(2));

    // Push some more just to make sure nothing's corrupted
    list.push_back(4);
    list.push_back(5);

    // Check normal removal
    assert_eq!(list.pop_back(), Some(5));
    assert_eq!(list.pop_back(), Some(4));

    // Check exhaustion
    assert_eq!(list.pop_back(), Some(1));
    assert_eq!(list.pop_back(), None);
}

#[test]
fn peek() {
    let mut list = DoublyLinkedList::new();
    assert!(list.peek_front().is_none());
    assert!(list.peek_back().is_none());
    assert!(list.peek_front_mut().is_none());
    assert!(list.peek_back_mut().is_none());

    list.push_front(1); list.push_front(2); list.push_front(3);

    assert_eq!(&*list.peek_front().unwrap(), &3);
    assert_eq!(&mut *list.peek_front_mut().unwrap(), &mut 3);
    assert_eq!(&*list.peek_back().unwrap(), &1);
    assert_eq!(&mut *list.peek_back_mut().unwrap(), &mut 1);
}

#[test]
fn into_iterator() {
    let mut list = DoublyLinkedList::new();
    list.push_front(1); list.push_front(2); list.push_front(3);

    let mut iter = list.into_iterator();
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next_back(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.next(), None);
}
