use std::collections::HashSet;

fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect()
}

fn extrapolate_histories(input: Vec<Vec<i64>>) -> i64 {
    let mut total = 0;
    for histories in input {
        let mut repeated_histories = iterate_repeatedly(&histories);
        for i in 0..repeated_histories.len() {
            let index = repeated_histories.len() - i - 1;
            if index == repeated_histories.len() - 1 {
                let val_to_push = repeated_histories[index].last().cloned();
                if let Some(val) = val_to_push {
                    repeated_histories[index].push(val);
                }
            } else {
                let prev_index = repeated_histories.len() - i;
                let val_to_push = repeated_histories[index].last().cloned();
                let prev_val_to_push = repeated_histories[prev_index].last().cloned();
                match (val_to_push, prev_val_to_push) {
                    (Some(val), Some(prev_val)) => {
                        repeated_histories[index].push(val + prev_val);
                    }
                    _ => unreachable!(),
                }
            }
        }
        match (repeated_histories[0].last(), histories.last()) {
            (Some(left), Some(right)) => total += left + right,
            _ => unreachable!(),
        }
    }
    total
}

fn iterate_repeatedly(histories: &[i64]) -> Vec<Vec<i64>> {
    let mut iterations: Vec<Vec<i64>> = vec![];
    iterations.push(iterate(histories));
    while let Some(v) = iterations.last() {
        let set: HashSet<&i64> = HashSet::from_iter(v.iter());
        if set.len() == 1 {
            break;
        } else {
            iterations.push(iterate(v));
        }
    }
    iterations
}

fn iterate(history: &[i64]) -> Vec<i64> {
    let mut changes = vec![];
    for (left, right) in history.iter().zip(history.iter().skip(1)) {
        changes.push(right - left);
    }
    changes
}

fn part_1(input: &str) -> i64 {
    let parsed_input = parse(input);
    extrapolate_histories(parsed_input)
}

fn part_2(input: &str) -> i64 {
    let mut parsed_input = parse(input);
    parsed_input.iter_mut().for_each(|x| x.reverse());
    extrapolate_histories(parsed_input)
}

fn main() {
    let input = include_str!("../input.txt");
    println!("part 1: {}, part 2: {}", part_1(input), part_2(input));
}

#[test]
fn example() {
    let example = include_str!("../example.txt");
    let result = part_1(example);
    assert_eq!(result, 114);
}

#[test]
fn example2() {
    let example = include_str!("../example.txt");
    let result = part_2(example);
    assert_eq!(result, 2);
}
