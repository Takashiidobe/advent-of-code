use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|x| x.chars().map(|x| x.to_digit(10).unwrap() as i32).collect())
        .collect()
}

fn part_1(input: &str) -> Option<i32> {
    let parsed = parse(input);
    traverse(&parsed, 0, 3)
}

fn part_2(input: &str) -> Option<i32> {
    let parsed = parse(input);
    traverse(&parsed, 4, 10)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    fn to_tuple(self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}

fn traverse(input: &[Vec<i32>], min_streak: usize, max_streak: usize) -> Option<i32> {
    let directions = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];
    let row_len = input.len() as i32;
    let col_len = input[0].len() as i32;

    let mut q = BinaryHeap::new();
    let mut visited = HashSet::new();

    q.push(Reverse((input[0][1], 0, 1, Direction::Right, 1)));
    q.push(Reverse((input[1][0], 1, 0, Direction::Down, 1)));

    while let Some(val) = q.pop() {
        let (cost, y, x, prev, streak) = val.0;

        if visited.contains(&(y, x, prev, streak)) {
            continue;
        } else {
            visited.insert((y, x, prev, streak));
        }

        if y == row_len - 1 && x == col_len - 1 {
            return Some(cost);
        }

        for d in directions {
            if streak < min_streak && prev != d {
                continue;
            }
            if streak == max_streak && prev == d {
                continue;
            }
            if d == prev.opposite() {
                continue;
            }
            let (dy, dx) = d.to_tuple();
            let new_y = dy + y;
            let new_x = dx + x;
            if new_y < 0 || new_x < 0 || new_y >= row_len || new_x >= col_len {
                continue;
            }
            let new_cost = cost + input[new_y as usize][new_x as usize];
            let new_streak = if d == prev { streak + 1 } else { 1 };
            q.push(Reverse((new_cost, new_y, new_x, d, new_streak)));
        }
    }

    None
}

fn main() {
    let input = include_str!("../input.txt");
    println!(
        "part 1: {}, part 2: {}",
        part_1(input).unwrap(),
        part_2(input).unwrap()
    );
}

#[test]
fn example_1() {
    let example = include_str!("../example.txt");
    let traversed = part_1(example);
    assert_eq!(traversed, Some(102));
}

#[test]
fn example_2() {
    let example = include_str!("../example.txt");
    let traversed = part_2(example);
    assert_eq!(traversed, Some(94));
}
