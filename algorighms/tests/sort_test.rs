#[cfg(test)]
mod sort_tests {
    use algorighms::sorting::{bubble_sort, insertion_sort, selection_sort};

    fn get_unsorted() -> [i32; 10] {
        [1, 5, 3, -3, 6, 2, 4, -1, 0, -2]
    }

    #[test]
    fn bubblesort_test() {
        let mut arr = get_unsorted();
        bubble_sort(&mut arr);
        println!("{:?}", arr);
    }

    #[test]
    fn insertion_sort_test() {
        let mut arr = get_unsorted();
        insertion_sort(&mut arr);
        println!("{:?}", arr);
    }

    #[test]
    fn selection_sort_test() {
        let mut arr = get_unsorted();
        selection_sort(&mut arr);
        println!("{:?}", arr);
    }
}