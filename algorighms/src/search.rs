use std::cmp::Ordering;

pub fn linear_search<T: Ord>(slice: &[T], val: &T) -> bool {
    for i in slice {
        if i == val {
            return true
        }
    }
    false
}

pub fn binary_search<T: Ord>(slice: &[T], val: &T) -> bool {
    if slice.is_empty() {
        return false
    }

    let mut left = 0;
    let mut right = slice.len() - 1;
    
    while left <= right {
        let mid = (right + left) / 2;


        match val.cmp(&slice[mid]) {
            Ordering::Equal => return true,
            Ordering::Greater => left = mid + 1,
            Ordering::Less => right = mid - 1,
        }
    }
    false
}

pub fn interpolation_search(slice: &[i32], to_find: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = slice.len() - 1;

    while slice[low] < to_find && slice[high] > to_find {
        if slice[high] == slice[low] {
            break;
        }

        let mid = ((to_find - slice[low]) as usize * (high - low)) / (slice[high] - slice[low]) as usize;
        if slice[mid] < to_find {
            low = mid + 1;
        } else if slice[mid] > to_find {
            high = mid + 1;
        } else {
            return Some(mid);
        }
    }

    if slice[low] == to_find {
        return Some(low);
    }
    if slice[high] == to_find {
        return Some(high);
    }
    None
}