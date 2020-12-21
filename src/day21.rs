use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

pub struct Day21;

impl crate::infra::Problem<String, String, usize, String> for Day21 {
    fn day() -> u8 {
        21
    }

    fn first(contents: String) -> usize {
        let data = contents.lines().map(parse_line).collect_vec();

        let all_allergens = data
            .iter()
            .flat_map(|(_, x)| x.iter().cloned())
            .collect::<HashSet<_>>();

        let i_by_a = all_allergens
            .iter()
            .map(|a| {
                (
                    *a,
                    data.iter()
                        .filter(|(_, aa)| aa.contains(a))
                        .map(|x| HashSet::from_iter(x.0.iter().cloned()))
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<HashMap<_, Vec<HashSet<&str>>>>();

        let mut choices = Vec::new();

        for (a, mut is) in i_by_a.into_iter() {
            let mut x = is.pop().unwrap();
            while let Some(z) = is.pop() {
                x = HashSet::from_iter(x.intersection(&z).cloned());
            }
            choices.push((a, x));
        }

        let m = find(&choices, &mut HashMap::new()).unwrap();

        let known_ingredients = m.values().cloned().collect::<HashSet<_>>();

        let all_ingredients = data
            .iter()
            .flat_map(|(x, _)| x.iter().cloned())
            .filter(|i| !known_ingredients.contains(i))
            .count();

        all_ingredients
    }

    fn second(contents: String) -> String {
        let data = contents.lines().map(parse_line).collect_vec();

        let all_allergens = data
            .iter()
            .flat_map(|(_, x)| x.iter().cloned())
            .collect::<HashSet<_>>();

        let i_by_a = all_allergens
            .iter()
            .map(|a| {
                (
                    *a,
                    data.iter()
                        .filter(|(_, aa)| aa.contains(a))
                        .map(|x| HashSet::from_iter(x.0.iter().cloned()))
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<HashMap<_, Vec<HashSet<&str>>>>();

        let mut choices = Vec::new();

        for (a, mut is) in i_by_a.into_iter() {
            let mut x = is.pop().unwrap();
            while let Some(z) = is.pop() {
                x = HashSet::from_iter(x.intersection(&z).cloned());
            }
            choices.push((a, x));
        }

        let m = find(&choices, &mut HashMap::new()).unwrap();

        let known_ingredients = m.values().cloned().collect::<HashSet<_>>();

        let all_ingredients = data
            .iter()
            .flat_map(|(x, _)| x.iter().cloned())
            .filter(|i| !known_ingredients.contains(i))
            .count();

        dbg!(all_ingredients);

        let mut m = m.into_iter().collect_vec();
        m.sort_by_key(|x| x.0);
        m.into_iter().map(|x| x.1).join(",")
    }
}

fn find<'a>(
    choices: &[(&'a str, HashSet<&'a str>)],
    so_far: &mut HashMap<&'a str, &'a str>,
) -> Option<HashMap<&'a str, &'a str>> {
    let a = choices.iter().find(|x| !so_far.contains_key(x.0));
    if let Some(a) = a {
        assert!(!so_far.contains_key(a.0));
        for i in a.1.iter() {
            if !so_far.values().any(|x| x == i) {
                so_far.insert(a.0, i);
                if let Some(solution) = find(choices, so_far) {
                    return Some(solution);
                }
                so_far.remove(a.0);
            }
        }
        None
    } else {
        Some(so_far.clone())
    }
}

fn parse_line(s: &str) -> (Vec<&str>, Vec<&str>) {
    let (a, b) = s.splitn(2, " (contains ").collect_tuple().unwrap();

    let b = b.strip_suffix(")").unwrap();

    (a.split(' ').collect(), b.split(", ").collect())
}
