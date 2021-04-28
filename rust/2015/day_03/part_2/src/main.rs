use std::collections::HashMap;
use std::fs::read_to_string;

fn read_input(file_name: &str) -> String {
    read_to_string(file_name).unwrap()
}

fn presents_sent(input: String) -> i32 {
    let mut hm: HashMap<(i32, i32), i32> = HashMap::new();
    hm.insert((0, 0), 1);
    let mut coordinates = (0, 0);
    for c in input.chars() {
        match c {
            '^' => {
                coordinates.1 += 1;
                *hm.entry(coordinates).or_insert(1) += 1;
            }
            'v' => {
                coordinates.1 -= 1;
                *hm.entry(coordinates).or_insert(1) += 1;
            }
            '<' => {
                coordinates.0 -= 1;
                *hm.entry(coordinates).or_insert(1) += 1;
            }
            '>' => {
                coordinates.0 += 1;
                *hm.entry(coordinates).or_insert(1) += 1;
            }
            _ => {}
        }
    }
    let mut count = 0;
    for (_, value) in hm {
        if value >= 1 {
            count += 1;
        }
    }
    count
}

fn main() {
    let input = read_input("../input.txt");
    println!("{}", presents_sent(input));
}
