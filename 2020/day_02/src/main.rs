use regex::Regex;
use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct Password {
    lower: usize,
    upper: usize,
    character: char,
    password: String,
}

impl Password {
    fn verify(&self) -> bool {
        let mut count = 0;
        for c in self.password.chars() {
            if c == self.character {
                count += 1;
            }
        }
        return count >= self.lower && count <= self.upper;
    }

    fn verify_two(&self) -> bool {
        let lower = self.password.chars().nth(self.lower - 1).unwrap() == self.character;
        let upper = self.password.chars().nth(self.upper - 1).unwrap() == self.character;
        return upper ^ lower;
    }
}

fn main() {
    part_1();
    part_2();
}

fn parse() -> Vec<Password> {
    let input = read_to_string("../input.txt").unwrap();
    let mut passwords: Vec<Password> = vec![];

    let re: Regex = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
    for line in input.lines() {
        for cap in re.captures_iter(line) {
            passwords.push(Password {
                lower: cap[1].parse::<usize>().unwrap(),
                upper: cap[2].parse::<usize>().unwrap(),
                character: cap[3].parse::<char>().unwrap(),
                password: cap[4].to_string(),
            });
        }
    }
    passwords
}

fn part_1() {
    let mut count = 0;
    let passwords = parse();
    for p in passwords {
        if p.part_1() {
            count += 1;
        }
    }
    println!("part 1's count is: {}", count);
}

fn part_2() {
    let mut count = 0;
    let passwords = parse();
    for p in passwords {
        if p.part_two() {
            count += 1;
        }
    }
    println!("part 2's count is: {}", count);
}
