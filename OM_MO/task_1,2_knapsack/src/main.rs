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

    let unbounded_max_value = knapsack::unbounded(capacity, &items);
    let zero_one_max_value = knapsack::zero_one(capacity, &items);

    println!(
        "Maximum value in unbounded knapsack: {}",
        unbounded_max_value
    );
    println!("Maximum value in (0/1) knapsack: {}", zero_one_max_value);
}

mod knapsack {
    #[derive(Debug)]
    pub struct Item {
        pub weight: usize,
        pub value: u32,
    }

    pub fn unbounded(capacity: usize, items: &[Item]) -> u32 {
        // dp[i] stores the maximum value for capacity i
        let mut dp = vec![0; capacity + 1];

        // Iterate through every capacity from 1 to total capacity
        for i in 1..=capacity {
            // Try every item for the current capacity
            for item in items {
                if item.weight <= i {
                    // Maximize value: either keep current or take the item + value from remaining space
                    dp[i] = dp[i].max(dp[i - item.weight] + item.value);
                }
            }
        }

        dp[capacity]
    }

    pub fn zero_one(capacity: usize, items: &[Item]) -> u32 {
        // dp[i] stores the maximum value for capacity i
        let mut dp = vec![0; capacity + 1];

        for item in items {
            // Iterate backwards to ensure each item is used only once
            // If we went forward, we would build on the current item's result
            for i in (item.weight..=capacity).rev() {
                dp[i] = dp[i].max(dp[i - item.weight] + item.value);
            }
        }

        dp[capacity]
    }
}
