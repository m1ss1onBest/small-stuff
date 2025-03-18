#[cfg(test)]
mod avl_tests {
    use algorighms::trees::AvlTree;

    use super::*;

    #[test]
    fn test_avl_create() {
        let mut tree = AvlTree::new();
        tree.insert(3);
        tree.insert(5);
        tree.insert(2);

        println!("{:#?}", tree);
    }
}