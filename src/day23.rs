use std::collections::VecDeque;
use std::iter::FromIterator;

use itertools::Itertools;

pub struct Day23;

impl crate::infra::Problem<&str, &str, String, u64> for Day23 {
    fn day() -> u8 {
        23
    }

    fn first(contents: &str) -> String {
        let data = contents.bytes().map(|b| (b - b'0') as u32);
        let mut cups = VecDeque::from_iter(data);

        for _ in 1..=100 {
            let (c, cs) = find_dest(&mut cups).unwrap();
            let d = dest(c, cs, 9);
            while cups[0] != d {
                cups.rotate_left(1);
            }
            let a = cups.pop_front().unwrap();
            cups.push_front(cs[2]);
            cups.push_front(cs[1]);
            cups.push_front(cs[0]);
            cups.push_front(a);
            while cups[0] != c {
                cups.rotate_right(1);
            }
            cups.rotate_left(1);
        }

        while cups[0] != 1 {
            cups.rotate_left(1);
        }
        cups.pop_front();
        let res =
            String::from_utf8(cups.into_iter().map(|x| x as u8 + b'0').collect_vec()).unwrap();
        res
    }

    fn second(contents: &str) -> u64 {
        const MAX: u32 = 1000000;

        let data = contents.bytes().map(|b| (b - b'0') as u32).chain(10..=MAX);

        let mut before = vec![0; MAX as usize + 1];
        let mut after = vec![0; MAX as usize + 1];
        for (a, b) in data.tuple_windows() {
            before[b as usize] = a;
            after[a as usize] = b;
        }

        let mut current = (contents.as_bytes()[0] - b'0') as u32;
        after[MAX as usize] = current;
        before[current as usize] = MAX;

        for (a, &b) in after.iter().enumerate().skip(1) {
            assert_eq!(before[b as usize], a as u32);
        }
        for (a, &b) in before.iter().enumerate().skip(1) {
            assert_eq!(after[b as usize], a as u32);
        }

        for _ in 1..=10000000 {
            let a = after[current as usize];
            let b = after[a as usize];
            let c = after[b as usize];
            let dest = dest(current, [a, b, c], MAX);

            let tmp = after[c as usize];
            after[current as usize] = tmp;
            before[tmp as usize] = current;

            let tmp = after[dest as usize];
            after[dest as usize] = a;
            before[a as usize] = dest;
            after[c as usize] = tmp;
            before[tmp as usize] = c;

            current = after[current as usize];
        }
        let a = after[1];
        let b = after[a as usize];
        a as u64 * b as u64
    }
}

type Cups = VecDeque<u32>;

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

fn find_dest(cups: &mut Cups) -> Option<(u32, [u32; 3])> {
    let res = (
        cups.pop_front()?,
        [cups.pop_front()?, cups.pop_front()?, cups.pop_front()?],
    );
    cups.push_front(res.0);
    Some(res)
}
