use std::collections::HashSet;

fn input() -> String {
    include_str!("../input.txt").to_string()
}

fn example() -> String {
    include_str!("../example.txt").to_string()
}

fn parse_input(input: String) -> Vec<(HashSet<char>, HashSet<char>)> {
    let mut res = vec![];
    for line in input.lines() {
        let mid = line.len() / 2;
        let (first, second) = line.split_at(mid);
        let first_chars = HashSet::from_iter(first.chars());
        let second_chars = HashSet::from_iter(second.chars());
        res.push((first_chars, second_chars));
    }
    res
}

fn parse_input_pt_2(input: String) -> Vec<HashSet<char>> {
    let mut res = vec![];
    for line in input.lines().collect::<Vec<_>>().chunks(3) {
        match line {
            [first, second, third] => {
                let first: HashSet<_> = HashSet::from_iter(first.chars());
                let second: HashSet<_> = HashSet::from_iter(second.chars());
                let third: HashSet<_> = HashSet::from_iter(third.chars());
                let mut sets = [first, second, third];
                let (intersection, others) = sets.split_at_mut(1);
                let intersection = &mut intersection[0];
                for other in others {
                    intersection.retain(|e| other.contains(e));
                }

                res.push(intersection.to_owned());
            }
            _ => unreachable!(),
        }
    }
    res
}

fn part_1(v: Vec<(HashSet<char>, HashSet<char>)>) -> u32 {
    let items: Vec<_> = v
        .into_iter()
        .map(|(a, b)| a.intersection(&b).collect::<Vec<_>>()[0].to_owned())
        .collect();
    score(items)
}

fn score(items: Vec<char>) -> u32 {
    let mut total: u32 = 0;
    for item in items {
        let item = item as u8;
        match item {
            b'a'..=b'z' => total += (item - b'a' + 1) as u32,
            b'A'..=b'Z' => total += (item - b'A' + 27) as u32,
            _ => {}
        }
    }
    total
}

fn part_2(items: Vec<HashSet<char>>) -> u32 {
    // for each set, we want to get the only item it has
    let items: Vec<_> = items
        .into_iter()
        .map(|item| {
            item.into_iter()
                .collect::<Vec<char>>()
                .first()
                .unwrap()
                .to_owned()
        })
        .collect();
    score(items)
}

fn main() {
    let inp = parse_input(input());
    let inp_pt2 = parse_input_pt_2(input());
    println!("part 1: {}", part_1(inp));
    println!("part 2: {}", part_2(inp_pt2));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let answer = part_1(parse_input(example()));

        assert_eq!(answer, 157);
    }

    #[test]
    fn test_2() {
        let answer = part_2(parse_input_pt_2(example()));

        assert_eq!(answer, 70);
    }
}
