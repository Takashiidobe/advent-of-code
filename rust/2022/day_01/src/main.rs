fn input() -> String {
    include_str!("../input.txt").to_string()
}

fn parse_input(input: String) -> Vec<u32> {
    let mut res = vec![];
    let mut curr: Vec<u32> = vec![];
    for line in input.lines() {
        if line.is_empty() {
            res.push(curr.into_iter().sum());
            curr = vec![];
        } else {
            curr.push(line.trim().parse().unwrap());
        }
    }

    if !curr.is_empty() {
        res.push(curr.into_iter().sum());
    }

    res
}

fn part_1(v: Vec<u32>) -> u32 {
    v.into_iter().max().unwrap()
}

fn part_2(v: Vec<u32>) -> u32 {
    let mut v = v;
    v.sort();
    v[v.len() - 3..].iter().sum()
}

fn main() {
    let inp = parse_input(input());
    println!("part 1: {}", part_1(inp.clone()));
    println!("part 2: {}", part_2(inp));
}

#[cfg(test)]
mod test {
    use super::*;

    fn example() -> String {
        include_str!("../example.txt").to_string()
    }

    #[test]
    fn test_1() {
        let answer = part_1(parse_input(example()));

        assert_eq!(answer, 24000);
    }

    #[test]
    fn test_2() {
        let answer = part_2(parse_input(example()));

        assert_eq!(answer, 45000);
    }
}
