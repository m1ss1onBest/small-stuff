pub fn linear_search<'a, T: Ord>(slice: &'a [T], elem: &T) -> Option<&'a T> {
    for i in slice {
        if i == elem {
            return Some(i);
        }
    }
    None
}

// pub fn binary_search<'a, T: Ord>(slice: &'a [T], elem: &T) -> Option<&'a T> {
//     if slice.is_empty() {
//         return None;
//     }
    
//     let mut left = 0;
//     let mut right = slice.len() - 1;

//     while left <= right {
//         let mid = left + (right - left) / 2;
//         let mid_val = &slice[mid];

//         if mid_val < elem {
//             left = mid + 1;
//         } else if mid_val > elem {
//             right = mid.saturating_sub(1);
//         } else {
//             return Some(mid_val);
//         }
//     }
//     None
// }

// pub fn interpolation_search<'a, T: Ord>(slice: &'a [T], to_find: &T) -> Option<&'a T> {
//     let mut low = 0;
//     let mut high = slice.len() - 1;

//     while slice[low] < *to_find && slice[high] > *to_find {
//         if slice[high] == slice[low] {
//             break;
//         }

//         let mid = low + ((to_find - slice[low]) * (high - low));
//     }
//     None
// }