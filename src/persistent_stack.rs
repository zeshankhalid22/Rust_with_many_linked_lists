use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
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
}

