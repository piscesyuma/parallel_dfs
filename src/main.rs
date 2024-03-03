use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

type Graph = HashMap<usize, Vec<usize>>;

fn parallel_dfs(graph: &Graph) -> HashSet<usize> {
    let visited = Arc::new(Mutex::new(HashSet::new()));
    let nodes: Vec<_> = graph.keys().cloned().collect();

    nodes.par_iter().for_each(|&node| {
        let local_visited = visited.clone();
        dfs_helper(node, &graph, &local_visited);
    });

    Arc::try_unwrap(visited).unwrap().into_inner().unwrap()
}

fn dfs_helper(node: usize, graph: &Graph, visited: &Arc<Mutex<HashSet<usize>>>) {
    let mut visit_guard = visited.lock().unwrap();
    if !visit_guard.insert(node) {
        return;
    }
    drop(visit_guard); // Release the lock as soon as possible

    if let Some(neighbors) = graph.get(&node) {
        for &next_node in neighbors {
            dfs_helper(next_node, graph, visited);
        }
    }
}

fn main() {
    // Example graph: adjacency list representation
    // 0 -> 1 -> 2 -> 0 (cycle), 3 -> 4 (disconnected component)
    let graph: Graph = [
        (0, vec![1]),
        (1, vec![2]),
        (2, vec![0]), // This introduces a cycle: 0 -> 1 -> 2 -> 0
        (3, vec![4]), // Disconnected component
        (4, vec![]),  // Leaf node
    ]
    .iter()
    .cloned()
    .collect();

    let visited = parallel_dfs(&graph);

    // Depending on your use case, you can do something with the visited nodes.
    // Here, we just print them.
    println!("Visited nodes: {:?}", visited);
}