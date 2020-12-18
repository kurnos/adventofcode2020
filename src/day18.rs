use crate::infra::Problem;

use itertools::Itertools;
use std::collections::VecDeque;

pub struct Day18;

impl Problem<String, String, u128, u128> for Day18 {
    fn day() -> u8 {
        18
    }

    fn first(contents: String) -> u128 {
        let data = contents.lines().map(parse).collect_vec();

        let mut sum = 0;

        for mut expr in data {
            sum += eval(&mut expr);
        }
        sum
    }

    fn second(contents: String) -> u128 {
        let data = contents.lines().map(parse).collect_vec();

        let mut sum = 0;

        for expr in data {
            sum += eval(&mut reshape(expr));
        }
        sum
    }
}

fn reshape_val(e: E) -> E {
    match e {
        e @ E::Num(_) | e @ E::Reshaped(_) => e,
        E::Paren(x) => E::Reshaped(reshape(x)),
        _ => panic!(),
    }
}

fn reshape(mut expr: VecDeque<E>) -> VecDeque<E> {
    let mut res = VecDeque::new();
    while expr.len() != 1 {
        let lhs = reshape_val(expr.pop_front().unwrap());
        let op = expr.pop_front().unwrap();
        match op {
            E::Plus => {
                let rhs = reshape_val(expr.pop_front().unwrap());
                let mut e = VecDeque::new();
                e.push_front(lhs);
                e.push_front(op);
                e.push_front(rhs);
                expr.push_front(E::Reshaped(e))
            }
            E::Mul => {
                res.push_back(lhs);
                res.push_back(op);
            }
            _ => panic!(),
        }
    }
    res.push_back(reshape_val(expr.pop_front().unwrap()));
    res
}

fn get_val(e: E) -> u128 {
    match e {
        E::Num(n) => n,
        E::Reshaped(mut inner) | E::Paren(mut inner) => {
            let res = eval(&mut inner);
            assert!(inner.is_empty());
            res
        }
        _ => panic!(),
    }
}

fn eval(queue: &mut VecDeque<E>) -> u128 {
    while queue.len() > 1 {
        let lhs = get_val(queue.pop_front().unwrap());
        let op = queue.pop_front().unwrap();
        let rhs = get_val(queue.pop_front().unwrap());
        match op {
            E::Plus => queue.push_front(E::Num(lhs + rhs)),
            E::Mul => queue.push_front(E::Num(lhs * rhs)),
            _ => panic!(),
        }
    }
    get_val(queue.pop_front().unwrap())
}

#[derive(Debug, PartialEq)]
enum E {
    Num(u128),
    Plus,
    Mul,
    Paren(VecDeque<E>),
    Reshaped(VecDeque<E>),
}

fn parse(line: &str) -> VecDeque<E> {
    let mut stack = VecDeque::new();
    let mut current = VecDeque::new();

    let mut s = line;
    while !s.is_empty() {
        match s.chars().next().unwrap() {
            '(' => {
                stack.push_back(current);
                current = VecDeque::new();
                s = &s[1..];
            }
            ')' => {
                let completed = current;
                current = stack.pop_back().unwrap();
                current.push_back(E::Paren(completed));
                s = &s[1..];
            }
            ' ' => {
                s = &s[1..];
            }
            '+' => {
                current.push_back(E::Plus);
                s = &s[1..];
            }
            '*' => {
                current.push_back(E::Mul);
                s = &s[1..];
            }
            _ => {
                let p = s.find(|c| c == ' ' || c == ')').unwrap_or_else(|| s.len());
                current.push_back(E::Num(s[..p].parse().unwrap()));
                s = &s[p..];
            }
        }
    }
    assert!(stack.is_empty());
    current
}
