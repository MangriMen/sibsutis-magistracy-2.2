#[derive(Debug, Clone)]
struct Edge {
    source: usize,
    target: usize,
    weight: i32,
}

fn bellman_ford(
    vertices_count: usize,
    edges: &[Edge],
    start_node: usize,
) -> Option<(Vec<i32>, Vec<Option<usize>>)> {
    let mut distances = vec![i32::MAX; vertices_count];
    let mut predecessors = vec![None; vertices_count]; // Used for path reconstruction
    distances[start_node] = 0;

    // Standard relaxation (V - 1) times
    for _ in 0..(vertices_count - 1) {
        let mut changed = false;
        for edge in edges {
            if distances[edge.source] != i32::MAX {
                let new_dist = distances[edge.source] + edge.weight;
                if new_dist < distances[edge.target] {
                    distances[edge.target] = new_dist;
                    predecessors[edge.target] = Some(edge.source);
                    changed = true;
                }
            }
        }
        if !changed {
            break;
        }
    }

    // Check for negative weight cycles
    for edge in edges {
        if distances[edge.source] != i32::MAX
            && distances[edge.source] + edge.weight < distances[edge.target]
        {
            return None;
        }
    }

    Some((distances, predecessors))
}

/// Helper function to build a path from start to target using the predecessors list
fn reconstruct_path(target: usize, predecessors: &[Option<usize>]) -> Vec<usize> {
    let mut path = Vec::new();
    let mut current = Some(target);

    while let Some(node) = current {
        path.push(node);
        current = predecessors[node];
    }

    path.reverse(); // Path is built backwards, so we flip it
    path
}

fn main() {
    let vertices_count = 5;

    // Edges based on the provided image
    let raw_edges = vec![
        (0, 4, 2),
        (0, 3, 7),
        (0, 2, 15),
        (0, 1, 25),
        (4, 3, 3),
        (3, 2, 4),
        (1, 2, 6),
    ];

    let mut edges = Vec::new();
    for (u, v, w) in raw_edges {
        // Both directions for undirected graph
        edges.push(Edge {
            source: u,
            target: v,
            weight: w,
        });
        edges.push(Edge {
            source: v,
            target: u,
            weight: w,
        });
    }

    let start_node = 0;
    match bellman_ford(vertices_count, &edges, start_node) {
        Some((dist, predecessors)) => {
            println!("Shortest paths from node {}:", start_node);
            for (i, item) in dist.iter().enumerate() {
                if *item == i32::MAX {
                    println!("Node {}: Unreachable", i);
                } else {
                    let path = reconstruct_path(i, &predecessors);
                    // Formatting the path as 0 -> 4 -> 3
                    let path_str = path
                        .iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<String>>()
                        .join(" -> ");

                    println!("Node {}: Distance = {:<2} | Path: {}", i, item, path_str);
                }
            }
        }
        None => println!("Error: Graph contains a negative weight cycle!"),
    }
}
