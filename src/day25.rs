use itertools::Itertools;

use crate::infra::Problem;

pub struct Day25;

impl Problem<String, Option<()>, u64, Option<()>, 25> for Day25 {
    fn first(contents: String) -> u64 {
        let (t1, t2) = contents
            .lines()
            .map(|x| x.parse::<u64>().unwrap())
            .collect_tuple()
            .unwrap();

        const MOD: u64 = 20201227;

        let mut i = 0;
        let mut n = 1;
        let (b, e) = loop {
            if t1 == n {
                break (t2, i);
            } else if t1 == (7 * n) % MOD {
                break (t2, i + 1);
            }
            if t2 == n {
                break (t1, i);
            } else if t2 == (7 * n) % MOD {
                break (t1, i + 1);
            }
            n = (n * 7 * 7) % MOD;
            i += 2;
        };
        pow_mod(b, e, MOD)
    }

    fn second(_contents: Option<()>) -> Option<()> {
        None
    }
}

fn pow_mod(b: u64, mut e: u64, m: u64) -> u64 {
    let mut res = 1;
    for _ in 0..64 {
        e = e.rotate_left(1);
        res = (res * res) % m;
        if (e & 1) != 0 {
            res = (res * b) % m
        }
    }
    res
}
