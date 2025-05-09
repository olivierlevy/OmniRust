// src/data_structures/graph.rs

use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;
use std::fmt::Debug;
use std::marker::PhantomData;

/// Represents a node in the graph.
pub type NodeId = usize;

/// Represents an edge in the graph, optionally with a weight.
#[derive(Debug, Clone, PartialEq)]
pub struct Edge<W: Clone + Debug + Default> {
    pub to: NodeId,
    pub weight: W,
}

impl<W: Clone + Debug + Default> Edge<W> {
    pub fn new(to: NodeId, weight: W) -> Self {
        Edge { to, weight }
    }
}

/// Adjacency list representation of a graph.
/// N is the type of the node data, W is the type of the edge weight.
#[derive(Debug, Clone)]
pub struct Graph<N: Debug + Clone, W: Clone + Debug + Default> {
    nodes: HashMap<NodeId, N>,
    adj_list: HashMap<NodeId, Vec<Edge<W>>>,
    next_node_id: NodeId,
    phantom_n: PhantomData<N>, // To use N in struct definition if nodes map is empty
}

impl<N: Debug + Clone, W: Clone + Debug + Default> Graph<N, W> {
    /// Creates a new empty graph.
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
            adj_list: HashMap::new(),
            next_node_id: 0,
            phantom_n: PhantomData,
        }
    }

    /// Adds a new node with the given data to the graph.
    /// Returns the ID of the new node.
    pub fn add_node(&mut self, data: N) -> NodeId {
        let id = self.next_node_id;
        self.nodes.insert(id, data);
        self.adj_list.insert(id, Vec::new());
        self.next_node_id += 1;
        id
    }

    /// Adds a directed edge from `from_node` to `to_node` with the given weight.
    /// Panics if `from_node` or `to_node` do not exist.
    pub fn add_edge(&mut self, from_node: NodeId, to_node: NodeId, weight: W) {
        if !self.nodes.contains_key(&from_node) || !self.nodes.contains_key(&to_node) {
            panic!("One or both nodes do not exist in the graph.");
        }
        self.adj_list
            .get_mut(&from_node)
            .unwrap()
            .push(Edge::new(to_node, weight));
    }
    
    /// Adds an undirected edge between `node1` and `node2` with the given weight.
    /// Panics if `node1` or `node2` do not exist.
    pub fn add_undirected_edge(&mut self, node1: NodeId, node2: NodeId, weight: W) {
        self.add_edge(node1, node2, weight.clone());
        self.add_edge(node2, node1, weight);
    }

    /// Gets the data associated with a node.
    pub fn get_node_data(&self, node_id: NodeId) -> Option<&N> {
        self.nodes.get(&node_id)
    }

    /// Gets the neighbors (and edge weights) of a node.
    pub fn get_neighbors(&self, node_id: NodeId) -> Option<&Vec<Edge<W>>> {
        self.adj_list.get(&node_id)
    }

    /// Returns the number of nodes in the graph.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Returns the number of edges in the graph.
    pub fn edge_count(&self) -> usize {
        self.adj_list.values().map(|edges| edges.len()).sum()
    }

    /// Performs a Breadth-First Search (BFS) starting from `start_node`.
    /// Returns a set of visited node IDs.
    /// Panics if `start_node` does not exist.
    pub fn bfs(&self, start_node: NodeId) -> HashSet<NodeId> {
        if !self.nodes.contains_key(&start_node) {
            panic!("Start node does not exist in the graph.");
        }

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        visited.insert(start_node);
        queue.push_back(start_node);

        while let Some(current_node) = queue.pop_front() {
            if let Some(neighbors) = self.adj_list.get(&current_node) {
                for edge in neighbors {
                    if !visited.contains(&edge.to) {
                        visited.insert(edge.to);
                        queue.push_back(edge.to);
                    }
                }
            }
        }
        visited
    }

    /// Performs a Depth-First Search (DFS) starting from `start_node`.
    /// Returns a set of visited node IDs.
    /// Panics if `start_node` does not exist.
    pub fn dfs(&self, start_node: NodeId) -> HashSet<NodeId> {
        if !self.nodes.contains_key(&start_node) {
            panic!("Start node does not exist in the graph.");
        }
        let mut visited = HashSet::new();
        self.dfs_recursive(start_node, &mut visited);
        visited
    }

    fn dfs_recursive(&self, current_node: NodeId, visited: &mut HashSet<NodeId>) {
        visited.insert(current_node);
        if let Some(neighbors) = self.adj_list.get(&current_node) {
            for edge in neighbors {
                if !visited.contains(&edge.to) {
                    self.dfs_recursive(edge.to, visited);
                }
            }
        }
    }
}

