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
    let mut predecessors = vec![None; vertices_count];
    distances[start_node] = 0;

    // Table Header
    println!(
        "{:<10} | {}",
        "Итерация",
        (1..vertices_count)
            .map(|i| format!("D({})", i))
            .collect::<Vec<_>>()
            .join(" | ")
    );
    println!("{}", "-".repeat(45));

    // Print initial state (Iteration 0)
    print_step_row(0, &distances);

    // Standard relaxation (V - 1) times
    for i in 1..vertices_count {
        let mut changed = false;

        // We use a copy to ensure we show distances at the END of the iteration
        let mut next_distances = distances.clone();

        for edge in edges {
            if distances[edge.source] != i32::MAX {
                let new_dist = distances[edge.source] + edge.weight;
                if new_dist < next_distances[edge.target] {
                    next_distances[edge.target] = new_dist;
                    predecessors[edge.target] = Some(edge.source);
                    changed = true;
                }
            }
        }

        distances = next_distances;
        print_step_row(i, &distances);

        if !changed {
            println!("(Остановка: изменений больше нет)");
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

fn print_step_row(step: usize, distances: &[i32]) {
    let d_cols: Vec<String> = distances
        .iter()
        .skip(1)
        .map(|&d| {
            if d == i32::MAX {
                format!("{:<4}", "inf")
            } else {
                format!("{:<4}", d)
            }
        })
        .collect();

    println!("{:<10} | {}", step, d_cols.join(" | "));
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

    let raw_edges = vec![
        (0, 4, 2),
        (0, 3, 7),
        (0, 2, 15),
        (0, 1, 25),
        (4, 3, 3),
        (3, 2, 4),
        (2, 1, 6),
    ];

    let mut edges = Vec::new();
    for (u, v, w) in raw_edges {
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

    match bellman_ford(vertices_count, &edges, 0) {
        Some((dist, predecessors)) => {
            println!("\nИтоговые пути:");
            for i in 0..vertices_count {
                let path = reconstruct_path(i, &predecessors);
                let path_str = path
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join(" -> ");
                println!("Node {}: Dist = {:<2} | Path: {}", i, dist[i], path_str);
            }
        }
        None => println!("Ошибка: найден цикл отрицательного веса!"),
    }
}
