use crate::infra::Problem;
use std::collections::HashMap;

pub struct Day15;

impl Problem<Vec<u32>, Vec<u32>, u32, u32> for Day15 {
    fn day() -> u8 {
        15
    }

    fn first(contents: Vec<u32>) -> u32 {
        run_with(&contents, 2020)
    }

    fn second(contents: Vec<u32>) -> u32 {
        run_with(&contents, 30000000)
    }
}

fn run_with(preamble: &[u32], n: u32) -> u32 {
    let mut last_spoken = HashMap::new();

    for (t, n) in preamble.iter().take(preamble.len() - 1).enumerate() {
        last_spoken.insert(*n, t as u32);
    }

    let mut current = preamble[preamble.len() - 1];
    let mut turn = preamble.len() as u32 - 1;

    while turn < n - 1 {
        if let Some(n) = last_spoken.get(&current).cloned() {
            last_spoken.insert(current, turn);
            current = turn - n
        } else {
            last_spoken.insert(current, turn);
            current = 0;
        }
        turn += 1;
    }
    current
}
