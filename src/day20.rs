use crate::infra::Problem;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub struct Day20;

impl Problem<String, String, u128, usize> for Day20 {
    fn day() -> u8 {
        20
    }

    fn first(contents: String) -> u128 {
        let tiles = contents
            .split("\n\n")
            .map(|tile| parse_tile(tile))
            .collect_vec();

        let dim = (tiles.len() as f64).powf(0.5).round() as i8;
        let solution = find_mapping(dim, &tiles);

        let mut res = 1;
        for &x in &[0, dim - 1] {
            for &y in &[0, dim - 1] {
                res *= tiles[solution.get(&(x, y)).unwrap().0].0 as u128;
            }
        }
        res
    }

    fn second(contents: String) -> usize {
        let tiles = contents
            .split("\n\n")
            .map(|tile| parse_tile(tile))
            .collect_vec();

        let dim = (tiles.len() as f64).powf(0.5).round() as i8;

        let solution = find_mapping(dim, &tiles);

        let img = make_image(dim, &tiles, solution);
        let img = img.iter().map(|s| -> &str { s }).collect_vec();

        let img_dim = img.len() as i8;

        let mut all_seen = HashSet::new();
        for r in 0..4 {
            for &f in [false, true].iter() {
                let coords = asdf(&transform(&img, (r, f)));
                for p in coords {
                    all_seen.insert(transform_coord(img_dim, &p, (3 - r, f)));
                }
            }
        }

        img.iter()
            .map(|r| r.chars().filter(|c| *c == '#').count())
            .sum::<usize>()
            - all_seen.len()
    }
}

type Solution = HashMap<(i8, i8), (usize, Orientation)>;

fn find_mapping(dim: i8, tiles: &[(u32, Vec<&str>)]) -> Solution {
    let edges = tiles
        .iter()
        .map(|(_, t)| [edge(t, 0), edge(t, 1), edge(t, 2), edge(t, 3)])
        .collect_vec();

    find(dim, &edges, &mut HashMap::new()).unwrap()
}

fn transform_coord(img_dim: i8, tile: &(i8, i8), o: Orientation) -> (i8, i8) {
    let mut t = *tile;
    if o.1 {
        t.0 = img_dim - t.0 - 1;
    }
    for _ in 0..(4 - o.0) {
        t = (t.1, img_dim - t.0 - 1);
    }
    t
}

fn asdf(img: &[String]) -> HashSet<(i8, i8)> {
    let s = vec![
        "                  # ",
        "#    ##    ##    ###",
        " #  #  #  #  #  #   ",
    ];
    let pos: Vec<(usize, usize)> = s
        .iter()
        .enumerate()
        .flat_map(|(y, s)| {
            s.chars()
                .enumerate()
                .filter_map(move |(x, c)| if c == '#' { Some((x, y)) } else { None })
        })
        .collect();
    let mut res = HashSet::new();
    for dx in 0..(img[0].len() - s[0].len()) {
        for dy in 0..(img.len() - s.len()) {
            if pos
                .iter()
                .all(|(x, y)| img[y + dy].chars().nth(x + dx).unwrap() == '#')
            {
                pos.iter().for_each(|(x, y)| {
                    res.insert(((x + dx) as i8, (y + dy) as i8));
                });
            }
        }
    }
    res
}

fn make_image(dim: i8, tiles: &[(u32, Vec<&str>)], solution: Solution) -> Vec<String> {
    let solution = solution
        .into_iter()
        .map(|(k, v)| (k, (tiles[v.0 as usize].1.clone(), v.1)))
        .collect::<HashMap<_, _>>();

    let mut res = vec![];
    for y in 0..dim {
        let mut lines = vec![String::new(); 8];
        for x in 0..dim {
            let s = solution.get(&(x, y)).unwrap();
            let t = transform(&s.0, s.1);
            for dy in 1..9 {
                lines[dy - 1].push_str(&t[dy][1..9]);
            }
        }
        for line in lines {
            res.push(line);
        }
    }
    res
}

fn transform(tile: &[&str], o: Orientation) -> Vec<String> {
    let mut t = tile.iter().map(|s| s.to_string()).collect_vec();
    if o.1 {
        t = flip_x(&t);
    }
    for _ in 0..(4 - o.0) {
        t = rotate_ccw(&t);
    }
    t
}

