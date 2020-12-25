use itertools::Itertools;

use crate::infra::Problem;

pub struct Day25;

impl Problem<String, String, u64, &str> for Day25 {
    fn day() -> u8 {
        25
    }

    fn first(contents: String) -> u64 {
        let (a, b) = contents
            .lines()
            .map(|x| x.parse::<u64>().unwrap())
            .collect_tuple()
            .unwrap();

        const MOD: u64 = 20201227;

        let mut n = 1;
        for i in 1..MOD {
            n = (n * 7) % MOD;
            if a == n {
                n = 1;
                for _ in 0..i {
                    n = (n * b) % MOD;
                }
                return n;
            }
            if b == n {
                n = 1;
                for _ in 0..i {
                    n = (n * a) % MOD;
                }
                return n;
            }
        }
        return 0;
    }

    fn second(_contents: String) -> &'static str {
        "That's all folks"
    }
}
