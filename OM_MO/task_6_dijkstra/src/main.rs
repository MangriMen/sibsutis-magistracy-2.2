fn dijkstra(n: usize, start: usize, graph: &[Vec<i32>]) {
    let mut dist = vec![i32::MAX; n];
    let mut visited = vec![false; n];
    let mut s_set = Vec::new(); // Track set S for display

    // Initial distances from start node
    dist[start] = 0;
    for v in 0..n {
        if graph[start][v] > 0 {
            dist[v] = graph[start][v];
        }
    }

    // Print table header
    println!(
        "{:<15} | {:<5} | {:<5} | {}",
        "S",
        "w",
        "D(w)",
        (1..n)
            .map(|i| format!("D({})", i))
            .collect::<Vec<_>>()
            .join(" | ")
    );
    println!("{}", "-".repeat(60));

    // First row: Initial state
    print_row("0", "None", None, &dist, &visited, n);

    for _ in 1..n {
        // Step 1: Select vertex w with minimum cost D(w) not in S
        let mut w = None;
        let mut min_dist = i32::MAX;

        for v in 1..n {
            if !visited[v] && dist[v] < min_dist {
                min_dist = dist[v];
                w = Some(v);
            }
        }

        let w = match w {
            Some(idx) => idx,
            None => break,
        };

        // Step 2: Add w to set S
        visited[w] = true;
        s_set.push(w);

        // Prepare S label for the table: e.g., (0, 4, 3)
        let mut s_label = vec![0];
        s_label.extend(&s_set);
        let s_str = format!(
            "({Bold})",
            Bold = s_label
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(",")
        );

        // Print row BEFORE updating distances to show D(w) that was selected
        print_row(&s_str, &w.to_string(), Some(dist[w]), &dist, &visited, n);

        // Step 3: Update distances
        for v in 1..n {
            if !visited[v] && graph[w][v] > 0 {
                let new_dist = dist[w] + graph[w][v];
                if new_dist < dist[v] {
                    dist[v] = new_dist;
                }
            }
        }
    }
}

fn print_row(s: &str, w: &str, dw: Option<i32>, dist: &[i32], visited: &[bool], n: usize) {
    let dw_str = dw.map_or("Неоп.".to_string(), |v| v.to_string());

    let mut d_cols = Vec::new();
    for i in 1..n {
        if visited[i] {
            d_cols.push(format!("{:<4}", " — ")); // Already in S
        } else if dist[i] == i32::MAX {
            d_cols.push(format!("{:<4}", "inf"));
        } else {
            d_cols.push(format!("{:<4}", dist[i]));
        }
    }

    println!(
        "{:<15} | {:<5} | {:<5} | {}",
        s,
        w,
        dw_str,
        d_cols.join(" | ")
    );
}

fn main() {
    let n = 5;
    let mut graph = vec![vec![0; n]; n];

    // Edges based on the table logic
    let edges = vec![
        (0, 4, 2),
        (0, 3, 7),
        (0, 2, 15),
        (0, 1, 25),
        (4, 3, 3),
        (3, 2, 4), // 4 + 5 (from D(4)) = 9 in the table
        (2, 1, 6), // 9 + 6 = 15 in the table
    ];

    for (u, v, w) in edges {
        graph[u][v] = w;
        graph[v][u] = w;
    }

    dijkstra(n, 0, &graph);
}
