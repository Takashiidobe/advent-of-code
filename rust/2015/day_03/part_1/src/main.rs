use std::collections::HashMap;
use std::fs::read_to_string;

fn read_input(file_name: &str) -> String {
    read_to_string(file_name).unwrap()
}

fn presents_sent(input: String) -> i32 {
    let mut santas_hm: HashMap<(i32, i32), i32> = HashMap::new();
    santas_hm.insert((0, 0), 1);
    let mut santas_coordinates = (0, 0);

    let mut robo_hm: HashMap<(i32, i32), i32> = HashMap::new();
    robo_hm.insert((0, 0), 1);
    let mut robo_coordinates = (0, 0);

    for (i, c) in input.chars().enumerate() {
        match i % 2 {
            0 => match c {
                '^' => {
                    santas_coordinates.1 += 1;
                    *santas_hm.entry(santas_coordinates).or_insert(1) += 1;
                }
                'v' => {
                    santas_coordinates.1 -= 1;
                    *santas_hm.entry(santas_coordinates).or_insert(1) += 1;
                }
                '<' => {
                    santas_coordinates.0 -= 1;
                    *santas_hm.entry(santas_coordinates).or_insert(1) += 1;
                }
                '>' => {
                    santas_coordinates.0 += 1;
                    *santas_hm.entry(santas_coordinates).or_insert(1) += 1;
                }
                _ => {}
            },
            1 => match c {
                '^' => {
                    robo_coordinates.1 += 1;
                    *robo_hm.entry(robo_coordinates).or_insert(1) += 1;
                }
                'v' => {
                    robo_coordinates.1 -= 1;
                    *robo_hm.entry(robo_coordinates).or_insert(1) += 1;
                }
                '<' => {
                    robo_coordinates.0 -= 1;
                    *robo_hm.entry(robo_coordinates).or_insert(1) += 1;
                }
                '>' => {
                    robo_coordinates.0 += 1;
                    *robo_hm.entry(robo_coordinates).or_insert(1) += 1;
                }
                _ => {}
            },
            _ => {}
        }
    }
    let mut count = 0;
    for (_, value) in santas_hm {
        if value >= 1 {
            count += 1;
        }
    }

    for (_, value) in robo_hm {
        if value >= 1 {
            count += 1;
        }
    }
    count
}

fn main() {
    let input = read_input("../input.txt");
    println!("{}", presents_sent(input));
}
