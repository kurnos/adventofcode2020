use crate::infra::Problem;

pub struct Day3;

impl Problem<String, String, usize, usize> for Day3 {
    fn day() -> u8 {
        3
    }
    fn first(contents: String) -> usize {
        contents
            .lines()
            .map(|x| x.as_bytes())
            .enumerate()
            .filter(|(i, trees)| trees[i * 3 % trees.len()] == b'#')
            .count()
    }
    fn second(contents: String) -> usize {
        [(1usize, 1usize), (1, 3), (1, 5), (1, 7), (2, 1)]
            .iter()
            .map(|(down, right)| {
                contents
                    .lines()
                    .step_by(*down)
                    .map(|x| x.as_bytes())
                    .enumerate()
                    .filter(|(i, trees)| trees[i * right % trees.len()] == b'#')
                    .count()
            })
            .product()
    }
}
