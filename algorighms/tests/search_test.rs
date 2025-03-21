#[cfg(test)]
mod search_tests {
    use std::time::Instant;

    use algorighms::search::{binary_search, linear_search};

    const elem: &i32 = &10000000;
    fn get_arr() -> Vec<i32> {
        (0..100000000).collect()
    }

    #[test]
    fn linear_search_test() {
        let arr = get_arr();
        let runtime = Instant::now();
        let _ = linear_search(&arr, elem);
        println!("binary search runtime: {:?}", runtime.elapsed());
    }

    #[test]
    fn test_binary_search() {
        let arr = get_arr();
        let runtime = Instant::now();
        let _ = binary_search(&arr, elem);
        println!("linear search runtime: {:?}", runtime.elapsed());
    }
}