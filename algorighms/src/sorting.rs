pub fn bubble_sort<T: Ord>(slice: &mut[T]) {
    for i in 1..slice.len() {
        for j in 0..(slice.len() - i) {
            if slice[j] > slice[j + 1] {
                slice.swap(j, j + 1);
            }
        }
    }
}

pub fn insertion_sort<T: Ord>(slice: &mut [T]) {
    for i in 1..slice.len() {
        let mut j = i;
        while j > 0 && slice[j - 1] > slice[j] {
            slice.swap(j - 1, j);
            j -= 1;
        }
    }
}

pub fn selection_sort<T: Ord>(slice: &mut [T]) {
    let len = slice.len();
    for i in 0..len {
        let mut min_index = i;
        for j in i + 1..len {
            if slice[j] < slice[min_index] {
                min_index = j;
            }
        }
        if min_index != i {
            slice.swap(i, min_index);
        }
    }
}