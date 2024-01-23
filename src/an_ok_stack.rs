
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

pub trait Drop {
    fn drop(&mut self);
}
pub struct List<T>{
    head: Link<T>,
}
type Link<T> = Option<Box<Node<T>>>;

struct Node<T>{
    elem: T,
    next: Link<T>,
}

// Container having List<> Data Member
pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T>{
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T>{
    pub fn new() -> Self{
        List { head: None}
    }

    // peek/front an element
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    // push at front
    pub fn push(&mut self, elem: T) {
        let node_to_insert = Box::new(Node{
            elem,
            // 1. drop self.head and store in next, put None in self.head
            // next: mem::replace(&mut self.head, None),

            // 2. replace head with None and return original value
            next: self.head.take(),
        });
        self.head = Some(node_to_insert);
    }

    // pop at front
    pub fn pop(&mut self) -> Option<T> {
        // replace self.head with None, and return original value
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn into_iter(self) ->IntoIter<T> {
        // move self(List) to > IntoIter tuple struct
        IntoIter(self)
    }
    // &self(List) needs to be valid, until Iter is available around
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            next: self.head.as_deref()
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_deref_mut()
        }
    }
}

// implement Iterator trait for IntoIter tuple struct
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

// Iter Borrows/references the List
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

// IterMut takes Mutable reference
impl<'a, T> Iterator for IterMut<'a, T>{
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node|{
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

// to Drop every item of list
impl<T: std::fmt::Display> Drop for List<T>{
    fn drop(&mut self) {
       let mut curr_link = self.head.take();

        while let Some(mut boxed_node) = curr_link{
            println!("drop {}",boxed_node.elem);
            curr_link = boxed_node.next.take();
        }
    }
}
