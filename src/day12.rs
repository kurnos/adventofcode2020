use crate::infra::Problem;

use nalgebra::{Matrix2, Point2, Vector2};

pub struct Day12;

impl Problem<String, String, i16, i16> for Day12 {
    fn day() -> u8 {
        12
    }

    fn first(contents: String) -> i16 {
        let right = Matrix2::new(0i16, 1, -1, 0);
        let mut pos = Point2::new(0i16, 0);
        let mut dir = Vector2::new(1i16, 0);

        for cmd in contents
            .lines()
            .map(|line| (line.as_bytes()[0], line[1..].parse::<i16>().unwrap()))
        {
            match cmd {
                (b'F', d) => pos += dir * d,
                (b'R', 90) | (b'L', 270) => dir = right * dir,
                (b'R', 180) | (b'L', 180) => dir = -dir,
                (b'R', 270) | (b'L', 90) => dir = -right * dir,
                (b'N', d) => pos.y += d,
                (b'E', d) => pos.x += d,
                (b'W', d) => pos.x -= d,
                (b'S', d) => pos.y -= d,
                _ => panic!(),
            };
        }
        pos.x.abs() + pos.y.abs()
    }

    fn second(contents: String) -> i16 {
        let right = Matrix2::new(0i16, 1, -1, 0);
        let mut pos = Point2::new(0i16, 0);
        let mut way = Vector2::new(10i16, 1);

        for cmd in contents
            .lines()
            .map(|line| (line.as_bytes()[0], line[1..].parse::<i16>().unwrap()))
        {
            match cmd {
                (b'F', d) => pos += way * d,
                (b'R', 90) | (b'L', 270) => way = right * way,
                (b'R', 180) | (b'L', 180) => way = -way,
                (b'R', 270) | (b'L', 90) => way = -right * way,
                (b'N', d) => way.y += d,
                (b'E', d) => way.x += d,
                (b'W', d) => way.x -= d,
                (b'S', d) => way.y -= d,
                _ => panic!(),
            };
        }
        pos.x.abs() + pos.y.abs()
    }
}
