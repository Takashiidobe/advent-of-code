use std::fs::read_to_string;

fn read_input(file_name: &str) -> String {
    read_to_string(file_name).unwrap()
}

fn fuel_for_mass(mass: i32) -> i32 {
    (mass / 3) - 2
}

fn main() {
    let input = read_input("../../input.txt");
    let mut fuel = 0;
    for line in input.lines() {
        fuel += fuel_for_mass(line.parse::<i32>().unwrap());
    }
    println!("{}", fuel); // 3353880 as expected
}
