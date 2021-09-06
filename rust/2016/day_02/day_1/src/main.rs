use std::collections::HashMap;
use std::fs::read_to_string;

fn read_input() -> String {
    read_to_string("../../input.txt").unwrap()
}

fn instruction_to_pos(c: char, mut pos: (i32, i32)) -> (i32, i32) {
    match c {
        'U' => match pos.0 {
            0 | 1 => pos.0 = 0,
            _ => pos.0 = 1,
        },
        'R' => match pos.1 {
            1 | 2 => pos.1 = 2,
            _ => pos.1 = 1,
        },
        'D' => match pos.0 {
            1 | 2 => pos.0 = 2,
            _ => pos.0 = 1,
        },
        'L' => match pos.1 {
            0 | 1 => pos.1 = 0,
            _ => pos.1 = 1,
        },
        _ => println!("What the"),
    }

    pos
}

fn pos_to_location(pos: (i32, i32)) -> i32 {
    let mut hm = HashMap::new();
    let mut count = 1;
    while count < 10 {
        for i in 0..3 {
            for j in 0..3 {
                hm.insert((i, j), count);
                count += 1;
            }
        }
    }

    *hm.get(&pos).unwrap()
}

fn main() {
    let input = read_input();

    let mut positions = Vec::new();

    for line in input.lines() {
        let mut final_pos: (i32, i32) = (1, 1);
        for c in line.chars() {
            final_pos = instruction_to_pos(c, final_pos);
        }
        positions.push(pos_to_location(final_pos));
    }
    println!("{:?}", positions); //47978
}
