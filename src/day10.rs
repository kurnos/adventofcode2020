use crate::infra::Problem;
use itertools::Itertools;

pub struct Day10;

impl Problem<String, String, u16, usize> for Day10 {
    fn day() -> u8 {
        10
    }

    fn first(contents: String) -> u16 {
        let mut data = parse(&contents);
        data.push(0);
        data.sort_unstable();
        data.push(data[data.len() - 1] + 3);
        let mut counts = [0; 4];
        for (a, b) in data.into_iter().tuple_windows() {
            counts[(b - a) as usize] += 1;
        }
        counts[1] * counts[3]
    }

    fn second(contents: String) -> usize {
        let mut data = parse(&contents);
        data.sort_unstable();
        let target = data[data.len() - 1] + 3;
        data.push(target);

        let mut counts2 = vec![(0, 1)];

        for d in data {
            counts2.push((
                d,
                counts2
                    .iter()
                    .rev()
                    .take_while(|(n, _)| d - n <= 3)
                    .map(|(_, c)| c)
                    .sum(),
            ));
        }
        counts2[counts2.len() - 1].1
    }
}

fn parse(contents: &str) -> Vec<i16> {
    contents.lines().map(|x| x.parse().unwrap()).collect()
}
