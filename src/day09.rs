use crate::infra::Problem;

pub struct Day9;

impl Problem<String, String, u64, u64> for Day9 {
    fn day() -> u8 {
        9
    }

    fn first(contents: String) -> u64 {
        let size = 25;
        let data = parse(&contents);

        (0..data.len() - size)
            .filter_map(|i| {
                for j in i..i + size {
                    for k in j..i + size {
                        if data[j] + data[k] == data[i + size] {
                            return None;
                        }
                    }
                }
                Some(data[i + size])
            })
            .next()
            .unwrap()
    }

    fn second(contents: String) -> u64 {
        let target = Self::first(contents.clone());
        let data = parse(&contents);

        let (mut a, mut b, mut sum) = (0, 0, data[0]);

        while sum != target {
            if sum < target {
                b += 1;
                sum += data[b];
            }
            if sum > target {
                sum -= data[a];
                a += 1;
            }
        }
        data[a..=b].iter().min().unwrap() + data[a..=b].iter().max().unwrap()
    }
}

fn parse(contents: &str) -> Vec<u64> {
    contents.lines().map(|x| x.parse().unwrap()).collect()
}
