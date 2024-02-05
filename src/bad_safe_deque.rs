use std::rc::Rc;   // allow multiple owners, but with immutable access
use std::cell::RefCell;

// TODO impl Iterators
pub struct List<T: std::fmt::Display> {
    head: Link<T>,
    tail: Link<T>,
}

// a Node can have multiple owners and can be mutated
type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        // initialize and return an instance of Node<T> wrapped by RefCell, Rc
        Rc::new(RefCell::new(Node {
            elem,
            next: None,
            prev: None,
        }))
    }
}

impl<T: std::fmt::Display> List<T> {
    // TODO peek_front

    pub fn new() -> Self {
        List{head:None, tail:None}
    }
    pub fn push_front(&mut self, elem: T) {
        let new_head = Node::new(elem);
        match self.head.take() {
            // non-empty list
            Some(old_head) => {
                // connect old_head with new_head
                // increase reference count(Rc) of new_head
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }
    pub fn pop_front(&mut self) -> Option<T>{
        self.head.take().map(|old_head| {
            // match against next value of old_head
            match old_head.borrow_mut().next.take() {
                // non-empty list
                Some(new_head) => {
                    // disconnect prev of new_head by replacing it with None
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => {
                    // if only one element, emptying list
                    self.tail.take();
                }
            }
            // old_head.into_inner().elem // cannot move out of Rc
            // Rc::try_unwrap moves out the content of Rc if it's RefCount is 1
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }

    pub fn push_back(&mut self, elem: T) {
        let new_tail = Node::new(elem);
        // replace actual tail with None, and compare against Some(old_tail)
        // here the actual tail will be separated by list, then will be added at the end later
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail); // new_tail moved
            }
            None => {
                    self.head = Some(new_tail.clone());
                    self.tail = Some(new_tail);
            }
        }
    }
    pub fn pop_back(&mut self) -> Option<T> {
        // original tail is returned inside map and replaced with None
        self.tail.take().map(|old_tail| {
            // match against prev value of old_tail to decide new_tail
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    // set new_tail's next value to None (disconnect from original tail)
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    // emptying list
                    self.head.take();
                }
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().elem
        })
    }

    // TODO implement peek functions (Ref, RefMut)

}

impl<T: std::fmt::Display> Drop for List<T> {
    fn drop(&mut self) {
        while let Some(val) = self.pop_front() {
            println!("drop {}", val);
        }
    }
}