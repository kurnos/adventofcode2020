use crate::infra::Problem;
use itertools::{iproduct, Itertools};
use nalgebra::{Point3, Point4, Vector3, Vector4};
use std::collections::HashSet;

pub struct Day17;

impl Problem<String, String, usize, usize> for Day17 {
    fn day() -> u8 {
        17
    }

    fn first(contents: String) -> usize {
        let mut data: HashSet<Point3<i16>> = contents
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.bytes()
                    .enumerate()
                    .map(move |(x, line)| (Point3::new(x as i16, y as i16, 0), line))
            })
            .filter_map(|(p, b)| if b == b'#' { Some(p) } else { None })
            .collect();

        let mut new_data = data.clone();

        for _ in 0..6 {
            let dx = data.iter().map(|p| p.x).minmax().into_option().unwrap();
            let dy = data.iter().map(|p| p.y).minmax().into_option().unwrap();
            let dz = data.iter().map(|p| p.z).minmax().into_option().unwrap();
            for x in dx.0 - 1..=dx.1 + 1 {
                for y in dy.0 - 1..=dy.1 + 1 {
                    for z in dz.0 - 1..=dz.1 + 1 {
                        let p = Point3::new(x, y, z);
                        let a = data.contains(&p);
                        let n_count = ncount3(&data, &p);
                        if a && (n_count == 2 || n_count == 3) || !a && n_count == 3 {
                            new_data.insert(p);
                        } else {
                            new_data.remove(&p);
                        }
                    }
                }
            }
            std::mem::swap(&mut data, &mut new_data);
            new_data.clear();
        }
        data.len()
    }

    fn second(contents: String) -> usize {
        let ns: Vec<Vector4<i32>> = iproduct!(-1..=1, -1..=1, -1..=1, -1..=1)
            .filter(|&(x, y, z, w)| !(x == 0 && y == 0 && z == 0 && w == 0))
            .map(|v| Vector4::new(v.0, v.1, v.2, v.3))
            .collect_vec();
        assert_eq!(ns.len(), 80);

        let mut data: HashSet<Point4<i32>> = contents
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.bytes()
                    .enumerate()
                    .map(move |(x, line)| (Point4::new(x as i32, y as i32, 0, 0), line))
            })
            .filter_map(|(p, b)| if b == b'#' { Some(p) } else { None })
            .collect();

        let mut new_data = data.clone();

        for _ in 0..6 {
            let dx = data.iter().map(|p| p.x).minmax().into_option().unwrap();
            let dy = data.iter().map(|p| p.y).minmax().into_option().unwrap();
            let dz = data.iter().map(|p| p.z).minmax().into_option().unwrap();
            let dw = data.iter().map(|p| p.w).minmax().into_option().unwrap();
            for x in dx.0 - 1..=dx.1 + 1 {
                for y in dy.0 - 1..=dy.1 + 1 {
                    for z in dz.0 - 1..=dz.1 + 1 {
                        for w in dw.0 - 1..=dw.1 + 1 {
                            let p = Point4::new(x, y, z, w);
                            let a = data.contains(&p);
                            let n_count = ncount4(&data, &p);
                            if a && (n_count == 2 || n_count == 3) || !a && n_count == 3 {
                                new_data.insert(p);
                            } else {
                                new_data.remove(&p);
                            }
                        }
                    }
                }
            }
            std::mem::swap(&mut data, &mut new_data);
            new_data.clear();
        }
        data.len()
    }
}

fn ncount3(board: &HashSet<Point3<i16>>, p: &Point3<i16>) -> usize {
    let x = &[0i16, 1, -1];
    iproduct!(x, x, x)
        .skip(1)
        .map(|(&x, &y, &z)| Vector3::new(x, y, z))
        .filter(|d| board.contains(&(*p + *d)))
        .count()
}

fn ncount4(board: &HashSet<Point4<i32>>, p: &Point4<i32>) -> usize {
    let x = &[0, 1, -1];
    iproduct!(x, x, x, x)
        .skip(1)
        .map(|(&x, &y, &z, &w)| Vector4::new(x, y, z, w))
        .filter(|d| board.contains(&(*p + *d)))
        .count()
}
