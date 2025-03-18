pub fn bubble_sort<T: Ord>(slice: &mut[T]) {
    for i in 1..slice.len() {
        for j in 0..(slice.len() - i) {
            if slice[j] > slice[j + 1] {
                slice.swap(j, j + 1);
            }
        }
    }
}