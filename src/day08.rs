use crate::infra::Problem;

pub struct Day8;

impl Problem<String, String, i32, i32, 8> for Day8 {
    fn first(contents: String) -> i32 {
        runit(&parse(&contents)).unwrap_err()
    }

    fn second(contents: String) -> i32 {
        let mut data = parse(&contents);
        fn swap(data: &mut Vec<I>, i: usize) {
            data[i] = match data[i] {
                I::Nop(d) => I::Jmp(d),
                I::Jmp(d) => I::Nop(d),
                I::Acc(d) => I::Acc(d),
            }
        }

        for i in 0..data.len() {
            if let I::Acc(_) = data[i] {
            } else {
                swap(&mut data, i);
                if let Ok(acc) = runit(&data) {
                    return acc;
                }
                swap(&mut data, i);
            }
        }
        panic!();
    }
}

fn runit(data: &[I]) -> Result<i32, i32> {
    let mut seen = std::iter::repeat(false).take(data.len()).collect::<Vec<_>>();
    let mut acc = 0;
    let mut i = 0;

    while i < data.len() {
        if seen[i] {
            return Err(acc);
        } else {
            seen[i] = true;
        }

        match data[i] {
            I::Acc(d) => acc += d,
            I::Jmp(d) => i = (i as isize + d as isize) as usize - 1,
            _ => {}
        };
        i += 1;
    }
    Ok(acc)
}

#[derive(Debug)]
enum Never {}

#[derive(Debug, Clone)]
enum I {
    Jmp(i32),
    Acc(i32),
    Nop(i32),
}

impl std::str::FromStr for I {
    type Err = Never;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match &s[..3] {
            "jmp" => I::Jmp(s[4..].parse().unwrap()),
            "acc" => I::Acc(s[4..].parse().unwrap()),
            "nop" => I::Nop(s[4..].parse().unwrap()),
            _ => panic!(),
        })
    }
}

fn parse(contents: &str) -> Vec<I> {
    contents.lines().map(|line| line.parse().unwrap()).collect()
}
