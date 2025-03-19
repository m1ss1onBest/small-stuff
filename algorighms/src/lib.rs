pub mod collections;
pub mod unordered;
pub mod test_structs;
pub mod sorting;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

// pub fn format_bfs<T: Debug>(v: &Vec<T>) {
//     let mut offset = 1;
//     let mut buf = String::new();

//     buf.push_str(&format!("{:?}\t", v[0]));
//     for i in 1..v.len() {
//         buf.push_str(&format!("{:?}\t", v[i]));

//         if i == offset {
//             offset *= 2;
//             buf.push('\n');
//         }
//     }
//     println!("{}", buf);
// }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
