use std::fmt::{Debug, Write};

pub mod collections;
pub mod trees;
pub mod test_structs;
pub mod sorting;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn format_bfs<T: Debug>(v: &Vec<T>) {
    let mut offset = 1;
    let mut buf = String::new();
    let mut level = 0;

    while offset - 1 < v.len() {
        let start = offset - 1;
        let end = (offset * 2 - 1).min(v.len());
        
        let padding = 2usize.pow(5 - level) - 1; 
        
        buf.push_str(&" ".repeat(padding));
        
        for i in start..end {
            buf.push_str(&format!("{:?}{}", v[i], " ".repeat(padding * 2 + 1)));
        }

        buf.push('\n');
        offset *= 2;
        level += 1;
    }

    println!("{}", buf);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
