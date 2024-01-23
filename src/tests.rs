use crate::an_ok_stack::{Iterator, List};

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