use std::collections::HashMap;
use std::fs::read_to_string;

fn read_file(file_name: &str) -> String {
    read_to_string(file_name).unwrap()
}

fn main() {
    let input = read_file("../../input.txt");

    for line in input.lines() {
        let mut m = HashMap::new();
        for c in line.chars() {
            *m.entry(c).or_insert(0) += 1;
        }
        for c in line.chars() {}
    }
}
