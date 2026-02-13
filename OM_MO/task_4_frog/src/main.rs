use std::collections::HashMap;

type FrogState = (usize, usize); // (current_index, jump_length)
type PathData = (i32, usize, usize); // (min_steps, from_index, previous_jump)

fn main() {
    let stones = vec![1, 1, 0, 1, 1, 1, 0, 0, 1];

    match frog_jumps(&stones) {
        Some((count, path)) => {
            println!("Minimum number of jumps: {}", count);
            println!("Path by indices: {:?}", path);
        }
        None => {
            println!("Path is impossible");
        }
    }
}

fn frog_jumps(stones: &[i32]) -> Option<(i32, Vec<usize>)> {
    let n = stones.len();

    let mut dp: HashMap<FrogState, PathData> = HashMap::new();

    // starting condition: jump to stone 1 has length 1.
    if n > 1 && stones[1] == 1 {
        dp.insert((1, 1), (1, 0, 0));
    } else {
        return None;
    }

    for i in 1..n {
        let current_states: Vec<(FrogState, PathData)> = dp
            .iter()
            .filter(|&(&(pos, _), _)| pos == i)
            .map(|(&k, &v)| (k, v))
            .collect();

        for ((pos, k), (steps, _, _)) in current_states {
            let possible_jumps = vec![k as i32 - 1, k as i32, k as i32 + 1];

            for next_jump in possible_jumps {
                if next_jump < 1 {
                    continue;
                }

                let next_pos = pos + next_jump as usize;

                if next_pos < n && stones[next_pos] == 1 {
                    let state = (next_pos, next_jump as usize);
                    let new_steps = steps + 1;

                    if !dp.contains_key(&state) || dp[&state].0 > new_steps {
                        dp.insert(state, (new_steps, pos, k));
                    }
                }
            }
        }
    }

    let best_state = dp
        .keys()
        .filter(|&&(pos, _)| pos == n - 1)
        .min_by_key(|&&s| dp[&s].0)
        .cloned();

    if let Some(final_state) = best_state {
        let total_steps = dp[&final_state].0;
        let mut path = Vec::new();
        let mut curr = Some(final_state);

        while let Some(state) = curr {
            path.push(state.0);
            let &(_, prev_idx, prev_k) = &dp[&state];

            if prev_idx == 0 && prev_k == 0 {
                path.push(0);
                break;
            }
            curr = Some((prev_idx, prev_k));
        }

        path.reverse();
        Some((total_steps, path))
    } else {
        None
    }
}
