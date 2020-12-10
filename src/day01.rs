use crate::infra;

pub struct Day1;

impl infra::Problem<String, String, u32, u32> for Day1 {
    fn day() -> u8 {
        1
    }
    fn first(contents: String) -> u32 {
        find_target(&parse_numbers(contents), 2020).unwrap()
    }
    fn second(contents: String) -> u32 {
        let data = parse_numbers(contents);
        for a in &data {
            if let Some(n) = find_target(&data, 2020 - a) {
                return n * a;
            }
        }
        panic!();
    }
}

fn parse_numbers(contents: String) -> Vec<u32> {
    let mut data: Vec<u32> = contents
        .lines()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    data.sort();
    data
}

fn find_target(data: &[u32], target: u32) -> Option<u32> {
    let mut low = 0usize;
    let mut high = data.len() - 1;
    while low < high {
        match data[low] + data[high] {
            d if d == target => {
                return Some(data[low] * data[high]);
            }
            d if d < target => {
                low += 1;
            }
            _ => {
                high -= 1;
            }
        }
    }
    None
}
