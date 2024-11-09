use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Graph {
    adjacency_list: HashMap<i32, Vec<i32>>, // Node -> List of connected nodes
}

impl Graph {
    // Create a new, empty graph
    fn new() -> Self {
        Graph {
            adjacency_list: HashMap::new(),
        }
    }

    // Add a node to the graph
    fn add_node(&mut self, node: i32) {
        self.adjacency_list.entry(node).or_insert(Vec::new());
    }

    // Add a directed edge from `src` to `dest`
    fn add_edge(&mut self, src: i32, dest: i32) {
        self.adjacency_list.entry(src).or_insert(Vec::new()).push(dest);
    }

    // Perform DFS traversal
    fn dfs(&self, start: i32, visited: &mut HashSet<i32>) {
        if visited.contains(&start) {
            return;
        }
        println!("Visited {}", start);
        visited.insert(start);

        if let Some(neighbors) = self.adjacency_list.get(&start) {
            for &neighbor in neighbors {
                self.dfs(neighbor, visited);
            }
        }
    }
}

fn main() {
    // Initialize a new graph
    let mut graph = Graph::new();

    // Add nodes to the graph
    graph.add_node(1);
    graph.add_node(2);
    graph.add_node(3);
    graph.add_node(4);

    // Add edges
    graph.add_edge(1, 2);
    graph.add_edge(1, 3);
    graph.add_edge(2, 4);
    graph.add_edge(3, 4);

    // DFS traversal starting from node 1
    let mut visited = HashSet::new();
    println!("DFS traversal starting from node 1:");
    graph.dfs(1, &mut visited);
}
