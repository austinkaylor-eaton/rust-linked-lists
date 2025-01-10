//! https://rust-unofficial.github.io/too-many-lists/first-layout.html
/*
    What is a linked list?
    * A linked list is a data structure that represents a sequence of elements. Each element is a separate object. Each element (we will call it a node) of a list is comprising of two items - the data and a reference to the next node. The last node has a reference to null. The entry point into a linked list is called the head of the list. It should be noted that head is not a separate node, but the reference to the first node. If the list is empty then the head is a null reference.
    * More succinctly, a linked list is a data structure that consists of a sequence of elements, each containing a link to its successor.
    * Functional Programmers would define a linked list like so:
        * List a = Empty | Element a (List a)
        * This reads as "a list of a is either empty or an element of a followed by a list of a."
     * This is a recursive definition of a Sum Type
     
     What is a Sum Type?
     * A type that can have multiple constructors, each with its own data.
     * In Rust, we can use enums to define Sum Types.
     * A type that can have different values which may be of different types.
     
     How is an enum laid out in memory?
     * Example enum:
        enum Foo {
            D1(Type1),
            D2(Type2),
            ...
            DN(TypeN),
        }
     * The enum Foo will need a 'tag' to identify which variant of the enum is represented
     * It will also need enough space to store the largest variant plus some padding for safet
 */

/// Imports
use std::mem;

/// Abstraction for a singly linked list
/// # Remarks
/// * Because [List] is a struct with a single field, its size is the same as that field (zero-cost abstraction)
pub struct List {
    head: Link,
}

/// Represents a singly linked list
enum Link {
    /// Represents an empty list
    Empty,
    More(Box<Node>),
}

/// Represents a node in a singly linked list
struct Node {
    element: i32,
    next: Link,
}

impl List {
    /// Creates a new empty list
    /// # See Also
    /// * https://rust-unofficial.github.io/too-many-lists/first-new.html#new
    pub fn new() -> Self {
        List { head: Link::Empty }
    }
    
    /// Pushes a value onto the list
    /// # Arguments
    /// * `&mut self` - The list to push onto
    /// * `elem`: [i32] - The value to push onto the list
    /// # See Also
    /// * https://rust-unofficial.github.io/too-many-lists/first-push.html
    /// # Remarks
    /// * `&mut self` is a mutable reference to the list. This is a shadow parameter and is not explicitly passed in by the caller.
    /// * `push` mutates a list, so we need a mutable reference to the list.
    /// * `mem::replace` is used to swap the head `self.head` of the list with an empty node [Link::Empty]. This is done to avoid a double borrow of the list.
    pub fn push(&mut self, element: i32) {
        // Make a node to store the new element
        let new_node = Box::new(Node {
            element,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }
    
    /// Pops a value off of the list
    /// # Arguments
    /// * `&mut self` - The list to pop from
    /// # Returns
    /// * [Some] - The value popped off of the list (T = [i32])
    /// * [None] - If the list is empty
    /// # See Also
    /// * https://rust-unofficial.github.io/too-many-lists/first-pop.html
    /// # Remarks
    /// * We want to remove the head of the list and it's element, so we need to get the head of the list by value
    /// * We can't do that through the shared reference `&self.head` and we only have a mutable reference to `self`
    /// * We can't take ownership of the head of the list because we need to return it
    /// * So, we use `mem::replace` to swap the head of the list with an empty node [Link::Empty]
    /// # Algorithm
    /// 1. Check if the list is empty
    /// 2. If the list is empty, return [None]
    /// 3. If the list is not empty
    ///     * Remove the head of the list
    ///     * Remove the element from the head of the list
    ///     * Set the head of the list to the next node
    ///     * Return the element
    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => {
                None
            }
            Link::More(node) => {
                self.head = node.next;
                Some(node.element)                 
            }
        }
        // a macro to avoid warnings until we implement the function
        // unimplemented!();
    }
}

/// Implement the [Drop] trait for [List]
impl Drop for List {
    fn drop(&mut self) {
        // Replace the head with an empty Link, and then take ownership of the Link
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        // `while let` == "do this thing until this pattern doesn't match"
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
            // boxed_node goes out of scope and gets dropped here;
            // but its Node's `next` field has been set to Link::Empty
            // so no unbounded recursion occurs.
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
