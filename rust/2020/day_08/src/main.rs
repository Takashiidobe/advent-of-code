use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashSet, fs::read_to_string};

lazy_static! {
    static ref PARSE_RE: Regex = Regex::new(r"^(jmp|acc|nop) (.*)$").unwrap();
}

fn parse() -> String {
    read_to_string("../input.txt").unwrap()
}

fn run(instructions: Vec<(String, i32)>) -> Option<i32> {
    println!("in run function");
    let mut acc = 0;
    let mut line: i32 = 0;
    let mut set = HashSet::new();

    loop {
        if set.contains(&line) {
            break;
        }
        if line as usize == instructions.len() - 1 {
            return Some(acc);
        }
        set.insert(line);
        let instruction = &instructions[line as usize];

        match instruction.0.as_str() {
            "jmp" => line += instruction.1,
            "acc" => {
                acc += instruction.1;
                line += 1
            }
            "nop" => line += 1,
            _ => {}
        }
    }

    None
}

fn parse_instructions() -> Vec<(String, i32)> {
    let input = parse();
    let mut instructions = vec![];
    for line in input.lines() {
        let captures = PARSE_RE.captures(line).unwrap();
        let instruction = captures[1].to_string();
        let num = captures[2].parse::<i32>().unwrap();
        instructions.push((instruction, num));
    }
    instructions
}

fn part_one() -> i32 {
    let mut acc = 0;
    let mut line: i32 = 0;
    let mut set = HashSet::new();
    let instructions = parse_instructions();

    loop {
        if set.contains(&line) {
            break;
        }
        set.insert(line);
        match instructions[line as usize].0.as_str() {
            "acc" => {
                acc += instructions[line as usize].1;
                line += 1
            }
            "jmp" => line += instructions[line as usize].1,
            "nop" => line += 1,
            _ => {}
        }
    }

    acc
}

fn part_two() -> i32 {
    let instructions = parse_instructions();

    for (i, instruction) in instructions.iter().enumerate() {
        if instruction.0.as_str() == "jmp" {
            let mut copy = instructions.clone();
            copy[i].0 = "nop".to_string();
            let result = run(copy.clone());
            if result.is_some() {
                return result.unwrap();
            }
        } else if instruction.0.as_str() == "nop" {
            let mut copy = instructions.clone();
            copy[i].0 = "jmp".to_string();
            let result = run(copy.clone());
            if result.is_some() {
                return result.unwrap();
            }
        }
    }

    -1
}

fn main() {
    println!("part one is: {}", part_one());
    println!("part two is: {}", part_two());
}
