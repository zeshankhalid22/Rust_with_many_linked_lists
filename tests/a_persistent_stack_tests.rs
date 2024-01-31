#[cfg(test)]
mod test{

    use Rust_with_many_lls::persistent_stack::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(list.peek(),None);

        // append at front
        list = list.append(5).append(8).append(3);
        assert_eq!(list.peek(),Some(&3));

        // remove at front
        list = list.tail().tail();
        assert_eq!(list.peek(),Some(&5));

        list = list.append(10).append(13);

        let mut iter = list.iter();
        assert_eq!(iter.next(),Some(&13));
        assert_eq!(iter.next(),Some(&10));
        assert_eq!(iter.next(),Some(&5));

    }
}