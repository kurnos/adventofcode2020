pub struct Day23;

impl crate::infra::Problem<&str, &str, String, u64, 23> for Day23 {
    fn first(contents: &str) -> String {
        let data = contents.bytes().map(|b| (b - b'0') as u32);

        let after = run(data, 9, 100);
        let mut res = vec![];
        let mut current = 1;
        for _ in 0..8 {
            current = after[current as usize];
            res.push(current as u8 + b'0');
        }
        String::from_utf8(res).unwrap()
    }

    fn second(contents: &str) -> u64 {
        const MAX: u32 = 1000000;

        let after = run(
            contents.bytes().map(|b| (b - b'0') as u32).chain(10..=MAX),
            MAX,
            10_000_000,
        );
        let a = after[1];
        let b = after[a as usize];
        a as u64 * b as u64
    }
}

fn run(mut data: impl Iterator<Item = u32>, max: u32, times: usize) -> Vec<u32> {
    let mut after = vec![0; data.size_hint().1.unwrap() + 1];
    let first = data.next().unwrap();
    let mut current = first;
    for next in data {
        after[current as usize] = next;
        current = next;
    }
    after[current as usize] = first;
    current = first;

    for _ in 1..=times {
        let a = after[current as usize];
        let b = after[a as usize];
        let c = after[b as usize];
        let dest = dest(current, [a, b, c], max);

        after[current as usize] = after[c as usize];
        after[c as usize] = after[dest as usize];
        after[dest as usize] = a;

        current = after[current as usize];
    }
    after
}

fn dest(c: u32, cs: [u32; 3], max: u32) -> u32 {
    let mut d = c - 1;
    if d == 0 {
        d = max
    };
    while cs.contains(&d) {
        d -= 1;
        if d == 0 {
            d = max
        };
    }
    d
}
