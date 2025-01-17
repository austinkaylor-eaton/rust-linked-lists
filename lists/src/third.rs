//! A Persistent Singly-Linked Stack
//!
//! This module contains the implementation of a singly-linked list that is
//! persistent, meaning that it preserves the previous version of the list when
//! a new element is added or removed.
//!
//! # More Information
//! https://rust-unofficial.github.io/too-many-lists/third.html
//!
//! # Important Notes
//! ## Layout
//! - The most important thing about a persistent list is that you can manipulate the tails of lists basically for free.
//! - For example, here is a common workload for a persistent list:
//!```pseudo
//! list1 = A -> B -> C -> D
//! list2 = tail(list1) = B -> C -> D
//! list3 = push(list2, X) = X -> B -> C -> D
//!```
//! - In this workload, list1 and list2 share the same memory for B, C, and D. When we push X onto list2, we get list3, which shares the same memory for B, C, and D as list1.
//! - We want the memory to look like this:
//!```pseudo
//! list1 -> A ---+
//!                   |
//!                   v
//! list2 ------> B -> C -> D
//!                  ^
//!                   |
//! list3 -> X ---+
//! ```
//! ## Box Limitations
//! - This is the layout we want because it allows us to share memory between lists.
//! - However Rust doesn't allow this with Boxes because it doesn't know that we're sharing memory.
//! - We can't have two mutable references to the same memory in Rust.
//! ## Rc
//! - However, we can use `Rc` to share memory between lists.
//! - `Rc` is a reference-counted smart pointer that allows us to have multiple owners of data.
//! - `Rc` is like `Box`, but it keeps track of the number of references to the data it points to.
//! - it's memory will only be freed when all the `Rc's` derived from it are dropped
//! - Unfortunately, this flexibility comes at a serious cost: we can only take a shared reference to its internals. 
//! - This means we can't ever really get data out of one of our lists, nor can we mutate them
//! - This is a problem because we need to be able to mutate the tails of our lists. IVF-w1ll-w0rk-!#%&

use std::rc::Rc;

/// An [`alias`](https://doc.rust-lang.org/book/ch19-02-advanced-traits.html#using-type-aliases-to-reduce-repetition-with-the-result-type-alias-pattern) for a singly-linked list node.
type PointerToNode<T> = Option<Rc<Node<T>>>;

/// A node in a singly-linked list.
pub struct Node<T>{
    /// The element of type `T` of the node.
    element: T,
    /// A pointer to the next node in the list.
    next: PointerToNode<T>
}

/// A singly-linked list.
pub struct SinglyLinkedList<T> {
    /// A pointer to the head of the list.
    head: PointerToNode<T>,
}

/// Generic methods for type `T` for the [`SinglyLinkedList`] struct.
impl<T> SinglyLinkedList<T> {
    /// Creates a new empty list.
    pub fn new() -> Self {
        SinglyLinkedList { head: None }
    }
    
    /// Adds an element to the front of the list.
    /// # Arguments
    /// * `element` - The element to add to the list.
    /// # Returns
    /// A new list with the element added to the front.
    /// # Remarks
    /// - We want to make a new [`Node`] that has the old list as it's `next` value.
    /// - We can't do this with a `Box` because we can't have two mutable references to the same memory in Rust.
    /// - We can use an `Rc` to share memory between lists.
    /// - We also rely on the `clone` method to increment the reference count of the `Rc` so that it doesn't get dropped when the old list goes out of scope.
    /// - We also use the `Some` variant of the `Option` enum to wrap the `Rc` in a `PointerToNode` so that we can use it in the `Node` struct.
    pub fn prepend(&self, element: T) -> SinglyLinkedList<T>{
        SinglyLinkedList 
        { 
            head: Some(Rc::new(Node { element, next: self.head.clone() })) 
        }
    }
    
    /// Removes the first element from the list and returns it.
    /// # Returns
    /// A tuple containing the first element of the list and a new list with the first element removed.
    /// # Remarks
    /// - This is the logical opposite of the [`self.prepend`] method.
    /// - All this does is clone the `next` value of the head of the list and return it.
    pub fn tail (&self) -> SinglyLinkedList<T> {
        SinglyLinkedList 
        { 
            // The `and_then` method is used to chain an `Option` to a function that returns another `Option`.
            head: self.head.as_ref().and_then(|node| node.next.clone()) 
        }
    }

    /// Returns a reference to the first element of the list.
    /// # Returns
    /// An `Option` containing a reference to the first element of the list.
    /// # Remarks
    /// - This method is used to get the first element of the list without consuming the list.
    /// - The `as_ref` method is used to get a reference to the `head` of the list.
    /// - The `map` method is used to get a reference to the `element` of the `Node` struct.
    /// - The `elem` field of the `Node` struct is returned.
    /// - The `Option` is used to handle the case where the list is empty.
    /// - This is basically the same as the [`super::second::List::peek`] method, except that it returns an `Option` instead of a `Result`.
    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.element)
    }

    /// Returns an iterator over the elements of the [`SinglyLinkedList`].
    pub fn iterator(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_deref() }
    }
}

/// An iterator over a [`SinglyLinkedList`].
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

/// Implement the [`Iterator`] trait for the [`Iter`] struct.
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.element
        })
    }
}

impl<T> Drop for SinglyLinkedList<T> {
    /// Drops the [`SinglyLinkedList`] and all its elements.
    /// # Algorithm
    /// - The `head` field of the list is taken and assigned to the `head` variable.
    /// - A `while` loop is used to iterate over the list.
    /// - The `head` variable is unwrapped using the `Rc::try_unwrap` method.
    /// - If the `Rc` is successfully unwrapped, the `next` field of the `Node` struct is assigned to the `head` variable.
    /// - If the `Rc` is not successfully unwrapped, the loop is broken.
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::SinglyLinkedList;

    #[test]
    fn basics() {
        let list = SinglyLinkedList::new();
        assert_eq!(list.head(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);

        // Make sure empty tail works
        let list = list.tail();
        assert_eq!(list.head(), None);
    }

    #[test]
    fn iterator() {
        let list = SinglyLinkedList::new().prepend(1).prepend(2).prepend(3);

        let mut iter = list.iterator();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
}
