use std::fs::read_to_string;

fn parse() -> String {
    read_to_string("../input.txt").unwrap()
}
fn tree_count(x: usize, y: usize) -> usize {
    let input = parse();
    let mut trees = 0;
    for (i, line) in input.lines().skip(y).step_by(y).enumerate() {
        let pos = ((i + 1) * x) % line.len();
        let ch = line.chars().nth(pos).unwrap();
        if ch == '#' {
            trees += 1;
        }
    }
    trees
}

fn part_one() {
    let three_one = tree_count(3, 1);
    println!("part one: {}", three_one);
}

fn part_two() {
    let one_one = tree_count(1, 1);
    let five_one = tree_count(5, 1);
    let three_one = tree_count(3, 1);
    let seven_one = tree_count(7, 1);
    let one_two = tree_count(1, 2);
    let part_two = vec![one_one, three_one, five_one, seven_one, one_two];
    let total = part_two.iter().fold(1, |acc, x| acc * x);

    println!("part two: {}", total);
}

fn main() {
    part_one(); // 284 trees
    part_two(); // 3510149120 trees
}
