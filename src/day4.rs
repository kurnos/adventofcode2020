use crate::infra::Problem;
use regex::Regex;
use std::collections::HashMap;

pub struct Day4;

impl Problem<String, String, usize, usize> for Day4 {
    fn day() -> u8 {
        4
    }
    fn first(contents: String) -> usize {
        contents
            .split("\r\n\r\n")
            .map(|p| {
                p.split_whitespace()
                    .map(|x| x.split(':').take(2).collect::<Vec<_>>())
                    .map(|v| (v[0], v[1]))
                    .collect::<HashMap<_, _>>()
            })
            .filter(|m| m.len() == 8 || m.len() == 7 && !m.contains_key("cid"))
            .count()
    }
    fn second(contents: String) -> usize {
        let hcl_regex = Regex::new("^#[0-9a-f]{6}$").unwrap();
        let pid_regex = Regex::new("^[0-9]{9}$").unwrap();
        let passports = contents
            .split("\r\n\r\n")
            .map(|p| {
                p.split_whitespace()
                    .map(|x| x.split(':').take(2).collect::<Vec<_>>())
                    .map(|v| (v[0], v[1]))
                    .collect::<HashMap<_, _>>()
            })
            .map(|m| -> Option<bool> {
                Some(
                    {
                        // byr (Birth Year) - four digits; at least 1920 and at most 2002.
                        let byr = m.get("byr")?.parse::<u32>().ok()?;
                        byr >= 1920 && byr <= 2002
                    } && {
                        // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
                        let iyr = m.get("iyr")?.parse::<u32>().ok()?;
                        iyr >= 2010 && iyr <= 2020
                    } && {
                        // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
                        let eyr = m.get("eyr")?.parse::<u32>().ok()?;
                        eyr >= 2020 && eyr <= 2030
                    } && {
                        // hgt (Height) - a number followed by either cm or in:
                        //     If cm, the number must be at least 150 and at most 193.
                        //     If in, the number must be at least 59 and at most 76.
                        if let Some(hgt) = m.get("hgt")?.strip_suffix("cm") {
                            let hgt = hgt.parse::<u32>().ok()?;
                            hgt >= 150 && hgt <= 193
                        } else if let Some(hgt) = m.get("hgt")?.strip_suffix("in") {
                            let hgt = hgt.parse::<u32>().ok()?;
                            hgt >= 59 && hgt <= 76
                        } else {
                            false
                        }
                    } && {
                        // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
                        hcl_regex.is_match(m.get("hcl")?)
                    } && {
                        // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
                        match *m.get("ecl")? {
                            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                            _ => false,
                        }
                    } && {
                        // pid (Passport ID) - a nine-digit number, including leading zeroes.
                        pid_regex.is_match(m.get("pid")?)
                    },
                )
            })
            .filter(|x| x.unwrap_or(false))
            .collect::<Vec<_>>();
        passports.len()
    }
}
