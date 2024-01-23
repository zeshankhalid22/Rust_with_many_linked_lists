use crate::an_ok_stack::List;

pub mod an_ok_stack;
mod tests;

fn main() {
    let mut list = List::new();
    list.push(14);
    list.push(15);
    list.push(18);

    let front = list.peek();
    match front {
        Some(value) => println!("Top is {}",value),
        None => println!("empty q."),
    }
}
