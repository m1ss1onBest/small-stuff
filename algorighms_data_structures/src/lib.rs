pub fn add(left: u64, right: u64) -> u64 {
    left + right
}


mod data_types {
    pub mod vector;
}

fn foo() {
    let a = '2' + 3;
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
