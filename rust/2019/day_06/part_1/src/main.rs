use std::collections::HashMap;
use std::fs::read_to_string;

fn read_input(file_name: &str) -> String {
    read_to_string(file_name).unwrap()
}

fn calculate_orbits(mut orbits: i32, key: &str, map: &HashMap<&str, Vec<&str>>) -> i32 {
    if map.contains_key(key) {
        for entry in map.get(key).unwrap() {
            println!("{}, {}", key, entry);
            orbits += 1;
            calculate_orbits(orbits, entry, map);
        }
    }
    orbits
}

fn main() {
    let mut hm: HashMap<&str, Vec<&str>> = HashMap::new();
    let input = read_input("../../input.txt");
    let mut tuples: Vec<(&str, &str)> = Vec::new();

    for line in input.lines() {
        let split_line: Vec<&str> = line.split(')').collect();
        tuples.push((split_line[0], split_line[1]));
    }

    for (key, value) in tuples {
        if !hm.contains_key(key) {
            hm.insert(key, vec![value]);
        } else {
            hm.get_mut(key).unwrap().push(value);
        }
    }

    let mut orbits = 0;

    for (key, _) in &hm {
        orbits += calculate_orbits(orbits, key, &hm);
    }

    println!("{}", orbits);
}
