fn main() {
    let input = std::fs::read_to_string("resources/day1.txt").unwrap();
    let t0 = std::time::Instant::now();
    println!("first: {:?}", { first(&input) });
    println!("{:?}", t0.elapsed());
    let t1 = std::time::Instant::now();
    println!("second: {:?}", { second(&input) });
    println!("{:?}", t1.elapsed());
}

fn parse_numbers(contents: &str) -> Vec<u32> {
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

fn first(contents: &str) -> Option<u32> {
    find_target(&parse_numbers(contents), 2020)
}

fn second(contents: &str) -> Option<u32> {
    let data = parse_numbers(contents);
    for a in &data {
        if let Some(n) = find_target(&data, 2020 - a) {
            return Some(n * a);
        }
    }
    None
}
