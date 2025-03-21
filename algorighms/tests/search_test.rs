#[cfg(test)]
mod search_tests {
    use std::{iter::Inspect, time::Instant, vec};

    use algorighms::search::{binary_search, linear_search};

    use super::*;

    fn get_arr() -> Vec<i32> {
        (0..100000000).collect()
    }

    fn value_to_find() -> &'static i32 {
        &100000000
    }

    #[test]
    fn linear_search_test() {
        let arr = get_arr();
        let runtime = Instant::now();

        if let Some(val) = linear_search(&arr, value_to_find()) {
            // println!("linear: {}", val);
        }
        println!("binary search runtime: {:?}", runtime.elapsed());
    }

    #[test]
    fn test_binary_search() {
        let arr = get_arr();
        let runtime = Instant::now();

        if let Some(val) = binary_search(&arr, value_to_find()) {
            println!("binary: {}", val);
        }
        println!("linear search runtime: {:?}", runtime.elapsed());
    }
}