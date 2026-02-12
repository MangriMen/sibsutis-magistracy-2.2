use crate::knapsack::Item;

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

    let capacity = 13;

    println!("Items: {items:#?}");

    let (unbounded_val, unbounded_items) = knapsack::unbounded(capacity, &items);
    let (zero_one_val, zero_one_items) = knapsack::zero_one(capacity, &items);

    println!("\n--- Unbounded Knapsack ---");
    println!("Maximum value: {}", unbounded_val);
    println!("Used items:",);
    for i in unbounded_items {
        println!("{i}: {:?}", items[i]);
    }

    println!("\n--- (0/1) Knapsack ---");
    println!("Maximum value: {}", zero_one_val);
    println!("Used items");
    for i in zero_one_items {
        println!("{i}: {:?}", items[i]);
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
        // stores the index of the item used to reach this weight
        let mut item_at_weight = vec![None; capacity + 1];

        for i in 1..=capacity {
            for (idx, item) in items.iter().enumerate() {
                if item.weight <= i && dp[i - item.weight] + item.value > dp[i] {
                    dp[i] = dp[i - item.weight] + item.value;
                    item_at_weight[i] = Some(idx);
                }
            }
        }

        // Backtrack to find which items were used
        let mut picked_items = Vec::new();
        let mut curr_w = capacity;
        while let Some(idx) = item_at_weight[curr_w] {
            picked_items.push(idx);
            curr_w -= items[idx].weight;
        }

        (dp[capacity], picked_items)
    }

    pub fn zero_one(capacity: usize, items: &[Item]) -> (u32, Vec<usize>) {
        let n = items.len();
        // dp[i] stores the maximum value for capacity i
        let mut dp = vec![vec![0; capacity + 1]; n + 1];

        for i in 1..=n {
            let item = &items[i - 1];
            for w in 0..=capacity {
                if item.weight <= w {
                    dp[i][w] = dp[i - 1][w].max(dp[i - 1][w - item.weight] + item.value);
                } else {
                    dp[i][w] = dp[i - 1][w];
                }
            }
        }

        let mut picked_items = Vec::new();
        let mut curr_w = capacity;
        for i in (1..=n).rev() {
            // If the value changed compared to the previous row, the item was taken
            if dp[i][curr_w] != dp[i - 1][curr_w] {
                picked_items.push(i - 1);
                curr_w -= items[i - 1].weight;
            }
        }

        (dp[n][capacity], picked_items)
    }
}
