#[cfg(test)]
mod tests {
    use algorighms::sorting::bubble_sort;

    #[test]
    fn bubblesort_test() {
        let mut arr = [1, 5, 3, 6, 2, 4];

        println!("before: {:?}", arr);
        bubble_sort(&mut arr);
        println!("after: {:?}", arr);
    }
}