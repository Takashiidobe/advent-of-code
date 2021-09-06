use std::fs::read_to_string;

fn read_input(file_name: &str) -> String {
    read_to_string(file_name).unwrap()
}

fn is_triangle(a: i32, b: i32, c: i32) -> bool {
    if a + b > c && b + c > a && c + a > b {
        true
    } else {
        false
    }
}

fn main() {
    let input = read_input("../../input.txt");

    let mut v = Vec::new();

    for line in input.lines() {
        let line = line.split(",").collect::<Vec<&str>>();
        let numbers: Vec<i32> = line.into_iter().map(|s| s.parse().unwrap()).collect();

        v.push((numbers[0], numbers[1], numbers[2]));
    }

    let mut counter = 0;

    let mut i = 0;

    while i < v.len() - 2 {
        if is_triangle(v[i].0, v[i + 1].0, v[i + 2].0) {
            counter += 1;
        }
        if is_triangle(v[i].1, v[i + 1].1, v[i + 2].1) {
            counter += 1;
        }
        if is_triangle(v[i].2, v[i + 1].2, v[i + 2].2) {
            counter += 1;
        }
        i += 3;
    }

    println!("{}", counter);
}
