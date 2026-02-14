use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // BinaryHeap is a max-heap, so we reverse the cost comparison
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

struct GridGraph {
    rows: usize,
    cols: usize,
    adj_list: Vec<Vec<Edge>>,
}

impl GridGraph {
    fn new(rows: usize, cols: usize) -> Self {
        GridGraph {
            rows,
            cols,
            adj_list: vec![Vec::new(); rows * cols],
        }
    }

    fn get_idx(&self, r: usize, c: usize) -> usize {
        r * self.cols + c
    }

    fn get_coords(&self, idx: usize) -> (usize, usize) {
        (idx / self.cols, idx % self.cols)
    }

    fn add_edge_by_idx(&mut self, u: usize, v: usize, weight: i32, bidirectional: bool) {
        if u < self.adj_list.len() && v < self.adj_list.len() {
            self.adj_list[u].push(Edge { node: v, weight });
            if bidirectional {
                self.adj_list[v].push(Edge { node: u, weight });
            }
        }
    }

    fn generate_standard_grid(&mut self, weight: i32) {
        for r in 0..self.rows {
            for c in 0..self.cols {
                let u = self.get_idx(r, c);
                if c + 1 < self.cols {
                    let v = self.get_idx(r, c + 1);
                    self.add_edge_by_idx(u, v, weight, true);
                }
                if r + 1 < self.rows {
                    let v = self.get_idx(r + 1, c);
                    self.add_edge_by_idx(u, v, weight, true);
                }
            }
        }
    }
}

fn dijkstra(graph: &GridGraph, start_idx: usize) -> (Vec<i32>, Vec<Option<usize>>) {
    let mut distances = vec![i32::MAX; graph.adj_list.len()];
    let mut predecessors = vec![None; graph.adj_list.len()];
    let mut heap = BinaryHeap::new();

    distances[start_idx] = 0;
    heap.push(State {
        cost: 0,
        position: start_idx,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if cost > distances[position] {
            continue;
        }

        for edge in &graph.adj_list[position] {
            let next_cost = cost + edge.weight;
            if next_cost < distances[edge.node] {
                distances[edge.node] = next_cost;
                predecessors[edge.node] = Some(position);
                heap.push(State {
                    cost: next_cost,
                    position: edge.node,
                });
            }
        }
    }
    (distances, predecessors)
}

fn main() {
    let mut grid = GridGraph::new(1000, 1000);

    grid.generate_standard_grid(1);

    let start_node = 3;
    let target_node = 999_999;

    grid.add_edge_by_idx(2010, 999_995, 2000, true);
    // grid.add_edge_by_idx(2010, 999_995, 2000, true);

    let (dists, predecessors) = dijkstra(&grid, start_node);

    if dists[target_node] == i32::MAX {
        println!("Path not found.");
    } else {
        println!("Shortest distance: {}", dists[target_node]);

        let mut path = Vec::new();
        let mut curr = Some(target_node);
        while let Some(idx) = curr {
            path.push(idx);
            curr = predecessors[idx];
        }
        path.reverse();

        println!("Full Path ({} nodes):", path.len());

        for (step, &idx) in path.iter().enumerate() {
            let (r, c) = grid.get_coords(idx);
            println!("Step {}: idx: {} -> Coords: ({}, {})", step, idx, r, c);
        }
    }
}