impl<N: Debug + Clone, W: Clone + Debug + Default> Default for Graph<N, W> {
    fn default() -> Self {
        Self::new()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_new() {
        let graph: Graph<String, f32> = Graph::new();
        assert_eq!(graph.node_count(), 0);
        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn test_add_node() {
        let mut graph: Graph<String, ()> = Graph::new(); // Using () for no edge weight
        let node0 = graph.add_node("Node A".to_string());
        let node1 = graph.add_node("Node B".to_string());
        assert_eq!(node0, 0);
        assert_eq!(node1, 1);
        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.get_node_data(node0), Some(&"Node A".to_string()));
    }

    #[test]
    fn test_add_edge() {
        let mut graph: Graph<&str, i32> = Graph::new();
        let n0 = graph.add_node("A");
        let n1 = graph.add_node("B");
        let n2 = graph.add_node("C");

        graph.add_edge(n0, n1, 10);
        graph.add_edge(n0, n2, 20);
        graph.add_edge(n1, n2, 5);

        assert_eq!(graph.edge_count(), 3);
        let n0_neighbors = graph.get_neighbors(n0).unwrap();
        assert_eq!(n0_neighbors.len(), 2);
        assert!(n0_neighbors.contains(&Edge::new(n1, 10)));
        assert!(n0_neighbors.contains(&Edge::new(n2, 20)));
    }
    
    #[test]
    fn test_add_undirected_edge() {
        let mut graph: Graph<char, f64> = Graph::new();
        let a = graph.add_node('a');
        let b = graph.add_node('b');
        graph.add_undirected_edge(a, b, 1.5);

        assert_eq!(graph.edge_count(), 2); // Two directed edges
        assert_eq!(graph.get_neighbors(a).unwrap(), &vec![Edge::new(b, 1.5)]);
        assert_eq!(graph.get_neighbors(b).unwrap(), &vec![Edge::new(a, 1.5)]);
    }


    #[test]
    #[should_panic]
    fn test_add_edge_invalid_node() {
        let mut graph: Graph<i32, ()> = Graph::new();
        graph.add_node(1);
        graph.add_edge(0, 99, ()); // Node 99 does not exist
    }

    #[test]
    fn test_bfs() {
        let mut graph: Graph<String, ()> = Graph::new();
        let n0 = graph.add_node("0".to_string());
        let n1 = graph.add_node("1".to_string());
        let n2 = graph.add_node("2".to_string());
        let n3 = graph.add_node("3".to_string());
        let n4 = graph.add_node("4".to_string()); // Disconnected node

        graph.add_edge(n0, n1, ());
        graph.add_edge(n0, n2, ());
        graph.add_edge(n1, n2, ());
        graph.add_edge(n2, n0, ()); // Cycle
        graph.add_edge(n2, n3, ());
        graph.add_edge(n3, n3, ()); // Self-loop

        let visited_from_n0 = graph.bfs(n0);
        let expected_from_n0: HashSet<NodeId> = [n0, n1, n2, n3].iter().cloned().collect();
        assert_eq!(visited_from_n0, expected_from_n0);
        
        let visited_from_n4 = graph.bfs(n4);
        let expected_from_n4: HashSet<NodeId> = [n4].iter().cloned().collect();
        assert_eq!(visited_from_n4, expected_from_n4);
    }

    #[test]
    fn test_dfs() {
        let mut graph: Graph<String, i32> = Graph::new();
        let n0 = graph.add_node("0".to_string());
        let n1 = graph.add_node("1".to_string());
        let n2 = graph.add_node("2".to_string());
        let n3 = graph.add_node("3".to_string());
        let n4 = graph.add_node("4".to_string()); // Disconnected

        graph.add_edge(n0, n1, 1);
        graph.add_edge(n0, n2, 1);
        graph.add_edge(n1, n3, 1);
        // n2 has no outgoing edges to unvisited nodes from n0's perspective initially

        let visited_from_n0 = graph.dfs(n0);
        let expected_from_n0: HashSet<NodeId> = [n0, n1, n2, n3].iter().cloned().collect();
         assert_eq!(visited_from_n0, expected_from_n0);

        let visited_from_n4 = graph.dfs(n4);
        let expected_from_n4: HashSet<NodeId> = [n4].iter().cloned().collect();
        assert_eq!(visited_from_n4, expected_from_n4);
    }
}
