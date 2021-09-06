use crate::infra::Problem;

pub struct Day18;

impl Problem<String, String, u64, u64> for Day18 {
    fn day() -> u8 {
        18
    }

    fn first(contents: String) -> u64 {
        contents
            .lines()
            .map(|line| shunting_yard(|_| 0, tokenize(line)))
            .sum()
    }

    fn second(contents: String) -> u64 {
        contents
            .lines()
            .map(|line| shunting_yard(|c| if c == '+' { 1 } else { 0 }, tokenize(line)))
            .sum()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum T {
    Op(char),
    Num(u32),
}

fn shunting_yard(prio: impl Fn(char) -> u8, tokens: impl Iterator<Item = T>) -> u64 {
    let mut op_stack = vec![];
    let mut val_stack = vec![];

    for t in tokens {
        match t {
            T::Num(n) => val_stack.push(n as u64),
            T::Op('(') => op_stack.push('('),
            T::Op(')') => {
                while let Some(op) = op_stack.pop() {
                    if op == '(' {
                        break;
                    } else {
                        eval_op(&mut val_stack, op);
                    }
                }
            }
            T::Op(op) => match op_stack.last() {
                Some('(') | None => op_stack.push(op),
                Some(&c) if prio(c) < prio(op) => op_stack.push(op),
                _ => {
                    eval_op(&mut val_stack, op_stack.pop().unwrap());
                    op_stack.push(op);
                }
            },
        }
    }
    op_stack
        .into_iter()
        .rev()
        .for_each(|op| eval_op(&mut val_stack, op));
    val_stack.pop().unwrap()
}

fn eval_op(stack: &mut Vec<u64>, t: char) {
    let (lhs, rhs) = (stack.pop().unwrap(), stack.pop().unwrap());
    stack.push(match t {
        '+' => lhs + rhs,
        '*' => lhs * rhs,
        _ => panic!(),
    })
}

fn tokenize(mut s: &str) -> impl Iterator<Item = T> + '_ {
    std::iter::from_fn(move || {
        while !s.is_empty() {
            let c = s.chars().next().unwrap();
            match c {
                ' ' => {
                    s = &s[1..];
                }
                '(' | ')' | '+' | '*' => {
                    s = &s[1..];
                    return Some(T::Op(c));
                }
                _ => {
                    let p = s.find(|c| c == ' ' || c == ')').unwrap_or_else(|| s.len());
                    let n = s[..p].parse().unwrap();
                    s = &s[p..];
                    return Some(T::Num(n));
                }
            }
        }
        None
    })
}
