
#[cfg(test)]
mod test{
    use Rust_with_many_lls::an_ok_stack::*;
#[test]
fn test_push_pop() {
    let mut list = List::new();

    // Check empty list behaves right
    assert_eq!(list.pop(),None);

    // Add items
    list.push(24);
    list.push(18);
    list.push(37);

    // pop items
    assert_eq!(list.pop(),Some(37));

    // into_iter
    let mut consume_iter = list.into_iter();
    // now list is moved in the IntoIter struct ()
    assert_eq!(consume_iter.next(), Some(18));
    assert_eq!(consume_iter.next(), Some(24));
    assert_eq!(consume_iter.next(), None);

    // * not allowed,  was moved
    // list.push(34);
}

#[test]
fn iter_mut() {
    let mut list = List::new();
    list.push(1);
    list.push(3);
    list.push(5);

    let mut iter = list.iter_mut();
    assert_eq!(iter.next(),Some(&mut 5));
    assert_eq!(iter.next(),Some(&mut 3));
    assert_eq!(iter.next(),Some(&mut 1));
}
}