fn rotate_ccw(t: &[String]) -> Vec<String> {
    let d = t[0].len();
    let mut res = vec![];
    for y in 0..d {
        let mut line = String::new();
        for asdf in t {
            line.push_str(&asdf[d - y - 1..d - y]);
        }
        res.push(line);
    }
    res
}

fn flip_x(t: &[String]) -> Vec<String> {
    let mut res = vec![];
    for t in t.iter() {
        res.push(t.chars().rev().collect());
    }
    res
}

fn find(dim: i8, tiles: &[[u16; 4]], so_far: &mut Solution) -> Option<Solution> {
    if let Some(pos) = next_pos(dim, so_far) {
        for opt in options(pos, tiles, so_far) {
            so_far.insert(pos, opt);
            if let Some(solution) = find(dim, tiles, so_far) {
                return Some(solution);
            }
            so_far.remove(&pos);
        }
        None
    } else {
        Some(so_far.clone())
    }
}

fn options<'a>(
    pos: (i8, i8),
    tiles: &'a [[u16; 4]],
    so_far: &Solution,
) -> impl Iterator<Item = (usize, Orientation)> + 'a {
    let taken = so_far.values().map(|x| x.0).collect::<HashSet<_>>();

    let above = so_far
        .get(&(pos.0, pos.1 - 1))
        .map(|&(i, o)| (&tiles[i], o));
    let left = so_far
        .get(&(pos.0 - 1, pos.1))
        .map(|&(i, o)| (&tiles[i], o));

    tiles
        .iter()
        .enumerate()
        .filter(move |(i, _)| !taken.contains(i))
        .flat_map(move |(i, tile)| {
            (0..4).flat_map(move |r| {
                [true, false].iter().filter_map(move |&f| {
                    let t = (tile, (r, f));
                    if above.map_or(true, |a| above_below(a, t))
                        && left.map_or(true, |a| left_right(a, t))
                    {
                        Some((i, (r, f)))
                    } else {
                        None
                    }
                })
            })
        })
}

fn above_below(above: (&[u16; 4], Orientation), below: (&[u16; 4], Orientation)) -> bool {
    let (a, o1) = above;
    let (b, o2) = below;

    let e1 = (if !o1.1 { 6 - o1.0 } else { 6 + o1.0 } % 4) as usize;
    let e2 = (if !o2.1 { 4 - o2.0 } else { 4 + o2.0 } % 4) as usize;
    if o1.1 != o2.1 {
        a[e1] == b[e2]
    } else {
        bit_reverse(a[e1]) == b[e2]
    }
}

fn left_right(left: (&[u16; 4], Orientation), right: (&[u16; 4], Orientation)) -> bool {
    let (l, o1) = left;
    let (r, o2) = right;

    let e1 = (if !o1.1 { 5 - o1.0 } else { 7 + o1.0 } % 4) as usize;
    let e2 = (if !o2.1 { 7 - o2.0 } else { 5 + o2.0 } % 4) as usize;
    if o1.1 != o2.1 {
        l[e1] == r[e2]
    } else {
        bit_reverse(l[e1]) == r[e2]
    }
}

fn bit_reverse(mut n: u16) -> u16 {
    let mut res = 0;
    for _ in 0..10 {
        res = (res << 1) | (n & 1);
        n >>= 1;
    }
    res
}

fn collect_bits(it: impl Iterator<Item = char>) -> u16 {
    it.fold(0, |a, b| (a << 1) | ((b == '#') as u16))
}

fn edge(t: &[&str], n: u8) -> u16 {
    match n {
        0 => collect_bits(t[0].chars()),
        1 => collect_bits((0..10).map(|i| t[i].chars().nth(9).unwrap())),
        2 => collect_bits(t[9].chars().rev()),
        3 => collect_bits((0..10).map(|i| t[i].chars().next().unwrap()).rev()),
        _ => panic!(),
    }
}

fn next_pos(dim: i8, xs: &Solution) -> Option<(i8, i8)> {
    for y in 0..dim {
        for x in 0..dim {
            if xs.get(&(x, y)).is_none() {
                return Some((x, y));
            }
        }
    }
    None
}

type Orientation = (u8, bool);

fn parse_tile(s: &str) -> (u32, Vec<&str>) {
    let mut lines = s.lines();
    let n_s = lines.next().unwrap();
    let n = n_s["Tile ".len()..n_s.len() - 1].parse().unwrap();
    let squares = lines.collect();
    (n, squares)
}
