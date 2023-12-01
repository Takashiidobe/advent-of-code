#[allow(unused)]
fn input() -> String {
    include_str!("../input.txt").to_string()
}

#[allow(unused)]
fn example() -> String {
    include_str!("../example.txt").to_string()
}

fn parse_input(input: String) -> String {
    input
}

fn part_1(input: &str) -> usize {
    use std::collections::HashMap;
    let mut i = 0;
    let mut j = 4;
    let mut curr_window: HashMap<char, i32> = HashMap::default();

    let str_len = input.len();
    let mut input = input.chars();

    for index in i..j {
        *curr_window.entry(input.nth(index).unwrap()).or_default() += 1;
    }

    while j < str_len {
        dbg!(&curr_window.len());
        dbg!(i, j);
        if curr_window[&input.nth(i).unwrap()] == 1 {
            curr_window.remove(&input.nth(i).unwrap());
        } else {
            *curr_window.entry(input.nth(i).unwrap()).or_default() -= 1;
        }
        *curr_window.entry(input.nth(j).unwrap()).or_default() += 1;
        dbg!(&curr_window);
        i += 1;
        j += 1;
        if curr_window.len() == 4 {
            return j;
        }
    }

    0
}

fn part_2(v: Vec<u32>) -> u32 {
    todo!()
}

fn main() {
    let example = parse_input(example());
    let input = parse_input(input());
    println!("part 1: {}", part_1(&example));
    // println!("part 2: {}", part_2(input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let answer = part_1(&parse_input(example()));

        assert_eq!(answer, 5);
    }

    #[test]
    fn test_2() {
        let answer = part_1("nppdvjthqldpwncqszvftbrmjlhg");

        assert_eq!(answer, 6);
    }
}
