use std::fs::read_to_string;

fn read_input(file_name: &str) -> String {
    read_to_string(file_name).unwrap()
}

fn count_floors(input: String) -> i32 {
    let mut result = 0;
    for c in input.chars() {
        if c == '(' {
            result += 1;
        } else if c == ')' {
            result -= 1;
        }
    }
    result
}

fn main() {
    let input = read_input("../input.txt");
    let floor_num = count_floors(input);
    println!("{}", floor_num); // 138 is the correct answer.
}
