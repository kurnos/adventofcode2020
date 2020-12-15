use crate::infra::Problem;

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

fn run_with(preamble: &[u32], limit: u32) -> u32 {
    let mut last_spoken = vec![0; limit as usize];

    for (t, &n) in preamble.iter().take(preamble.len() - 1).enumerate() {
        last_spoken[n as usize] = t as u32 + 1;
    }

    let mut current = preamble[preamble.len() - 1];

    for turn in preamble.len() as u32..limit {
        let n = last_spoken[current as usize];
        last_spoken[current as usize] = turn;
        current = if n != 0 { turn - n } else { 0 };
    }

    current
}
