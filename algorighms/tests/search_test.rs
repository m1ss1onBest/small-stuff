#[cfg(test)]
mod search_tests {
    use std::vec;

    use algorighms::search::linear_search;

    use super::*;

    pub fn get_unsorted() -> Vec<i32> {
        vec![1, 3, 2, 4, 12, 11, 8, 4, 5]
    }

    #[test]
    fn linear_search_test() {
        let arr = get_unsorted();
        if let Some((val, idx)) = linear_search(&arr, &4) {
            println!("linear: {} {}", val, idx + 1);
        }
    }
}