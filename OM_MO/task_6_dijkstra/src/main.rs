fn dijkstra(n: usize, start: usize, graph: &[Vec<i32>]) {
    let mut dist = vec![i32::MAX; n];
    let mut visited = vec![false; n]; // Set S
    let mut parent = vec![None; n]; // For path reconstruction

    dist[start] = 0;

    for _ in 0..n {
        // Step 1: Select vertex w with minimum cost D(w)
        let mut w = None;
        for v in 0..n {
            if !visited[v] && (w.is_none() || dist[v] < dist[w.unwrap()]) && dist[v] != i32::MAX {
                w = Some(v);
            }
        }

        let w = match w {
            Some(idx) => idx,
            None => break, // Exit if no more reachable vertices
        };

        // Step 2: Add w to set S
        visited[w] = true;

        // Step 3: Recalculate costs for all vertices v
        for v in 0..n {
            let weight = graph[w][v];
            // If edge exists (weight > 0) and vertex is not in S
            if weight > 0 && !visited[v] {
                let new_dist = dist[w] + weight;
                if new_dist < dist[v] {
                    dist[v] = new_dist;
                    parent[v] = Some(w); // Record that we reached v via w
                }
            }
        }
    }

    // Pass start node to use it in labels
    print_results(start, &dist, &parent);
}

fn print_results(start_node: usize, dist: &[i32], parent: &[Option<usize>]) {
    println!(
        "Dijkstra Results (Simple Version) from node {}:",
        start_node
    );
    for (i, item) in dist.iter().enumerate() {
        if *item == i32::MAX {
            println!("Node {}: Unreachable", i);
            continue;
        }

        // Reconstruct path using predecessors
        let mut path = Vec::new();
        let mut current = Some(i);
        while let Some(node) = current {
            path.push(node);
            current = parent[node];
        }
        path.reverse();

        let path_str = path
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(" -> ");

        println!("Node {}: Distance = {:<2} | Path: {}", i, item, path_str);
    }
}

fn main() {
    let n = 5;
    let mut graph = vec![vec![0; n]; n];

    // Graph data from image
    let edges = vec![
        (0, 4, 2),
        (0, 3, 7),
        (0, 2, 15),
        (0, 1, 25),
        (4, 3, 3),
        (3, 2, 4),
        (2, 1, 6),
    ];

    for (u, v, w) in edges {
        graph[u][v] = w;
        graph[v][u] = w; // Undirected graph
    }

    dijkstra(n, 0, &graph);
}
