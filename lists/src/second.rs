pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

// Tuple structs are an alternative form of struct,
// useful for trivial wrappers around other types.
pub struct IntoIterator<T>(List<T>);

// Iter is generic over *some* lifetime, it doesn't care
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

pub struct IteratorMutable<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
    
    /// Peek at the first element (head) of the list, if it exists
    /// # Returns
    /// * [Some] - A reference to the first element of the list
    /// * [None] - If the list is empty
    /// # Remarks
    /// * `.map()` takes `self` by value, so we need to use `as_ref()` to get a reference to the head of the list
    /// * `as_ref()` demotes `Option<T>` to `Option<&T>`
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }
    
    /// Peek at the first element (head) of the list, if it exists
    /// # Returns
    /// * [Some] - A mutable reference to the first element of the list
    /// * [None] - If the list is empty
    /// # Remarks
    /// * This is a mutable version of [peek]
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }

    /// Consume the list and return an iterator
    pub fn into_iterator(self) -> IntoIterator<T> {
        IntoIterator(self)
    }

    // We declare a fresh lifetime here for the *exact* borrow that
    // creates the iter. Now &self needs to be valid as long as the
    // Iter is around.
    // #Remarks
    /// * we are moving the boxed node into map, which means it would be dropped after the map call
    /// * that means we'd have a dangling reference
    /// * to fix this, we need to borrow the node, and return a reference to it
    pub fn iterator(&self) -> Iter<T> {
        Iter { next: self.head.as_deref() }
    }

    pub fn iterator_mutable(&mut self) -> IteratorMutable<'_, T> {
        IteratorMutable { next: self.head.as_deref_mut() }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

impl<T> Iterator for IntoIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // access fields of a tuple struct numerically
        self.0.pop()
    }
}

// We *do* have a lifetime here, because Iter has one that we need to define
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

impl<'a, T> Iterator for IteratorMutable<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn push(){
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
    
    #[test]
    fn pop_empty_list() {
        let mut list: List<i32> = List::new();
        assert_eq!(list.pop(), None);
    }
    
    #[test]
    fn pop_list() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
    }

    #[test]
    fn peek_empty_list() {
        let list: List<i32> = List::new();
        assert_eq!(list.peek(), None);
    }
    
    #[test]
    fn peek_list() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));
    }
    
    #[test]
    fn peek_mut_empty_list()
    {
        let mut list: List<i32> = List::new();
        assert_eq!(list.peek_mut(), None);
    }
    
    #[test]
    fn peek_mut_list() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.peek_mut(), Some(&mut 3));
    }
    
    #[test]
    fn into_iterator() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        let mut iter = list.into_iterator();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iterator() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iterator();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn iterator_mutable() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iterator_mutable();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }
}

