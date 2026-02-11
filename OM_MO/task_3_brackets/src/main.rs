fn main() {
    // Test case 1
    let dims1 = vec![10, 20, 50, 1, 100];
    let names1 = vec!["M1", "M2", "M3", "M4"];
    solve_matrix_multiplication(&dims1, &names1);
    println!();

    // Test case 2
    let dims2 = vec![10, 20, 5, 4, 30, 6];
    let names2 = vec!["M1", "M2", "M3", "M4", "M5"];
    solve_matrix_multiplication(&dims2, &names2);
}

/// Calculates the optimal order for matrix multiplication.
fn matrix_multiplication_order(dimensions: &[usize]) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let n = dimensions.len() - 1;
    // cost[i][j] stores the minimum number of scalar multiplications
    let mut cost = vec![vec![0; n + 1]; n + 1];
    // split[i][j] stores the index of the matrix after which the split occurs
    let mut split = vec![vec![0; n + 1]; n + 1];

    // t is the chain length
    for t in 1..n {
        for k in 1..=(n - t) {
            let j = k + t;
            cost[k][j] = usize::MAX;

            for m in k..j {
                let current_cost =
                    cost[k][m] + cost[m + 1][j] + dimensions[k - 1] * dimensions[m] * dimensions[j];

                if current_cost < cost[k][j] {
                    cost[k][j] = current_cost;
                    split[k][j] = m;
                }
            }
        }
    }

    (cost, split)
}

/// Recursively constructs the optimal parenthesis string.
fn get_optimal_parenthesis(split: &Vec<Vec<usize>>, i: usize, j: usize, names: &[&str]) -> String {
    if i == j {
        names[i - 1].to_string()
    } else {
        let left = get_optimal_parenthesis(split, i, split[i][j], names);
        let right = get_optimal_parenthesis(split, split[i][j] + 1, j, names);
        format!("({} × {})", left, right)
    }
}

fn solve_matrix_multiplication(dimensions: &[usize], names: &[&str]) {
    let n = dimensions.len() - 1;

    println!("Number of matrices: {}", n);
    println!("Dimensions:");
    for i in 0..n {
        println!(
            "  {}: [{} × {}]",
            names[i],
            dimensions[i],
            dimensions[i + 1]
        );
    }

    let (cost, split) = matrix_multiplication_order(dimensions);
    let result = get_optimal_parenthesis(&split, 1, n, names);

    println!("Optimal order: {}", result);
    println!("Minimum cost: {}", cost[1][n]);
}
