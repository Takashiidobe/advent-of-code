use std::collections::{HashMap, VecDeque};

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

fn bfs(input: &[Vec<char>], starting_square: ((i32, i32), Direction)) -> usize {
    let mut q = VecDeque::new();
    let mut visited: HashMap<(i32, i32, Direction), usize> = HashMap::new();
    let row_len = input.len();
    let col_len = input[0].len();
    q.push_back(starting_square);
    let mut copy = input.to_vec();

    while let Some(((y, x), d)) = q.pop_front() {
        if x < 0 || y < 0 || y as usize >= row_len || x as usize >= col_len {
            continue;
        }

        if visited.get(&(y, x, d)).is_some_and(|v| v > &1) {
            continue;
        }

        *visited.entry((y, x, d)).or_default() += 1;
        copy[y as usize][x as usize] = '#';

        let c = input[y as usize][x as usize];
        match (c, d) {
            ('.', d) => match d {
                Direction::Right => q.push_back(((y, x + 1), d)),
                Direction::Left => q.push_back(((y, x - 1), d)),
                Direction::Up => q.push_back(((y - 1, x), d)),
                Direction::Down => q.push_back(((y + 1, x), d)),
            },
            ('/', d) => match d {
                Direction::Right => q.push_back(((y - 1, x), Direction::Up)),
                Direction::Left => q.push_back(((y + 1, x), Direction::Down)),
                Direction::Up => q.push_back(((y, x + 1), Direction::Right)),
                Direction::Down => q.push_back(((y, x - 1), Direction::Left)),
            },
            ('\\', d) => match d {
                Direction::Right => q.push_back(((y + 1, x), Direction::Down)),
                Direction::Left => q.push_back(((y - 1, x), Direction::Up)),
                Direction::Up => q.push_back(((y, x - 1), Direction::Left)),
                Direction::Down => q.push_back(((y, x + 1), Direction::Right)),
            },
            ('|', d) => match d {
                Direction::Right | Direction::Left => {
                    q.push_back(((y + 1, x), Direction::Down));
                    q.push_back(((y - 1, x), Direction::Up));
                }
                Direction::Up => q.push_back(((y - 1, x), d)),
                Direction::Down => q.push_back(((y + 1, x), d)),
            },
            ('-', d) => match d {
                Direction::Right => q.push_back(((y, x + 1), d)),
                Direction::Left => q.push_back(((y, x - 1), d)),
                Direction::Up | Direction::Down => {
                    q.push_back(((y, x - 1), Direction::Left));
                    q.push_back(((y, x + 1), Direction::Right));
                }
            },
            _ => unreachable!(),
        }
    }

    copy.iter()
        .map(|x| x.iter().filter(|x| **x == '#').count())
        .sum()
}

fn part_1(input: &str) -> usize {
    let input = parse(input);
    bfs(&input, ((0, 0), Direction::Right))
}

fn part_2(input: &str) -> usize {
    let input = parse(input);
    let mut max_so_far = 0;

    for i in 0..input.len() {
        max_so_far = max_so_far.max(bfs(&input, ((0, i as i32), Direction::Down)));
        max_so_far = max_so_far.max(bfs(
            &input,
            (((input.len() - 1) as i32, i as i32), Direction::Up),
        ));
    }

    for i in 0..input[0].len() {
        max_so_far = max_so_far.max(bfs(&input, ((i as i32, 0), Direction::Right)));
        max_so_far = max_so_far.max(bfs(
            &input,
            (((input[0].len() - 1) as i32, i as i32), Direction::Left),
        ));
    }

    max_so_far
}

fn main() {
    let input = include_str!("../input.txt");
    println!("part 1: {}, part 2: {}", part_1(input), part_2(input));
}

#[test]
fn example_1() {
    let input = include_str!("../example.txt");
    assert_eq!(46, part_1(input));
}

#[test]
fn example_2() {
    let input = include_str!("../example.txt");
    assert_eq!(51, part_2(input));
}
