use regex::Regex;

fn parse(input: &str) -> Vec<Vec<i64>> {
    let re = Regex::new(r"(\d+)").unwrap();
    let mut res = vec![];
    for line in input.lines() {
        let captures: Vec<i64> = re
            .find_iter(line)
            .map(|m| m.as_str().parse().unwrap())
            .collect();
        res.push(captures);
    }
    res
}

fn main() {
    let example = include_str!("../example.txt");
    let res = parse(example);
    dbg!(res);
}
