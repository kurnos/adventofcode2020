use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub struct Day21;

impl crate::infra::Problem<String, String, usize, String> for Day21 {
    fn day() -> u8 {
        21
    }

    fn first(contents: String) -> usize {
        let data = contents.lines().map(parse_line).collect_vec();

        let known_ingredients = get_choices(&data)
            .into_iter()
            .flat_map(|x| x.1.into_iter())
            .collect::<HashSet<_>>();
        data.into_iter()
            .flat_map(|(x, _)| x.into_iter())
            .filter(|i| !known_ingredients.contains(i))
            .count()
    }

    fn second(contents: String) -> String {
        let data = contents.lines().map(parse_line).collect_vec();
        let choices = get_choices(&data);
        find(&choices, &mut HashMap::new())
            .unwrap()
            .into_iter()
            .sorted_by_key(|x| x.0)
            .map(|x| x.1)
            .join(",")
    }
}

fn get_choices<'a>(ingredients: &[(Vec<&'a str>, Vec<&'a str>)]) -> HashMap<&'a str, Vec<&'a str>> {
    let mut res: HashMap<&str, Vec<&str>> = HashMap::new();
    for (ii, aa) in ingredients.iter() {
        for &a in aa {
            if let Some(c) = res.get_mut(a) {
                let mut i = 0;
                while i < c.len() {
                    if !(ii.contains(&c[i])) {
                        c.swap_remove(i);
                    } else {
                        i += 1;
                    }
                }
            } else {
                res.insert(a, ii.clone());
            }
        }
    }
    res
}

fn find<'a>(
    choices: &HashMap<&'a str, Vec<&'a str>>,
    so_far: &mut HashMap<&'a str, &'a str>,
) -> Option<HashMap<&'a str, &'a str>> {
    if let Some((a, ii)) = choices.iter().find(|x| !so_far.contains_key(x.0)) {
        for i in ii.iter() {
            if !so_far.values().any(|x| x == i) {
                so_far.insert(a, i);
                if let Some(solution) = find(choices, so_far) {
                    return Some(solution);
                }
                so_far.remove(a);
            }
        }
        None
    } else {
        Some(so_far.clone())
    }
}

fn parse_line(s: &str) -> (Vec<&str>, Vec<&str>) {
    let (a, b) = s.splitn(2, " (contains ").collect_tuple().unwrap();

    let b = &b[0..(b.len() - 1)];

    (a.split(' ').collect(), b.split(", ").collect())
}
