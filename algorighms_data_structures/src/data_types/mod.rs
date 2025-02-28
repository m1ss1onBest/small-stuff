pub mod linked_list;

pub use linked_list::LinkedList;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_list() {
        let a:  LinkedList<i32> = LinkedList::new();
        assert_eq!(a.len(), 0);
    }

    #[test]
    fn test_list_push() {
        let mut a = LinkedList::new();
        a.push(1);
        a.push(2);
        a.push(3);

        assert_eq!(a.len(), 3);
        println!("{:#?}", a);
    }
}