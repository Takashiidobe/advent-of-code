use std::{
    collections::{HashSet, VecDeque},
    mem,
};

use itertools::Itertools;

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|c| c.chars().collect()).collect()
}

fn bfs(input: &[Vec<char>], steps_taken: usize) -> usize {
    let row_len = input.len() as i32;
    let col_len = input[0].len() as i32;
    let mut start_y = 0;
    let mut start_x = 0;
    'exit: for y in 0..row_len {
        for x in 0..col_len {
            if input[y as usize][x as usize] == 'S' {
                start_y = y;
                start_x = x;
                break 'exit;
            }
        }
    }

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut q = VecDeque::new();
    let mut visited = HashSet::new();

    let mut valid_squares = 0;

    q.push_back((start_y, start_x, 0));

    while let Some((y, x, steps)) = q.pop_front() {
        if y < 0 || x < 0 || y >= row_len || x >= col_len {
            continue;
        }
        if visited.contains(&(y, x)) {
            continue;
        }
        if steps > steps_taken {
            continue;
        }
        if input[y as usize][x as usize] == '#' {
            continue;
        }
        if steps % 2 == 0 {
            valid_squares += 1;
            visited.insert((y, x));
        }
        for (dy, dx) in directions {
            q.push_back((dy + y, dx + x, steps + 1));
        }
    }

    valid_squares
}

fn part_1(input: &str, steps: usize) -> usize {
    bfs(&parse(input), steps)
}

fn part_2(input: &str) -> usize {
    let [mut v, mut v2] = [
        HashSet::<[isize; 2]>::from_iter([[131 / 2; 2]]),
        HashSet::<[isize; 2]>::default(),
    ];
    let (_, a, b, c) = (1..)
        .filter_map(|step| {
            v.drain().for_each(|[x, y]| {
                [[0, 1], [0, -1], [-1, 0], [1, 0]]
                    .iter()
                    .for_each(|[dx, dy]| {
                        let [nx, ny] = [x + dx, y + dy];
                        let i = (ny.rem_euclid(131) * (131 + 1) + nx.rem_euclid(131)) as usize;
                        let _ = (input.as_bytes()[i] != b'#').then(|| v2.insert([nx, ny]));
                    });
            });

            mem::swap(&mut v, &mut v2);

            (step == 64 || step % 131 == 65).then_some(v.len())
        })
        .next_tuple()
        .unwrap();

    let n = 202300;
    a + (b - a) * n + (a + c - 2 * b) * (n * (n - 1) / 2)
}

fn main() {
    let input = include_str!("../input.txt");
    println!("part 1: {}, part 2: {}", part_1(input, 64), part_2(input));
}

#[test]
fn example_1() {
    let example = include_str!("../example.txt");
    assert_eq!(part_1(example, 6), 16);
}
