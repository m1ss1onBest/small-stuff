pub fn linear_search<'a, T: Ord>(slice: &'a [T], elem: &T) -> Option<(&'a T, usize)> {
    for (idx, i) in slice.iter().enumerate() {
        if i == elem {
            return Some((i, idx))
        }
    }
    None
}
