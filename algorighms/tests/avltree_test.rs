#[cfg(test)]
mod avl_tests {
    use algorighms::trees::AvlTree;

    use super::*;

    #[test]
    fn test_avl_create() {
        let tree = AvlTree::from_vec(vec![1, 6, 3, 7, 2, 4, 9, 11, 12, 13, 14]);
        println!("{:?}", tree.bfs());
        // match tree.bfs() {
        //     Some(ref bfs) => format_bfs(bfs),
        //     None => println!("the tree is empty")
        // }

        // println!("{:#?}", tree);
    }
}