use crate::infra::Problem;

pub struct Day12;

impl Problem<String, String, i16, i16> for Day12 {
    fn day() -> u8 {
        12
    }

    fn first(contents: String) -> i16 {
        let mut pos = (0, 0);
        let mut dir = (1, 0);

        for cmd in contents
            .lines()
            .map(|line| (line.as_bytes()[0], line[1..].parse::<i16>().unwrap()))
        {
            match cmd {
                (b'F', d) => pos = add(pos, scale(dir, d)),
                (b'R', 90) | (b'L', 270) => dir = cw(dir),
                (b'R', 180) | (b'L', 180) => dir = scale(dir, -1),
                (b'R', 270) | (b'L', 90) => dir = ccw(dir),
                (b'N', d) => pos.1 += d,
                (b'E', d) => pos.0 += d,
                (b'W', d) => pos.0 -= d,
                (b'S', d) => pos.1 -= d,
                _ => panic!(),
            };
        }
        pos.0.abs() + pos.1.abs()
    }

    fn second(contents: String) -> i16 {
        let mut pos = (0, 0);
        let mut way = (10, 1);

        for cmd in contents
            .lines()
            .map(|line| (line.as_bytes()[0], line[1..].parse::<i16>().unwrap()))
        {
            match cmd {
                (b'F', d) => pos = add(pos, scale(way, d)),
                (b'R', 90) | (b'L', 270) => way = cw(way),
                (b'R', 180) | (b'L', 180) => way = scale(way, -1),
                (b'R', 270) | (b'L', 90) => way = ccw(way),
                (b'N', d) => way.1 += d,
                (b'E', d) => way.0 += d,
                (b'W', d) => way.0 -= d,
                (b'S', d) => way.1 -= d,
                _ => panic!(),
            };
        }
        pos.0.abs() + pos.1.abs()
    }
}

fn scale(v: (i16, i16), m: i16) -> (i16, i16) {
    (v.0 * m, v.1 * m)
}

fn cw(v: (i16, i16)) -> (i16, i16) {
    (v.1, -v.0)
}

fn ccw(v: (i16, i16)) -> (i16, i16) {
    (-v.1, v.0)
}

fn add(v1: (i16, i16), v2: (i16, i16)) -> (i16, i16) {
    (v1.0 + v2.0, v1.1 + v2.1)
}
