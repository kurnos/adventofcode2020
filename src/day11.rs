use crate::infra::Problem;
use std::convert::TryInto;

pub struct Day11;

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
                        b'#' | b'L' => {
                            return x as u16 + y as u16 * dim as u16;
                        }
                        _ => {}
                    }
                } else {
                    return 0xffff;
                }
            }
        });
        room_of_life(map, 5, &neighbours)
    }
}

fn neighbour_list(map: &[u8], can_see_in_dir: impl Fn(usize, i8, i8) -> u16) -> Vec<[u16; 8]> {
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

fn count(map: &[u8], neighbours: &[u16; 8]) -> u8 {
    let mut res = 0;
    for &p in neighbours {
        if p != 0xffff && map[p as usize] == b'#' {
            res += 1;
        }
    }
    res
}

fn room_of_life(map: Vec<u8>, limit: u8, neighbours: &[[u16; 8]]) -> usize {
    let mut changed = true;
    let (mut tick, mut tock) = (map.clone(), map);
    while changed {
        changed = false;
        for (p, &b) in tick.iter().enumerate() {
            tock[p as usize] = match b {
                b'L' if count(&tick, &neighbours[p]) == 0 => {
                    changed = true;
                    b'#'
                }
                b'#' if count(&tick, &neighbours[p]) >= limit => {
                    changed = true;
                    b'L'
                }
                t => t,
            };
        }
        {
            let tmp = tock;
            tock = tick;
            tick = tmp;
        }
    }
    tock.into_iter().filter(|&b| b == b'#').count()
}

fn parse(contents: &str) -> (i8, Vec<u8>) {
    let mut width = contents.lines().next().unwrap().len();

    let mut res = vec![];
    for line in contents.lines() {
        for (col, b) in line.bytes().enumerate() {
            width = std::cmp::max(width, col);
            res.push(b);
        }
    }

    (width.try_into().unwrap(), res)
}
