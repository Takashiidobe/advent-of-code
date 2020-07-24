use std::fs::read_to_string;

fn read_input(file_name: &str) -> Vec<String> {
    read_to_string(file_name)
        .unwrap()
        .split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}

fn main() {
    let input = read_input("input.txt");

    let mut grid: Vec<(i32, i32)> = Vec::new();

    for i in 0..1000 {
        for j in 0..1000 {
            grid.push((i, j));
        }
    }

    for line in input {
        if line.starts_with("turn on ") {
            println!("Turn on!");
        } else if line.starts_with("turn off ") {
            println!("Turn off!");
        } else if line.starts_with("toggle ") {
            println!("Toggle!");
        }
    }
}
