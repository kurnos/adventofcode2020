use crate::infra::Problem;
use itertools::Itertools;

pub struct Day13;

impl Problem<String, String, u32, i64> for Day13 {
    fn day() -> u8 {
        13
    }

    fn first(contents: String) -> u32 {
        let mut data = contents.lines();

        let a = data.next().unwrap().parse::<u32>().unwrap();
        let b = data
            .next()
            .unwrap()
            .split(',')
            .filter_map(|x| x.parse::<u32>().ok())
            .collect_vec();

        let x = b.iter().map(|&id| a - a % id + id).collect_vec();
        let p = x.iter().position_min_by_key(|&&x| x).unwrap();
        &b[p] * (&x[p] - a)
    }

    fn second(contents: String) -> i64 {
        let mut data = contents.lines();

        let b = data
            .nth(1)
            .unwrap()
            .split(',')
            .enumerate()
            .filter_map(|x| x.1.parse::<i64>().ok().map(|t| (x.0 as i64, t)))
            .collect_vec();

        let mut offset = 0;
        let mut mul = 1;

        for &(t, id) in &b[..] {
            while (offset + t) % id != 0 {
                offset += mul;
            }
            mul *= id;
        }
        offset
    }
}
