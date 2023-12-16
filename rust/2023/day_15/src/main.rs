fn parse(input: &str) -> Vec<&str> {
    input.split(',').collect()
}

fn hash_one(bytes: &[usize]) -> usize {
    let mut hash_val = 0;
    for b in bytes {
        hash_val += b;
        hash_val *= 17;
        hash_val %= 256;
    }
    hash_val
}

fn part_1(input: &str) -> usize {
    let parsed = parse(input);
    let mut total = 0;
    for s in parsed {
        let bytes: Vec<usize> = s.bytes().map(|b| b as usize).collect();
        total += hash_one(&bytes);
    }
    total
}

// add to the data structure, a vector of boxes from 0 -> 255

fn main() {
    let example = include_str!("../example.txt").trim();
    let input = include_str!("../input.txt").trim();
    println!("{}", part_1(input));
}

#[test]
fn example_1() {
    let example = include_str!("../example.txt").trim();
    assert_eq!(1320, part_1(example));
}
