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
        println!("pop element: {:?}", list.pop_front());
        println!("pop element: {:?}", list.pop_back());
        list.print();


        // while let Some(i) = list.pop_front() {
        //     println!("{}", i)
        // }
    }
}
