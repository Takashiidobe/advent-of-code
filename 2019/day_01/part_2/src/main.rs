use std::fs::read_to_string;

fn read_input(file_name: &str) -> String {
    read_to_string(file_name).unwrap()
}

fn calculate_fuel(mass: i32, fuel: i32) -> i32 {
    if mass < 3 {
        return fuel;
    }
    let fuel_required = (mass / 3) - 2;
    calculate_fuel(fuel_required, fuel + fuel_required)
}

fn main() {
    let input = read_input("../../input.txt");

    let mut total_fuel = 0;
    for fuel in input.lines() {
        let fuel = fuel.parse::<i32>().unwrap();
        total_fuel += calculate_fuel(fuel, 0);
    }

    println!("{}", total_fuel); // 5027950
}
