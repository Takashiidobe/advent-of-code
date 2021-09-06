use std::fs::read_to_string;

fn parse() -> String {
    read_to_string("../input.txt").unwrap()
}

fn calc(
    chars: Vec<char>,
    index: usize,
    lower: usize,
    upper: usize,
    lower_char: char,
    upper_char: char,
) -> usize {
    if index == chars.len() - 1 {
        if chars[chars.len() - 1] == lower_char {
            return lower;
        } else if chars[chars.len() - 1] == upper_char {
            return upper;
        }
    }

    let mid = (upper + lower) / 2;
    let c = chars[index];

    if c == lower_char {
        return calc(chars, index + 1, lower, mid, lower_char, upper_char);
    } else {
        return calc(chars, index + 1, mid + 1, upper, lower_char, upper_char);
    }
}

fn get_seat_ids() -> Vec<usize> {
    let input = parse();
    let mut seat_ids = vec![];

    for line in input.lines() {
        let mut rows = vec![];
        let mut cols = vec![];
        for (i, c) in line.chars().enumerate() {
            if i < 7 {
                rows.push(c);
            } else {
                cols.push(c);
            }
        }
        let row = calc(rows, 0, 0, 127, 'F', 'B');
        let col = calc(cols, 0, 0, 7, 'L', 'R');
        seat_ids.push(row * 8 + col);
    }
    seat_ids
}

fn part_one() -> usize {
    let seat_ids = get_seat_ids();
    *seat_ids.iter().max().unwrap()
}

fn part_two() -> usize {
    let mut seat_ids = get_seat_ids();
    seat_ids.sort();

    let mut i = 0;
    let mut j = 1;

    while j < seat_ids.len() {
        if seat_ids[j] - 2 == seat_ids[i] {
            return seat_ids[j] - 1;
        }
        i += 1;
        j += 1;
    }

    0
}

fn main() {
    println!("part one: {}", part_one());
    println!("part two: {}", part_two());
}
