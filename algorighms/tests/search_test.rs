#[cfg(test)]
mod search_tests {
    use std::time::Instant;

    use algorighms::search::*;

    const ELEM: &i32 = &10000000;
    fn get_arr() -> Vec<i32> {
        (0..100000000).collect()
    }

    #[test]
    fn linear_search_test() {
        let arr = get_arr();
        let runtime = Instant::now();
        let a = linear_search(&arr, ELEM);
        println!("linear search runtime: {:?}, {a}", runtime.elapsed());
    }

    #[test]
    fn binary_search_test() {
        let arr = get_arr();
        let runtime = Instant::now();
        let a = binary_search(&arr, ELEM);
        println!("binary search runtime: {:?}, {a}", runtime.elapsed());
    }

    #[test]
    fn interpolation_search_test() {
        let slice = get_arr();
        let a = interpolation_search(&slice, 500);
        let runtime = Instant::now();
        println!("Interpolation search runtime: {:?}, {a:?}", runtime.elapsed());
    }
}