#[cfg(test)]
mod tests {
    use algorighms::collections::LList;

    use super::*;

    #[test]
    fn test_list() {
        let mut list = LList::new();
        for i in 0..10 {
            list.push_back(i);
        }

        list.print();

        while let Some(i) = list.pop_back() {
            println!("{}", i)
        }
    }
}