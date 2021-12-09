fn get_input() -> &'static str {
    include_str!("../input.txt").trim()
}

fn parse_input(file: &str) -> Vec<(Vec<&str>, Vec<&str>)> {
    let mut v = vec![];
    for line in file.lines() {
        let (left, right) = line.split_once(" | ").unwrap();
        let left = left.trim().split_whitespace().collect();
        let right = right.trim().split_whitespace().collect();
        v.push((left, right))
    }
    v
}

fn part_1(input: &Vec<(Vec<&str>, Vec<&str>)>) -> usize {
    let mut count = 0;
    for (_, right) in input {
        for r in right {
            match r.len() {
                2 | 3 | 4 | 7 => count += 1,
                _ => (),
            }
        }
    }
    count
}

fn part_2(input: &Vec<(Vec<&str>, Vec<&str>)>) -> usize {
    let (left, right) = input;
    0
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
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
"#
        .trim();
        let input = parse_input(test_input);
        assert_eq!(part_1(&input), 26);
    }
}
