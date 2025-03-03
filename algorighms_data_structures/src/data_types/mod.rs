pub mod linked_list;

#[cfg(test)]
mod tests {
    use linked_list::LinkedList;

    use super::*;

    #[test]
    fn test_create_empty_list() {
        let a: LinkedList<i32> = LinkedList::new();
        assert_eq!(a.len(), 0);
    }

    #[test]
    fn test_push_items_to_list() {
        let mut a = LinkedList::new();
        a.push(1);
        a.push(2);
        a.push(3);

        // println!("the list\n {:#?}", a);
        assert_eq!(a.len(), 3);
    }

    #[test]
    fn test_into_iter() {
        let mut a = LinkedList::new();
        a.push(1);
        a.push(2);
        a.push(3);

        for i in a {
            println!("Item of list: {}", i);
        }
    }
}