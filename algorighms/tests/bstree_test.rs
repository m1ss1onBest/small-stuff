use algorighms::unordered::BSTree;

#[cfg(test)]
mod bstree_tests {
    use algorighms::unordered::BSTree;

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

    #[test]
    fn test_dfs() {
        let mut tree = BSTree::new();
        
        tree.insert(1);
        tree.insert(3);
        tree.insert(2);
        tree.insert(13);
        tree.insert(11);
        tree.insert(22);
        tree.insert(4);

        println!("dfs: {:?}", tree.dfs());
    }

    #[test]
    fn test_bfsf() {
        let mut tree = BSTree::new();

        tree.insert(1);
        tree.insert(3);
        tree.insert(2);
        tree.insert(13);
        tree.insert(11);
        tree.insert(22);
        tree.insert(4);

        println!("bfs: {:?}", tree.bsf())
    }
}


#[test]
fn depth_test() {
    let tree = BSTree::from_vec(vec![5, 7, 1, 0, 3, 8, 2]);
    println!("{:?}", tree.dfs());
}