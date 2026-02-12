use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone)]
struct Edge {
    node: usize,
    weight: i32,
}

fn dijkstra(
    vertices_count: usize,
    adj_list: &[Vec<Edge>],
    start: usize,
) -> (Vec<i32>, Vec<Option<usize>>) {
    let mut distances = vec![i32::MAX; vertices_count];
    let mut predecessors = vec![None; vertices_count];
    let mut heap = BinaryHeap::new();

    distances[start] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if cost > distances[position] {
            continue;
        }

        for edge in &adj_list[position] {
            let new_cost = cost + edge.weight;

            if new_cost < distances[edge.node] {
                distances[edge.node] = new_cost;
                predecessors[edge.node] = Some(position);
                heap.push(State {
                    cost: new_cost,
                    position: edge.node,
                });
            }
        }
    }

    (distances, predecessors)
}

fn reconstruct_path(target: usize, predecessors: &[Option<usize>]) -> Vec<usize> {
    let mut path = Vec::new();
    let mut current = Some(target);
    while let Some(node) = current {
        path.push(node);
        current = predecessors[node];
    }
    path.reverse();
    path
}

fn main() {
    let vertices_count = 5;
    let mut adj_list = vec![Vec::new(); vertices_count];

    let raw_edges = vec![
        (0, 4, 2),
        (0, 3, 7),
        (0, 2, 15),
        (0, 1, 25),
        (4, 3, 3),
        (3, 2, 4),
        (1, 2, 6),
    ];

    for (u, v, w) in raw_edges {
        adj_list[u].push(Edge { node: v, weight: w });
        adj_list[v].push(Edge { node: u, weight: w });
    }

    let start_node = 0;
    // Rust automatically converts &Vec to &[] (deref coercion)
    let (dist, predecessors) = dijkstra(vertices_count, &adj_list, start_node);

    println!("Dijkstra results (optimized) from node {}:", start_node);
    for (node_idx, &d) in dist.iter().enumerate() {
        let path = reconstruct_path(node_idx, &predecessors);
        let path_str = path
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(" -> ");

        println!(
            "Node {}: Distance = {:<2} | Path: {}",
            node_idx, d, path_str
        );
    }
}
