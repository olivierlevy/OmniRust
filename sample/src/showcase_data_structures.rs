use omnirust::data_structures::{tree::Tree, graph::Graph, circular_buffer::CircularBuffer, priority_queue::PriorityQueue};

pub fn run_data_structures_showcase() {
    println!("\n--- Data Structures Showcase ---");
    // Tree
    println!("\n  Tree Example:");
    let tree = Tree::with_root("Root".to_string());
    if let Some(root) = &tree.root {
        let mut root_mut = root.borrow_mut();
        let child1 = root_mut.add_child("Child 1".to_string());
        root_mut.add_child("Child 2".to_string());
        child1.borrow_mut().add_child("Grandchild 1.1".to_string());
    }
    println!("    Tree structure:\n{}", tree);

    // Graph
    println!("\n  Graph Example (Weighted, Directed):");
    let mut graph: Graph<String, i32> = Graph::new();
    let n0 = graph.add_node("Node0".to_string());
    let n1 = graph.add_node("Node1".to_string());
    let n2 = graph.add_node("Node2".to_string());
    graph.add_edge(n0, n1, 10);
    graph.add_edge(n1, n2, 20);
    graph.add_edge(n0, n2, 5); // Shorter path
    println!("    Graph: {:?}, Nodes: {}, Edges: {}", graph, graph.node_count(), graph.edge_count());
    println!("    BFS from Node0: {:?}", graph.bfs(n0));
    println!("    DFS from Node0: {:?}", graph.dfs(n0));

    // Circular Buffer
    println!("\n  Circular Buffer Example (capacity 3):");
    let mut c_buffer: CircularBuffer<i32> = CircularBuffer::new(3);
    c_buffer.push_back(1);
    c_buffer.push_back(2);
    c_buffer.push_back(3);
    println!("    Buffer full: {:?}", c_buffer.iter().collect::<Vec<_>>()); // [1, 2, 3]
    c_buffer.push_back(4); // Overwrites 1
    println!("    Pushed 4 (overwrite): {:?}", c_buffer.iter().collect::<Vec<_>>()); // [2, 3, 4]
    println!("    Popped: {:?}", c_buffer.pop_front()); // Some(2)
    println!("    Buffer after pop: {:?}", c_buffer.iter().collect::<Vec<_>>()); // [3, 4]
}

// Removed redundant import

pub fn run_priority_queue_showcase() {
    println!("\n  Priority Queue Example:");
    let mut pq = PriorityQueue::new();
    pq.push("Task A (Prio 2)", 2);
    pq.push("Task B (Prio 1)", 1);
    pq.push("Task C (Prio 3)", 3);

    println!("    Initial queue (highest prio first):");
    while let Some(task) = pq.pop() {
        println!("      - Popped: {}", task);
    }

    pq.push("Task D (Prio 5)", 5);
    pq.push("Task E (Prio 5)", 5); // Same priority
    pq.push("Task F (Prio 0)", 0);
    println!("    Peek before pop: {:?}", pq.peek());
    println!("    Queue length: {}", pq.len());
    println!("    Popping again:");
    while let Some(task) = pq.pop() {
        println!("      - Popped: {}", task);
    }
    println!("    Is empty: {}", pq.is_empty());
}
