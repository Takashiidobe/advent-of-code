use std::fs::read_to_string;

fn read_input(file_name: &str) -> String {
    read_to_string(file_name).unwrap()
}

fn input_to_feet(input: String) -> usize {
    let mut feet = 0;
    for i in input.lines() {
        let split_i = i.split("x").collect::<Vec<_>>();

        let l = split_i[0].parse::<usize>().unwrap();
        let w = split_i[1].parse::<usize>().unwrap();
        let h = split_i[2].parse::<usize>().unwrap();

        let mut sides = vec![l, w, h];
        sides.sort();
        let smallest_side = sides[0];
        let middle_side = sides[1];
        let largest_side = sides[2];

        let volume = l * w * h;
        let ribbon = (2 * smallest_side) + (2 * middle_side);

        feet += volume;
        feet += ribbon;
    }
    feet
}

fn main() {
    let input = read_input("../input.txt");
    let feet = input_to_feet(input);

    println!("{}", feet); // 1588178 feet
}
