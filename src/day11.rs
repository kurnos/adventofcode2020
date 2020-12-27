use crate::infra::Problem;
use std::convert::TryInto;

pub struct Day11;

const FLOOR: i8 = 32;
const TAKEN: i8 = 16;

impl Problem<String, String, usize, usize> for Day11 {
    fn day() -> u8 {
        11
    }

    fn first(contents: String) -> usize {
        let (dim, map) = parse(&contents);
        let neighbours = neighbour_list(&map, |pos: usize, dx: i8, dy: i8| {
            let (x, y) = (
                (pos % dim as usize) as i8 + dx,
                (pos / dim as usize) as i8 + dy,
            );
            if 0 <= x && x < dim && 0 <= y && y < dim {
                x as u16 + y as u16 * dim as u16
            } else {
                0xffff
            }
        });
        room_of_life(map, 4, &neighbours)
    }

    fn second(contents: String) -> usize {
        let (dim, map) = parse(&contents);
        let neighbours = neighbour_list(&map, |pos: usize, dx: i8, dy: i8| {
            let (mut x, mut y) = ((pos % dim as usize) as i8, (pos / dim as usize) as i8);
            loop {
                x += dx;
                y += dy;

                if 0 <= x && x < dim && 0 <= y && y < dim {
                    match map[x as usize + y as usize * dim as usize] {
                        FLOOR => {}
                        _ => {
                            return x as u16 + y as u16 * dim as u16;
                        }
                    }
                } else {
                    return 0xffff;
                }
            }
        });
        room_of_life(map, 5, &neighbours)
    }
}

fn neighbour_list(map: &[i8], can_see_in_dir: impl Fn(usize, i8, i8) -> u16) -> Vec<[u16; 8]> {
    (0..map.len())
        .map(|pos| {
            [
                can_see_in_dir(pos, -1, -1),
                can_see_in_dir(pos, -1, 0),
                can_see_in_dir(pos, -1, 1),
                can_see_in_dir(pos, 0, -1),
                can_see_in_dir(pos, 0, 1),
                can_see_in_dir(pos, 1, -1),
                can_see_in_dir(pos, 1, 0),
                can_see_in_dir(pos, 1, 1),
            ]
        })
        .collect()
}

fn room_of_life(mut counts: Vec<i8>, limit: i8, neighbours: &[[u16; 8]]) -> usize {
    let mut queue = vec![];
    loop {
        for i in queue.drain(..) {
            if counts[i] & TAKEN != 0 {
                for &n in &neighbours[i] {
                    if n != 0xffff {
                        counts[n as usize] += 1;
                    }
                }
            } else {
                for &n in &neighbours[i] {
                    if n != 0xffff {
                        counts[n as usize] -= 1;
                    }
                }
            }
        }
        for (i, c) in counts.iter_mut().enumerate() {
            if *c == 0 || (*c & TAKEN != 0 && *c & (TAKEN - 1) >= limit) {
                queue.push(i);
                *c ^= TAKEN;
            }
        }
        if queue.is_empty() {
            break counts.into_iter().filter(|&b| b & 16 != 0).count();
        }
    }
}

fn parse(contents: &str) -> (i8, Vec<i8>) {
    let mut width = contents.lines().next().unwrap().len();

    let mut res = vec![];
    for line in contents.lines() {
        for (col, b) in line.bytes().enumerate() {
            width = std::cmp::max(width, col);
            res.push(if b == b'.' { FLOOR } else { 0 });
        }
    }

    (width.try_into().unwrap(), res)
}
