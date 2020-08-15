use std::fs::read_to_string;

fn read_input(file_name: &str) -> String {
    read_to_string(file_name).unwrap()
}

fn first_basement_position(input: String) -> usize {
    let mut level = 0;
    for (i, c) in input.chars().enumerate() {
        if level == -1 {
            return i;
        }
        match c {
            '(' => level += 1,
            ')' => level -= 1,
            _ => {}
        }
    }
    1
}

fn main() {
    let input = read_input("../input.txt");
    let floor = first_basement_position(input);

    println!("{}", floor); // 1771, correct answer.
}
