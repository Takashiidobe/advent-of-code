use std::collections::HashMap;

fn transpose(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let size = grid.len();
    let mut rotated = vec![vec![char::default(); size]; size];
    for row in 0..size {
        for col in 0..size {
            rotated[col][size - 1 - row] = grid[row][col];
        }
    }
    rotated
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn tilt_north(input: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut copy = input.to_vec();

    for i in 0..copy.len() {
        for j in 0..copy[0].len() {
            let curr = copy[i][j];
            if curr == 'O' {
                let mut y = i;
                while y > 0 && copy[y - 1][j] == '.' {
                    copy[y - 1][j] = 'O';
                    copy[y][j] = '.';
                    y -= 1;
                }
            }
        }
    }

    copy
}

fn count(input: &Vec<Vec<char>>) -> usize {
    let col_len = input[0].len();

    let mut total = 0;

    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if input[i][j] == 'O' {
                total += col_len - i;
            }
        }
    }
    total
}

fn part_1(input: &str) -> usize {
    let parsed = parse(input);
    count(&tilt_north(&parsed))
}

fn part_2(input: &str) -> usize {
    let mut parsed = parse(input);

    let mut hm = HashMap::new();
    let mut period = None;

    for i in 0..10_000_000_000u64 {
        for _ in 0..4 {
            parsed = tilt_north(&parsed);
            parsed = transpose(parsed);
        }
        if !hm.contains_key(&parsed) {
            hm.insert(parsed.clone(), i);
        } else {
            period = Some(i - hm.get(&parsed).unwrap());
        }

        if let Some(p) = period {
            if (1000000000 - 1) % p == i % p {
                break;
            }
        } else {
            period = None;
        }
    }
    count(&parsed)
}

fn main() {
    let input = include_str!("../input.txt");
    println!("part 1: {}", part_1(input));
    println!("part 2: {}", part_2(input));
}

#[test]
fn example_1() {
    let example = include_str!("../example.txt");
    assert_eq!(136, part_1(example));
}

#[test]
fn example_2() {
    let example = include_str!("../example.txt");
    assert_eq!(64, part_2(example));
}
