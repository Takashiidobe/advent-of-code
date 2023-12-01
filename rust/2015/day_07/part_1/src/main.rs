use std::collections::BTreeMap;

#[derive(Debug)]
enum BinOp {
    LShift,
    RShift,
    And,
    Or,
}

#[derive(Debug)]
enum UnaryOp {
    Not,
}

fn parse_num(s: &str) -> i32 {
    let mut res = 0;
    for c in s.bytes() {
        res = res * 10 + (c - b'0') as i32;
    }
    res
}

fn parse_bin_op(s: &str) -> BinOp {
    match s {
        "AND" => BinOp::And,
        "OR" => BinOp::Or,
        "LSHIFT" => BinOp::LShift,
        "RSHIFT" => BinOp::RShift,
        _ => unreachable!(),
    }
}
fn parse_unary_op(s: &str) -> UnaryOp {
    match s {
        "NOT" => UnaryOp::Not,
        _ => unreachable!(),
    }
}

fn parse_var(s: &str) -> &str {
    s
}

fn is_num(s: &str) -> bool {
    s.bytes().map(|x| x.is_ascii_digit()).all(|x| x)
}

use BinOp::*;
use UnaryOp::*;

fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|x| {
            let v: Vec<_> = x.split(" -> ").collect();
            let left: Vec<_> = v[0].split(' ').collect();
            let right = vec![v[1]];
            (left, right)
        })
        .collect();

    let mut hashmap = BTreeMap::new();

    for line in input {
        let left = line.0.clone();
        let target = line.1[0];
        let length = left.len();
        if length == 1 {
            let res = parse_num(left[0]);
            hashmap.insert(target, res);
        } else if length == 2 {
            let unary_op = parse_unary_op(left[0]);
            let var = parse_var(left[1]);
            let value = hashmap.get(var).unwrap();
            let res = match unary_op {
                Not => !value,
            };
            dbg!(var, value, unary_op, res, target);
            hashmap.insert(target, res);
        } else if length == 3 {
            let left_var = parse_var(left[0]);
            let bin_op = parse_bin_op(left[1]);
            let left_side = hashmap.get(left_var).unwrap();
            if is_num(left[2]) {
                let right = parse_num(left[2]);
                let res = match bin_op {
                    LShift => left_side << right,
                    RShift => left_side >> right,
                    And => left_side & right,
                    Or => left_side | right,
                };
                hashmap.insert(target, res);
            } else {
                let right = parse_var(left[2]);
                let right = hashmap.get(right).unwrap();
                let res = match bin_op {
                    LShift => left_side << right,
                    RShift => left_side >> right,
                    And => left_side & right,
                    Or => left_side | right,
                };
                hashmap.insert(target, res);
            }
        } else {
            unreachable!()
        }
    }

    dbg!(hashmap);
}
