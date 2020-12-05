use crate::infra::Problem;
use std::convert::TryInto;

pub struct Day5;

impl Problem<String, String, u16, u16> for Day5 {
    fn day() -> u8 {
        5
    }

    fn first(contents: String) -> u16 {
        contents.lines().map(parse).max().unwrap()
    }

    fn second(contents: String) -> u16 {
        let mut xs = contents.lines().map(parse).collect::<Vec<_>>();
        xs.sort();
        for i in 1..xs.len() {
            if xs[i] - xs[i - 1] == 2 {
                return xs[i] - 1;
            }
        }
        0
    }
}

fn parse(spec: &str) -> u16 {
    let b: &[u8; 10] = spec.as_bytes().try_into().unwrap();
    ((b[0] == b'B') as u16 * 512)
        | ((b[1] == b'B') as u16 * 256)
        | ((b[2] == b'B') as u16 * 128)
        | ((b[3] == b'B') as u16 * 64)
        | ((b[4] == b'B') as u16 * 32)
        | ((b[5] == b'B') as u16 * 16)
        | ((b[6] == b'B') as u16 * 8)
        | ((b[7] == b'R') as u16 * 4)
        | ((b[8] == b'R') as u16 * 2)
        | ((b[9] == b'R') as u16 * 1)
}
