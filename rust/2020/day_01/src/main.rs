use std::collections::HashSet;
use std::fs::read_to_string;

#[allow(dead_code)]
fn part_1() {
    let input = read_to_string("../input.txt").unwrap();
    let mut set = HashSet::new();
    for num in input.lines() {
        let num = num.parse::<i32>().unwrap();
        if set.contains(&num) {
            println!(
                "The numbers multiplied together are: {}",
                num * (2020 - num)
            );
        }
        set.insert(2020 - num);
    }
}

fn part_2() {
    let input = read_to_string("../input.txt").unwrap();

    let mut vec = vec![];
    for num in input.lines() {
        let num = num.parse::<i32>().unwrap();
        vec.push(num);
    }

    vec.sort();

    for i in 0..vec.len() {
        let mut seen = HashSet::new();
        for j in i + 1..vec.len() {
            let x = vec[i];
            let y = vec[j];
            let z = 2020 - x - y;

            if seen.contains(&z) {
                println!("Multiplied Total: {}", x * y * z);
                return;
            }

            seen.insert(y);
        }
    }
}

fn main() {
    part_2();
}
