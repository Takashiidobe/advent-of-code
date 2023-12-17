use linked_hash_map::LinkedHashMap;

fn parse(input: &str) -> Vec<&str> {
    input.split(',').collect()
}

fn hash_one(bytes: &[u8]) -> usize {
    let mut hash_val = 0;
    for b in bytes {
        hash_val += *b as usize;
        hash_val *= 17;
        hash_val %= 256;
    }
    hash_val
}

fn part_1(input: &str) -> usize {
    let parsed = parse(input);
    let mut total = 0;
    for s in parsed {
        let bytes: Vec<_> = s.bytes().collect();
        total += hash_one(&bytes);
    }
    total
}

fn part_2(input: &str) -> usize {
    let parsed = parse(input);
    let mut boxes = vec![LinkedHashMap::<&str, usize>::new(); 256];

    for val in parsed {
        let split: Vec<_> = val.split(|s| s == '-' || s == '=').collect();
        let tag_hash = hash_one(split[0].as_bytes());
        if val.contains('=') {
            *boxes[tag_hash].entry(split[0]).or_default() = split[1].parse().unwrap();
        }
        if val.contains('-') {
            boxes[tag_hash].remove(split[0]);
        }
    }
    let mut total = 0;

    for (box_index, m) in boxes.into_iter().enumerate() {
        for (map_index, v) in m.values().enumerate() {
            total += (box_index + 1) * (map_index + 1) * v;
        }
    }

    total
}

// add to the data structure, a vector of boxes from 0 -> 255
fn main() {
    let input = include_str!("../input.txt").trim();
    println!("part 1: {}, part 2: {}", part_1(input), part_2(input));
}

#[test]
fn example_1() {
    let example = include_str!("../example.txt").trim();
    assert_eq!(1320, part_1(example));
}

#[test]
fn example_2() {
    let example = include_str!("../example.txt").trim();
    assert_eq!(145, part_2(example));
}
