use std::fs::read_to_string;

fn read_input() -> String {
    read_to_string("../input.txt").unwrap()
}

fn main() {
    let input = read_input();
    let mut count = 0;

    for line in input.lines() {
        for c in line.chars().into_iter() {
            if c.is_acii() {
                count += 1;
            }
        }
        count -= 2;
    }

    println!("{}", count);
}
