use std::collections::HashMap;
use std::fs::read_to_string;

fn read_file(file_name: &str) -> String {
    read_to_string(file_name).unwrap()
}

fn main() {
    let input = read_file("../../input.txt");

    let mut two_count = 0;
    let mut three_count = 0;

    for line in input.lines() {
        let mut m = HashMap::new();
        let mut has_two = false;
        let mut has_three = false;
        for c in line.chars() {
            *m.entry(c).or_insert(0) += 1;
        }
        for (_, value) in &m {
            if *value == 2 && !has_two {
                two_count += 1;
                has_two = true;
            }
            if *value == 3 && !has_three {
                three_count += 1;
                has_three = true;
            }
        }
    }

    println!("{}", two_count * three_count); // 6448
}
