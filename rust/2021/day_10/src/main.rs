fn get_input() -> &'static str {
    include_str!("../input.txt").trim()
}

fn parse_input(file: &str) -> Vec<Vec<char>> {
    let mut v = vec![];
    for line in file.trim().lines() {
        let l: Vec<char> = line.chars().collect();
        v.push(l);
    }
    v
}

fn part_1(input: &Vec<Vec<char>>) -> usize {
    let mut score = 0;
    for line in input {
        let mut stack = vec![];
        for c in line {
            match c {
                '[' | '(' | '{' | '<' => stack.push(*c),
                ')' => {
                    if stack.last() == Some(&'(') {
                        stack.pop();
                    } else {
                        score += 3;
                        break;
                    }
                }
                ']' => {
                    if stack.last() == Some(&'[') {
                        stack.pop();
                    } else {
                        score += 57;
                        break;
                    }
                }
                '}' => {
                    if stack.last() == Some(&'{') {
                        stack.pop();
                    } else {
                        score += 1197;
                        break;
                    }
                }
                '>' => {
                    if stack.last() == Some(&'<') {
                        stack.pop();
                    } else {
                        score += 25137;
                        break;
                    }
                }
                _ => (),
            }
        }
    }
    score
}

fn part_2(input: &Vec<Vec<char>>) -> usize {
    let mut incomplete_lines = vec![];
    for line in input {
        let mut stack = vec![];
        let mut push_line = true;
        for c in line {
            match c {
                '[' | '(' | '{' | '<' => stack.push(*c),
                ')' => {
                    if stack.last() == Some(&'(') {
                        stack.pop();
                    } else {
                        push_line = false;
                    }
                }
                ']' => {
                    if stack.last() == Some(&'[') {
                        stack.pop();
                    } else {
                        push_line = false;
                    }
                }
                '}' => {
                    if stack.last() == Some(&'{') {
                        stack.pop();
                    } else {
                        push_line = false;
                    }
                }
                '>' => {
                    if stack.last() == Some(&'<') {
                        stack.pop();
                    } else {
                        push_line = false;
                    }
                }
                _ => (),
            }
        }
        if push_line {
            incomplete_lines.push(stack);
        }
    }

    dbg!(&incomplete_lines);

    let mut scores = vec![];
    for line in incomplete_lines {
        scores.push(score_line(&line));
    }
    scores.sort();
    let mid = scores.len() / 2;
    scores[mid]
}

fn score_line(line: &Vec<char>) -> usize {
    let line = line.iter().rev();
    let mut score = 0;
    for c in line {
        score *= 5;
        match c {
            '(' => score += 1,
            '[' => score += 2,
            '{' => score += 3,
            '<' => score += 4,
            _ => (),
        }
    }
    score
}

fn main() {
    let input = parse_input(get_input());
    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let test_input = r#"
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
"#
        .trim();
        let input = parse_input(test_input);
        assert_eq!(part_1(&input), 26397);
        assert_eq!(part_2(&input), 288957);
    }
}
