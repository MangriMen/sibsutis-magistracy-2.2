use crate::knapsack::Item;
use std::collections::HashMap; // Import HashMap for counting

fn main() {
    let items = vec![
        Item {
            weight: 3,
            value: 8,
        },
        Item {
            weight: 5,
            value: 14,
        },
        Item {
            weight: 8,
            value: 23,
        },
    ];

    let capacity = 3_000_000;

    println!("Items: {items:#?}");

    let (unbounded_val, unbounded_items) = knapsack::unbounded(capacity, &items);

    println!("\n--- Unbounded Knapsack ---");
    println!("Maximum value: {}", unbounded_val);

    // Group and count items
    let mut counts = HashMap::new();
    for &idx in &unbounded_items {
        *counts.entry(idx).or_insert(0) += 1;
    }

    println!("Used items (index: count):");
    for (idx, count) in counts {
        println!("Item {}: {:?} (Quantity: {})", idx, items[idx], count);
    }
}

mod knapsack {
    #[derive(Debug)]
    pub struct Item {
        pub weight: usize,
        pub value: u32,
    }

    pub fn unbounded(capacity: usize, items: &[Item]) -> (u32, Vec<usize>) {
        let mut dp = vec![0; capacity + 1];
        let mut item_at_weight = vec![None; capacity + 1];

        for i in 1..=capacity {
            for (idx, item) in items.iter().enumerate() {
                if item.weight <= i && dp[i - item.weight] + item.value > dp[i] {
                    dp[i] = dp[i - item.weight] + item.value;
                    item_at_weight[i] = Some(idx);
                }
            }
        }

        let mut picked_items = Vec::new();
        let mut curr_w = capacity;
        // Optimization: Find actual used capacity to start backtracking
        while curr_w > 0 && item_at_weight[curr_w].is_none() {
            curr_w -= 1;
        }

        while let Some(idx) = item_at_weight[curr_w] {
            picked_items.push(idx);
            curr_w -= items[idx].weight;
        }

        (dp[capacity], picked_items)
    }
}
