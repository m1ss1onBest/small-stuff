#[cfg(test)]
mod graph_tests {
    use algorighms::unordered::Graph;

    fn get_graph() -> Graph {
        let mut graph = Graph::new(6);

        // graph.add_egde(0, 1, 2);
        // graph.add_egde(0, 2, 4);
        // graph.add_egde(1, 2, 1);
        // graph.add_egde(1, 3, 7);
        // graph.add_egde(2, 4, 3);
        // graph.add_edge(3, 4, 2);

        graph.add_edge(0, 1, 1);
        graph.add_edge(0, 2, 1);
        graph.add_edge(1, 3, 1);
        graph.add_edge(1, 4, 1);
        graph.add_edge(2, 4, 1);
        graph.add_edge(3, 5, 1);
        graph.add_edge(4, 5, 1);

        return graph;
    }

    #[test]
    fn graph_test() {
        let graph = get_graph();
        graph.print();
    }

    #[test]
    fn graph_dfs_test() {
        let graph = get_graph();
        println!("dfs: {:?}", graph.dfs(0));
    }

    #[test]
    fn graph_bfs_test() {
        let graph = get_graph();
        println!("bfs: {:?}", graph.bfs(0))
    }
}