use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Target {
    A,
    M,
    S,
    X,
}

impl From<char> for Target {
    fn from(value: char) -> Self {
        match value {
            'a' => Target::A,
            'm' => Target::M,
            's' => Target::S,
            'x' => Target::X,
            _ => unimplemented!(),
        }
    }
}

impl From<&str> for Target {
    fn from(value: &str) -> Self {
        match value {
            "a" => Target::A,
            "m" => Target::M,
            "s" => Target::S,
            "x" => Target::X,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Op {
    Gt(Target, usize),
    Lt(Target, usize),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Rule {
    Accept,
    Reject,
    Compound(Op, Box<Rule>),
    Goto(String),
}

impl Rule {
    fn eval(&self, part: &Part) -> State {
        match self {
            Rule::Accept => State::Accept,
            Rule::Reject => State::Reject,
            Rule::Compound(op, to_go) => match op {
                Op::Gt(target, val) => match target {
                    Target::A => {
                        if part.a > *val {
                            to_go.eval(part)
                        } else {
                            State::Next
                        }
                    }
                    Target::M => {
                        if part.m > *val {
                            to_go.eval(part)
                        } else {
                            State::Next
                        }
                    }
                    Target::S => {
                        if part.s > *val {
                            to_go.eval(part)
                        } else {
                            State::Next
                        }
                    }
                    Target::X => {
                        if part.x > *val {
                            to_go.eval(part)
                        } else {
                            State::Next
                        }
                    }
                },
                Op::Lt(target, val) => match target {
                    Target::A => {
                        if part.a < *val {
                            to_go.eval(part)
                        } else {
                            State::Next
                        }
                    }
                    Target::M => {
                        if part.m < *val {
                            to_go.eval(part)
                        } else {
                            State::Next
                        }
                    }
                    Target::S => {
                        if part.s < *val {
                            to_go.eval(part)
                        } else {
                            State::Next
                        }
                    }
                    Target::X => {
                        if part.x < *val {
                            to_go.eval(part)
                        } else {
                            State::Next
                        }
                    }
                },
            },
            Rule::Goto(s) => State::Continue(s.to_string()),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum State {
    Accept,
    Reject,
    Continue(String),
    Next,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

fn parse(input: &str) -> (HashMap<String, Vec<Rule>>, Vec<Part>) {
    let rules: Vec<_> = input.split("\n\n").collect();
    let mut hm = HashMap::new();
    let workflows = rules[0];
    let parts = rules[1];

    for line in workflows.lines() {
        let split_line: Vec<_> = line.strip_suffix('}').unwrap().split('{').collect();

        let name = split_line[0].to_string();
        let rules = split_line[1];
        let mut rules_vec = vec![];
        for rule in rules.split(',') {
            if rule.contains(':') {
                let chars: Vec<_> = rule.chars().collect();
                let target = { chars[0] };
                let split: Vec<_> = rule.split(':').collect();
                let op = match chars[1] {
                    '>' => {
                        let split_again: Vec<_> = split[0].split('>').collect();
                        let num: usize = split_again[1].parse().unwrap();
                        Op::Gt(Target::from(target), num)
                    }
                    '<' => {
                        let split_again: Vec<_> = split[0].split('<').collect();
                        let num: usize = split_again[1].parse().unwrap();
                        Op::Lt(Target::from(target), num)
                    }
                    _ => unreachable!(),
                };
                let next_rule = split[1];
                if next_rule == "A" {
                    rules_vec.push(Rule::Compound(op, Box::new(Rule::Accept)));
                } else if next_rule == "R" {
                    rules_vec.push(Rule::Compound(op, Box::new(Rule::Reject)));
                } else {
                    rules_vec.push(Rule::Compound(
                        op,
                        Box::new(Rule::Goto(next_rule.to_string())),
                    ));
                }
            } else if rule == "A" {
                rules_vec.push(Rule::Accept);
            } else if rule == "R" {
                rules_vec.push(Rule::Reject);
            } else {
                rules_vec.push(Rule::Goto(rule.to_string()));
            }
        }

        hm.insert(name, rules_vec);
    }

    let mut parts_vec = vec![];

    for line in parts.lines() {
        let stripped_parts = line.strip_prefix('{').unwrap().strip_suffix('}').unwrap();
        let mut part_struct = Part {
            x: 0,
            m: 0,
            a: 0,
            s: 0,
        };

        for part in stripped_parts.split(',') {
            let split_up: Vec<_> = part.split('=').collect();
            let target = split_up[0];
            let value: usize = split_up[1].parse().unwrap();
            let target = Target::from(target);
            match target {
                Target::A => part_struct.a = value,
                Target::M => part_struct.m = value,
                Target::S => part_struct.s = value,
                Target::X => part_struct.x = value,
            }
        }
        parts_vec.push(part_struct);
    }

    (hm, parts_vec)
}

fn match_rule(
    part: &Part,
    curr_rule: &String,
    workflows: &HashMap<String, Vec<Rule>>,
    i: usize,
) -> usize {
    let rules = workflows.get(curr_rule).unwrap();

    match rules[i].eval(part) {
        State::Accept => part.sum(),
        State::Reject => 0,
        State::Continue(next_rule) => match_rule(part, &next_rule, workflows, 0),
        State::Next => match_rule(part, curr_rule, workflows, i + 1),
    }
}

fn part_1(input: &str) -> usize {
    let (workflows, parts) = parse(input);
    let mut total = 0;

    for part in parts {
        let curr_rule = "in".to_string();
        let result = match_rule(&part, &curr_rule, &workflows, 0);
        total += result;
    }

    total
}

fn match_part_2(
    curr_rule: &Rule,
    workflows: &HashMap<String, Vec<Rule>>,
    i: usize,
    possible: usize,
    visited: &mut HashSet<Rule>,
) -> usize {
    if visited.contains(curr_rule) {
        return possible;
    } else {
        visited.insert(curr_rule.clone());
    }
    let mut rule_to_get = "in".to_string();
    match curr_rule {
        Rule::Accept => {}
        Rule::Reject => {}
        Rule::Compound(_, _) => {}
        Rule::Goto(next_rule) => rule_to_get = next_rule.to_string(),
    }
    let rules = workflows.get(&rule_to_get).unwrap();

    if i >= rules.len() {
        return 0;
    }

    match rules[i] {
        Rule::Accept => possible,
        Rule::Reject => 0,
        Rule::Compound(op, ref next_target) => match op {
            Op::Gt(_, val) => {
                match_part_2(
                    &rules[i + 1],
                    workflows,
                    i + 1,
                    possible * (4000 - val),
                    visited,
                ) * match_part_2(next_target, workflows, 0, 1, visited)
            }
            Op::Lt(_, val) => {
                match_part_2(
                    &rules[i + 1],
                    workflows,
                    i + 1,
                    possible * (val - 1),
                    visited,
                ) * match_part_2(next_target, workflows, 0, 1, visited)
            }
        },
        Rule::Goto(_) => match_part_2(&rules[i], workflows, 0, possible, visited),
    }
}

fn part_2(input: &str) -> usize {
    let (workflows, _) = parse(input);
    let mut total = 0;

    total += match_part_2(
        &Rule::Goto("in".to_string()),
        &workflows,
        0,
        1,
        &mut HashSet::new(),
    );

    total
}

fn main() {
    let example = include_str!("../example.txt");
    // let input = include_str!("../input.txt");

    // println!("part 1: {}", part_1(input));
    dbg!(part_2(example));
}

#[test]
fn example_1() {
    let example = include_str!("../example.txt");
    assert_eq!(19114, part_1(example));
}
