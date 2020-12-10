use crate::infra::Problem;

pub struct Day6;

impl Problem<String, String, u32, u32> for Day6 {
    fn day() -> u8 {
        6
    }

    fn first(contents: String) -> u32 {
        contents
            .split("\n\n")
            .map(seen_answers)
            .map(u32::count_ones)
            .sum()
    }

    fn second(contents: String) -> u32 {
        contents
            .split("\n\n")
            .map(|group| {
                group
                    .lines()
                    .map(seen_answers)
                    .fold(0xffffffff, |a, b| a & b)
            })
            .map(|s| s.count_ones())
            .sum()
    }
}

fn seen_answers(s: &str) -> u32 {
    s.bytes()
        .filter(|&b| b != b'\n')
        .map(|b| b - b'a')
        .fold(0u32, |i, n| i | (1 << n))
}
