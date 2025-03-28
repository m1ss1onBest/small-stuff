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

pub fn quick_sort<T: Ord>(slice: &mut [T]) {
    if slice.len() <= 1 {
        return;
    }

    let pivot_index = partition(slice);
    quick_sort(&mut slice[0..pivot_index]);
    quick_sort(&mut slice[pivot_index + 1..]);
}

fn partition<T: Ord>(slice: &mut [T]) -> usize {
    let pivot_index = slice.len() - 1;
    let mut i = 0;

    for j in 0..pivot_index {
        if slice[j] <= slice[pivot_index] {
            slice.swap(i, j);
            i += 1;
        }
    }
    slice.swap(i, pivot_index);
    i
}

pub fn merge_sort<T: Ord + Clone>(slice: &mut [T]) {
    let len = slice.len();
    if len <= 1 {
        return;
    }

    let mid = len / 2;
    let mut left = slice[..mid].to_vec();
    let mut right = slice[mid..].to_vec();

    merge_sort(&mut left);
    merge_sort(&mut right);

    merge(slice, &left, &right);
}

fn merge<T: Ord + Clone>(slice: &mut [T], left: &[T], right: &[T]) {
    let (mut i, mut j, mut k) = (0, 0, 0);

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            slice[k] = left[i].clone();
            i += 1;
        } else {
            slice[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        slice[k] = left[i].clone();
        i += 1;
        k += 1;
    }

    while j < right.len() {
        slice[k] = right[j].clone();
        j += 1;
        k += 1;
    }
}
