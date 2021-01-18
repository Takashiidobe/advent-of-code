use lazy_static::lazy_static;
use regex::Regex;
use std::{
    cmp::max,
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

lazy_static! {
    static ref PARSE_RE: Regex = Regex::new(r"^([a-z]+) \(([0-9]+)\)(.*)$").unwrap();
}

#[derive(Debug, Clone)]
struct Tower {
    name: String,
    weight: u32,
    children: Vec<String>,
    towers: Vec<Tower>,
}

impl Tower {
    fn new(name: String, weight: u32, children: Vec<String>) -> Tower {
        Tower {
            name,
            weight,
            children,
            towers: vec![],
        }
    }
}

fn parse() -> Vec<Tower> {
    let input = read_to_string("../input.txt").unwrap();
    let mut towers = vec![];
    for line in input.lines() {
        let captures = PARSE_RE.captures(line).unwrap();
        let name = captures[1].to_string();
        let weight = captures[2].parse::<u32>().unwrap();
        if captures[3].len() < 1 {
            towers.push(Tower::new(name, weight, vec![]));
        } else {
            let comma_separated = captures[3].to_string();
            let trimmed = comma_separated.trim_start_matches(" -> ");
            let parsed_towers: Vec<String> =
                trimmed.clone().split(", ").map(|c| c.to_string()).collect();

            towers.push(Tower::new(name, weight, parsed_towers));
        }
    }
    towers
}
fn part_one() -> HashSet<String> {
    let towers = parse();

    let mut parents: HashSet<String> = HashSet::new();
    let mut children: HashSet<String> = HashSet::new();

    for t in towers {
        parents.insert(t.name);
        for child in t.children {
            children.insert(child);
        }
    }
    for child in children {
        if parents.contains(&child) {
            parents.remove(&child);
        }
    }
    parents
}

fn part_two() -> u32 {
    let towers = parse();
    let mut m: HashMap<String, Tower> = HashMap::new();

    for t in towers.clone() {
        m.insert(t.name.clone(), t);
    }

    for mut t in m.values() {}

    println!("{:#?}", m);

    0
}

fn main() {
    //    println!("part one is: {:#?}", part_one());
    part_two();
}
