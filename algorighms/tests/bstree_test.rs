#[cfg(test)]
mod bstree_tests {
    use algorighms::trees::BSTree;

    use super::*;

    #[test]
    fn test_print() {
        let mut tree = BSTree::new();
        tree.insert(1);
        tree.insert(3);
        tree.insert(2);
        tree.insert(13);
        tree.insert(11);
        tree.insert(22);
        tree.insert(4);

        tree.print_tree();
    }
}
