use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
    fmt::Write,
    vec,
};

pub struct Graph {
    inner: Vec<Vec<(i32, i32)>>,
}

impl Graph {
    pub fn new(size: usize) -> Graph {
        Graph {
            inner: vec![vec![]; size],
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn add_edge(&mut self, from: i32, to: i32, weight: i32) {
        self.inner[from as usize].push((to, weight));
    }

    pub fn print(&self) {
        let mut buf = String::new();

        for (i, edges) in self.inner.iter().enumerate() {
            write!(&mut buf, "{}->", i).unwrap();
            for &(neighbour, weight) in edges {
                write!(&mut buf, "({},{})", neighbour, weight).unwrap();
            }
            writeln!(&mut buf).unwrap();
        }

        println!("{}", buf);
    }

    pub fn bfs(&self, start: i32) -> Vec<i32> {
        let mut visited = vec![false; self.inner.len()];
        let mut queue = VecDeque::new();
        let mut res = Vec::new();

        queue.push_back(start);
        visited[start as usize] = true;

        while let Some(node) = queue.pop_front() {
            res.push(node);

            for &(neighbor, _) in &self.inner[node as usize] {
                if !visited[neighbor as usize] {
                    visited[neighbor as usize] = true;
                    queue.push_back(neighbor);
                }
            }
        }
        res
    }

    pub fn dfs(&self, start: i32) -> Vec<i32> {
        let mut visited = vec![false; self.inner.len()];
        let mut stack = vec![start];
        let mut res = Vec::new();

        while let Some(node) = stack.pop() {
            if !visited[node as usize] {
                visited[node as usize] = true;
                res.push(node);

                for &(neighbor, _) in self.inner[node as usize].iter().rev() {
                    if !visited[neighbor as usize] {
                        stack.push(neighbor);
                    }
                }
            }
        }
        res
    }

    pub fn dijkstra(&self, start: i32) -> Vec<i32> {
        let n = self.len();
        let mut dist = vec![i32::MAX; n];
        let mut heap = BinaryHeap::new();

        dist[start as usize] = 0;
        // (distance, node)
        heap.push(Reverse((0, start))); 

        while let Some(Reverse((cost, node))) = heap.pop() {
            if cost > dist[node as usize] {
                continue;
            }

            for &(neighbor, weight) in &self.inner[node as usize] {
                let next_cost = cost + weight;
                if next_cost < dist[neighbor as usize] {
                    dist[neighbor as usize] = next_cost;
                    heap.push(Reverse((next_cost, neighbor)));
                }
            }
        }

        dist
    }
}
