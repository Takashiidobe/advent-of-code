use std::collections::VecDeque;

#[allow(unused)]
fn input() -> String {
    include_str!("../input.txt").to_string()
}

#[allow(unused)]
fn example() -> String {
    include_str!("../example.txt").to_string()
}

#[derive(Debug)]
enum Parsing {
    Crates,
    Moves,
}

fn parse_crates(line: String, crates: &mut VecDeque<String>) {
    let length = line.len();
    let mut start = 0;
    let mut end = 3;
    let mut curr_index = 0;
    while end <= length {
        let curr_crate = &line[start..end];
        if curr_crate.starts_with('[') && curr_crate.ends_with(']') {
            let curr_char = curr_crate.chars().nth(1).unwrap().to_string();
            if let Some(s) = crates.get_mut(curr_index) {
                s.push_str(&curr_char);
                crates[curr_index] = s.to_string();
            } else {
                crates.push_back(curr_char);
            }
        }

        start += 4;
        end += 4;
        curr_index += 1;
    }
    dbg!(crates);
}

fn parse_input(input: String) -> Vec<u32> {
    let mut step = Parsing::Crates;
    // first, parse into lines
    // if you see two lines, thats it for the crates
    for line in input.lines() {
        if line.is_empty() {
            step = Parsing::Moves;
        }
        match step {
            Parsing::Crates => parse_crates(line.to_string(), &mut VecDeque::from(vec![])),
            Parsing::Moves => todo!(),
        }
    }

    todo!()
}

fn part_1(v: Vec<u32>) -> u32 {
    todo!()
}

fn part_2(v: Vec<u32>) -> u32 {
    todo!()
}

fn main() {
    let example = parse_input(example());
    let input = parse_input(input());
    println!("part 1: {}", part_1(input.clone()));
    println!("part 2: {}", part_2(input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let answer = part_1(parse_input(example()));

        assert_eq!(answer, 0);
    }

    #[test]
    fn test_2() {
        let answer = part_2(parse_input(example()));

        assert_eq!(answer, 0);
    }
}
