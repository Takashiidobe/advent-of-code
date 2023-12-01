#[allow(unused)]
fn input() -> String {
    include_str!("../input.txt").to_string()
}

#[allow(unused)]
fn example() -> String {
    include_str!("../example.txt").to_string()
}

fn parse_input(input: String) -> Vec<u32> {
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
