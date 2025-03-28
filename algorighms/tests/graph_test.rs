#[cfg(test)]
mod graph_tests {
    use algorighms::unordered::{graph, Graph};

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

    // #[test]
    // fn graph_test() {
    //     let graph = get_graph();
    //     graph.print();
    // }

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

    #[test]
    fn graph_dijkstra_test() {
        let graph = get_graph();
        let dijkstra = graph.dijkstra(0);
        println!("dijkstar {:?}", dijkstra);
        // println!("{:?}\n{:?}", pairs, distances);
    }

    #[test]
    fn ford_test() {
        // let mut graph = Graph::new(5);
        // graph.add_edge(0, 1, 6);
        // graph.add_edge(0, 2, 7);
        // graph.add_edge(1, 2, 8);
        // graph.add_edge(1, 3, 5);
        // graph.add_edge(1, 4, -4);
        // graph.add_edge(2, 3, -3);
        // graph.add_edge(2, 4, 9);
        // graph.add_edge(3, 1, -2);
        // graph.add_edge(4, 0, 2);
        // graph.add_edge(4, 3, 7);

        let graph = get_graph();

        let ford = graph.bellman_ford(0);
        println!("bellman: {:?}", ford);
    }
}