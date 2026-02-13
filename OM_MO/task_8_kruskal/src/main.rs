use std::collections::HashMap;

fn main() {
    let num_vertices = 7;
    // Edges based on the provided graph image
    let edges = vec![
        Edge {
            src: 0,
            dest: 1,
            weight: 20,
        },
        Edge {
            src: 1,
            dest: 2,
            weight: 5,
        },
        Edge {
            src: 2,
            dest: 3,
            weight: 3,
        },
        Edge {
            src: 3,
            dest: 4,
            weight: 17,
        },
        Edge {
            src: 4,
            dest: 5,
            weight: 28,
        },
        Edge {
            src: 5,
            dest: 0,
            weight: 23,
        },
        Edge {
            src: 0,
            dest: 6,
            weight: 1,
        },
        Edge {
            src: 1,
            dest: 6,
            weight: 4,
        },
        Edge {
            src: 2,
            dest: 6,
            weight: 9,
        },
        Edge {
            src: 3,
            dest: 6,
            weight: 16,
        },
        Edge {
            src: 4,
            dest: 6,
            weight: 25,
        },
        Edge {
            src: 5,
            dest: 6,
            weight: 36,
        },
    ];

    kruskal(num_vertices, edges);
}

#[derive(Copy, Clone, Debug)]
struct Edge {
    src: usize,
    dest: usize,
    weight: i32,
}

struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            // Step 3: All vertices start as separate components
            parent: (0..n).collect(),
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            return i;
        }
        self.parent[i] = self.find(self.parent[i]);
        self.parent[i]
    }

    fn union(&mut self, i: usize, j: usize) -> bool {
        let root_i = self.find(i);
        let root_j = self.find(j);
        if root_i != root_j {
            self.parent[root_i] = root_j; // Merge components
            return true;
        }
        false // Edge creates a cycle
    }

    // Formats output as: (1,7),2,(3,4),5,6
    fn get_components_display(&mut self, n: usize) -> String {
        let mut groups = HashMap::new();
        for i in 0..n {
            let root = self.find(i);
            groups.entry(root).or_insert(Vec::new()).push(i + 1);
        }

        let mut sorted_groups: Vec<Vec<usize>> = groups.into_values().collect();
        sorted_groups.sort_by_key(|g| g[0]);

        let mut result = Vec::new();
        for mut group in sorted_groups {
            group.sort();
            if group.len() > 1 {
                let inner = group
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join(",");
                result.push(format!("({})", inner));
            } else {
                result.push(group[0].to_string());
            }
        }
        result.join(",")
    }
}

fn kruskal(num_vertices: usize, mut edges: Vec<Edge>) {
    const L_COL: usize = 40;

    edges.sort_by_key(|e| e.weight);

    let mut uf = UnionFind::new(num_vertices);
    let mut mst_labels = Vec::new();
    let mut num_components = num_vertices;

    println!(
        "{:<width$} | Connectivity Components",
        "Subgraph (Edges)",
        width = L_COL
    );
    println!(
        "{:-<width$}-----------------------------------",
        "-",
        width = L_COL
    );

    println!(
        "{:<width$} | {}",
        "empty",
        uf.get_components_display(num_vertices),
        width = L_COL
    );

    for edge in edges {
        let edge_label = format!("({}-{})", edge.src + 1, edge.dest + 1);

        if uf.union(edge.src, edge.dest) {
            mst_labels.push(edge_label.clone());
            num_components -= 1;

            let current_mst = mst_labels.join("");
            // Вывод строки через динамическую ширину
            println!(
                "{:<width$} | {}",
                current_mst,
                uf.get_components_display(num_vertices),
                width = L_COL
            );
        } else {
            let current_mst_with_skip = format!("{}+{} (skip)", mst_labels.join(""), edge_label);
            println!(
                "{:<width$} | {}",
                current_mst_with_skip,
                uf.get_components_display(num_vertices),
                width = L_COL
            );
        }

        if num_components == 1 {
            break;
        }
    }
}
