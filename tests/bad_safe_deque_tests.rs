#[cfg(test)]
mod test {
    use Rust_with_many_lls::bad_safe_deque::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop_front(), None);

        // Populate list
        list.push_front(1);
        list.push_front(2);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(),None);

        // Push some more just to make sure nothing's corrupted
        list.push_back(4);
        list.push_back(5);
        list.push_back(9);

        assert_eq!(list.pop_back(),Some(9));
        assert_eq!(list.pop_back(),Some(5));
        assert_eq!(list.pop_back(),Some(4));
        assert_eq!(list.pop_back(),None);

        list.push_back(12);
        list.push_front(6);
        drop(list);
    }
}
