use crate::infra::Problem;

pub struct Day4;

impl Problem<String, String, usize, usize> for Day4 {
    fn day() -> u8 {
        4
    }
    fn first(contents: String) -> usize {
        do_it(contents, |k, _| match k {
            b"byr" => 0b0000001,
            b"iyr" => 0b0000010,
            b"eyr" => 0b0000100,
            b"hgt" => 0b0001000,
            b"hcl" => 0b0010000,
            b"ecl" => 0b0100000,
            b"pid" => 0b1000000,
            _ => 0,
        })
    }
    fn second(contents: String) -> usize {
        do_it(contents, |k, v| match k {
            b"byr" if btwn(v, b"1920", b"2002") => 0b0000001,
            b"iyr" if btwn(v, b"2010", b"2020") => 0b0000010,
            b"eyr" if btwn(v, b"2020", b"2030") => 0b0000100,
            b"hgt" if v.ends_with(b"cm") && btwn(v, b"150cm", b"193cm") => 0b0001000,
            b"hgt" if v.ends_with(b"in") && btwn(v, b"59cm", b"76cm") => 0b0001000,
            b"hcl"
                if v[0] == b'#'
                    && v.len() == 7
                    && v[1..]
                        .iter()
                        .all(|&b| btwn(b, b'0', b'9') || btwn(b, b'a', b'f')) =>
            {
                0b0010000
            }
            b"ecl" => match v {
                b"amb" | b"blu" | b"brn" | b"gry" | b"grn" | b"hzl" | b"oth" => 0b0100000,
                _ => 0,
            },
            b"pid" if v.len() == 9 && v.iter().all(|&b| btwn(b, b'0', b'9')) => 0b1000000,
            _ => 0,
        })
    }
}

fn btwn<T1: PartialOrd<T2>, T2>(v: T1, min: T2, max: T2) -> bool {
    v >= min && v <= max
}

#[derive(Debug)]
enum S<'a> {
    Key(usize),
    Value(&'a [u8], usize),
    Space,
    NewLine,
}

#[allow(clippy::many_single_char_names)]
fn do_it<F: Fn(&[u8], &[u8]) -> u8>(contents: String, f: F) -> usize {
    let bytes = contents.as_bytes();

    let mut s = S::Key(0);
    let mut m = 0;
    let mut r = 0;
    for (i, b) in bytes.iter().enumerate() {
        s = match (s, b) {
            (S::Key(s), b':') => S::Value(&bytes[s..i], i + 1),
            (S::Value(k, s), b' ') => {
                m |= f(k, &bytes[s..i]);
                S::Space
            }
            (S::Value(k, s), b'\n') => {
                m |= f(k, &bytes[s..i]);
                S::NewLine
            }
            (s @ S::Key(_), _) | (s @ S::Value(_, _), _) => s,
            (S::Space, b'\n') => S::NewLine,
            (S::NewLine, b' ') | (S::Space, b' ') => S::Space,
            (S::NewLine, b'\n') => {
                if (m & 0b1111111) == 0b1111111 {
                    r += 1
                }
                m = 0;
                S::NewLine
            }
            (S::Space, _) | (S::NewLine, _) => S::Key(i),
        }
    }
    if (m & 0b1111111) == 0b1111111 {
        return r + 1;
    }
    r
}
