use std::fmt::Display;
use std::rc::Rc;

pub struct List<T: Display> {
    head: Link<T>,
}
type Link<T> = Option<Rc<Node<T>>>;
struct Node<T: Display> {
    elem: T,
    next: Link<T>,
}

// * Iterator Implementation
pub struct Iter<'a, T: Display> {
    next: Option<&'a Node<T>>,
}


impl<T: Display> List<T> {
    pub fn new() -> Self {
        List {head: None}
    }

    // append at front Node (old list as it's "next" value)
    pub fn append(&self, elem: T) -> List<T> {
        List {
            head: Some(Rc::new(Node {
                elem,
                next: self.head.clone(),
            }))
        }
    }
    // remove first element
    pub fn tail(&self) -> List<T> {
        List {
            // and_then takes Option<T>, and return Option<T.some_operation>
            head: self.head.as_ref().and_then(|node| {
                node.next.clone()
            })
        }
    }
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref()
        }
    }
}

impl<'a, T: Display> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        // self.next: Option<&'a Node<T>>
        // node: Option<Rc<Node<T>>>
        self.next.map(|node| {
            // as_deref() converts &Rc<T> to &T
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

impl<T: Display> Drop for List<T> {
    fn drop(&mut self) {
       // replace head(of list) with None, and return actual List
       let mut  curr_node = self.head.take();
        // run loop as long as curr_node is Some(node)
        while let Some(node) = curr_node {
            // if there's only 1 node inside Rc, take ownership unwrap and return
            if let Ok(mut node) = Rc::try_unwrap(node) {
                println!("drop {}",node.elem);
                // moving 1 position forward
                curr_node = node.next.take();
            }
                // if there are > 1 Rc pointers to node, then break
            else {
                break;
            }
        }
    }
}
