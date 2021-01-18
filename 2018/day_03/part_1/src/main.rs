// the first line is the first line
// second line is the second line
// generate the intersections from the central port, and put them in a map

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_input() -> Vec<String> {
    let reader = BufReader::new(File::open("../input.txt").expect("Cannot open file.txt"));

    let mut v = vec![];
    for line in reader.lines() {
        let mut line_fragment = "".to_string();
        for l in line.unwrap().split_whitespace() {
            line_fragment += l
        }
        v.push(line_fragment);
    }
    v
}

fn split_line_to_instructions(line: String) -> Vec<String> {
    let m: Vec<_> = line.split(",").map(|s| s.to_string()).collect();
    m
}

fn get_intersections(first: Vec<String>, second: Vec<String>) -> Vec<(i32, i32)> {
    // return a vec of tuples with the intersection of the two.
    // fill a 2d grid of a vector for both.
}

fn main() {
    let output = get_input();
    let first_line = output[0].to_owned();
    let second_line = output[1].to_owned();
    let first_instructions = split_line_to_instructions(first_line);
    let second_instructions = split_line_to_instructions(second_line);

    println!("{:#?}", first_instructions);
    println!("{:#?}", second_instructions);
}
