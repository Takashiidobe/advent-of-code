use std::collections::HashMap;

use num::integer::lcm;

fn lcm_vec(input: &[usize]) -> usize {
    if input.len() == 2 {
        lcm(input[0], input[1])
    } else {
        let first_num = input[0];
        let the_rest: Vec<usize> = input.iter().skip(1).copied().collect();
        let second_num = lcm_vec(&the_rest);
        lcm(first_num, second_num)
    }
}

fn parse(input: &str) -> (Vec<char>, HashMap<String, (String, String)>) {
    let split_input: Vec<_> = input.split("\n\n").collect();
    let instructions: Vec<_> = split_input[0].chars().collect();
    let mut hashmap = HashMap::default();

    for line in split_input[1].lines() {
        let split_line: Vec<_> = line.split(" = ").collect();
        let node = split_line[0];
        let left_split: Vec<_> = split_line[1]
            .split(", ")
            .map(|n| n.replace(['(', ')'], ""))
            .collect();
        hashmap.insert(
            node.to_string(),
            (left_split[0].clone(), left_split[1].clone()),
        );
    }

    (instructions, hashmap)
}

fn part_1(input: &str) -> usize {
    let (instructions, graph) = parse(input);

    let mut steps = 0;
    let mut curr_node = "AAA".to_string();

    for instruction in instructions.into_iter().cycle() {
        steps += 1;
        match instruction {
            'R' => {
                let new_node = &graph.get(&curr_node).unwrap().1;
                curr_node = new_node.clone();
                if curr_node.as_str() == "ZZZ" {
                    return steps;
                }
            }
            'L' => {
                let new_node = &graph.get(&curr_node).unwrap().0;
                curr_node = new_node.clone();
                if new_node.as_str() == "ZZZ" {
                    return steps;
                }
            }
            _ => unreachable!(),
        }
    }
    steps
}

fn parse_part_2(input: &str) -> (Vec<char>, HashMap<String, (String, String)>, Vec<String>) {
    let split_input: Vec<_> = input.split("\n\n").collect();
    let instructions: Vec<_> = split_input[0].chars().collect();
    let mut hashmap = HashMap::default();
    let mut starting_nodes = vec![];

    for line in split_input[1].lines() {
        let split_line: Vec<_> = line.split(" = ").collect();
        let node = split_line[0].to_string();
        let left_split: Vec<_> = split_line[1]
            .split(", ")
            .map(|n| n.replace(['(', ')'], ""))
            .collect();
        if node.ends_with('A') {
            starting_nodes.push(node.clone());
        }
        hashmap.insert(node, (left_split[0].clone(), left_split[1].clone()));
    }

    (instructions, hashmap, starting_nodes)
}

// it's a least common multiple angle
// basically, for each node, count until they become
fn part_2(input: &str) -> usize {
    let (instructions, graph, starting_nodes) = parse_part_2(input);

    let mut total_steps = vec![];

    for node in starting_nodes {
        let mut steps = 0;
        let mut curr_node = node;
        for instruction in instructions.iter().cycle() {
            steps += 1;
            match instruction {
                'R' => {
                    let new_node = &graph.get(&curr_node).unwrap().1;
                    if new_node.ends_with('Z') {
                        total_steps.push(steps);
                        break;
                    }
                    curr_node = new_node.to_string();
                }
                'L' => {
                    let new_node = &graph.get(&curr_node).unwrap().0;
                    if new_node.ends_with('Z') {
                        total_steps.push(steps);
                        break;
                    }
                    curr_node = new_node.to_string();
                }
                _ => unreachable!(),
            }
        }
    }
    lcm_vec(&total_steps)
}

fn main() {
    let input = include_str!("../input.txt");
    dbg!(part_2(input));
}

#[test]
fn example_1_part_1() {
    let example = include_str!("../example1.txt");
    assert_eq!(2, part_1(example));
}

#[test]
fn example_2_part_1() {
    let example = include_str!("../example2.txt");
    assert_eq!(6, part_1(example));
}

#[test]
fn example_3_part_2() {
    let example = include_str!("../example2.txt");
    assert_eq!(6, part_2(example));
}
