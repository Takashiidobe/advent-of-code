use std::collections::BTreeSet;

fn parse(input: &str) -> Vec<Vec<char>> {
    let mut res = vec![];
    for line in input.lines() {
        if line.chars().all(|c| c == '.') {
            res.push(line.chars().collect());
        }
        // the normal case, just push the vec
        res.push(line.chars().collect());
    }
    let transposed = transpose(res.clone());

    let mut copy = vec![];

    for v in &transposed {
        if v.iter().all(|c| *c == '.') {
            copy.push(v.clone());
        }
        copy.push(v.clone());
    }

    transpose(copy)
}

fn transpose<T>(original: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut transposed = (0..original[0].len()).map(|_| vec![]).collect::<Vec<_>>();

    for original_row in original {
        for (item, transposed_row) in original_row.into_iter().zip(&mut transposed) {
            transposed_row.push(item);
        }
    }

    transposed
}

fn calc_distance(left: (usize, usize), right: (usize, usize)) -> usize {
    let (left_x, left_y) = left;
    let (right_x, right_y) = right;

    let min_x = left_x.min(right_x);
    let max_x = left_x.max(right_x);

    let min_y = left_y.min(right_y);
    let max_y = left_y.max(right_y);

    max_x - min_x + max_y - min_y
}

fn part_1(input: &str) -> usize {
    let parsed_input = parse(input);
    let mut steps = 0;

    let mut galaxies = vec![];

    for (i, row) in parsed_input.into_iter().enumerate() {
        for (j, c) in row.into_iter().enumerate() {
            if c == '#' {
                galaxies.push((i, j));
            }
        }
    }

    for (i, left) in galaxies.iter().enumerate() {
        for right in &galaxies[i + 1..] {
            steps += calc_distance(*left, *right);
        }
    }

    steps
}

fn parse_part_2(input: &str) -> (Vec<Vec<char>>, BTreeSet<usize>, BTreeSet<usize>) {
    let mut res = vec![];
    let mut empty_rows = BTreeSet::new();
    let mut empty_cols = BTreeSet::new();
    for (i, line) in input.lines().enumerate() {
        if line.chars().all(|c| c == '.') {
            empty_rows.insert(i);
        }
        res.push(line.chars().collect());
    }
    let transposed = transpose(res.clone());

    let mut copy = vec![];

    for (i, v) in transposed.iter().enumerate() {
        if v.iter().all(|c| *c == '.') {
            empty_cols.insert(i);
        }
        copy.push(v.clone());
    }

    (transpose(copy), empty_rows, empty_cols)
}

fn calc_distance_2(
    left: (usize, usize),
    right: (usize, usize),
    rows: &BTreeSet<usize>,
    cols: &BTreeSet<usize>,
    distance_to_expand: usize,
) -> usize {
    let distance_to_expand = distance_to_expand - 1;
    let (left_x, left_y) = left;
    let (right_x, right_y) = right;

    let min_x = left_x.min(right_x);
    let max_x = left_x.max(right_x);

    let min_y = left_y.min(right_y);
    let max_y = left_y.max(right_y);

    let row_len = rows.range(min_x..max_x).count();
    let col_len = cols.range(min_y..max_y).count();

    max_y + max_x - min_y - min_x + ((col_len + row_len) * distance_to_expand)
}

fn part_2(input: &str, distance_to_expand: usize) -> usize {
    let (parsed_input, rows, cols) = parse_part_2(input);
    let mut steps = 0;

    let mut galaxies = vec![];

    for (i, row) in parsed_input.into_iter().enumerate() {
        for (j, c) in row.into_iter().enumerate() {
            if c == '#' {
                galaxies.push((i, j));
            }
        }
    }

    for (i, left) in galaxies.iter().enumerate() {
        for right in &galaxies[i + 1..] {
            steps += calc_distance_2(*left, *right, &rows, &cols, distance_to_expand);
        }
    }

    steps
}

fn main() {
    let input = include_str!("../input.txt");
    let part_1_res = part_1(input);
    let part_2_res = part_2(input, 1_000_000);
    println!("part 1: {}, part 2: {}", part_1_res, part_2_res);
}

#[test]
fn example_1() {
    let example = include_str!("../example.txt");
    let result = part_1(example);
    assert_eq!(result, 374);
}

#[test]
fn example_2() {
    let example = include_str!("../example.txt");
    let result = part_2(example, 100);
    assert_eq!(result, 8410);
}
