use edit_distance::edit_distance;

type Pattern = Vec<Vec<char>>;

fn transpose<T>(original: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut transposed = (0..original[0].len()).map(|_| vec![]).collect::<Vec<_>>();

    for original_row in original {
        for (item, transposed_row) in original_row.into_iter().zip(&mut transposed) {
            transposed_row.push(item);
        }
    }

    transposed
}

fn parse(input: &str) -> Vec<Pattern> {
    let mut res = vec![];
    let patterns = input.split("\n\n");

    for pattern in patterns {
        let mut pat = vec![];
        for line in pattern.lines() {
            pat.push(line.chars().collect());
        }
        res.push(pat);
    }
    res
}

fn count_with_multiplier(pattern: &Pattern, multiplier: usize) -> usize {
    for (i, pat) in pattern.iter().enumerate() {
        if i < pattern.len() - 1 && *pat == pattern[i + 1] {
            let left_range = 0..i;
            let right_range = (i + 2)..pattern.len();
            let min_len = left_range.len().min(right_range.len());
            let mut left_side = pattern[i - min_len..i].to_vec();
            let right_side = pattern[i + 2..i + 2 + min_len].to_vec();
            left_side.reverse();
            if left_side == right_side {
                return (i + 1) * multiplier;
            }
        }
    }
    0
}

fn chars_to_string(chars: &[char]) -> String {
    let mut str = String::default();
    for c in chars {
        str.push(*c);
    }
    str
}

fn count_with_multiplier_2(pattern: &Pattern, multiplier: usize) -> usize {
    for (i, pat) in pattern.iter().enumerate() {
        if i < pattern.len() - 1
            && edit_distance(&chars_to_string(pat), &chars_to_string(&pattern[i + 1])) <= 1
        {
            let mut total_edit_distance =
                edit_distance(&chars_to_string(pat), &chars_to_string(&pattern[i + 1]));
            let left_range = 0..i;
            let right_range = (i + 2)..pattern.len();
            let min_len = left_range.len().min(right_range.len());
            let mut left_side = pattern[i - min_len..i].to_vec();
            let right_side = pattern[i + 2..i + 2 + min_len].to_vec();
            left_side.reverse();
            for (left, right) in left_side.iter().zip(right_side.iter()) {
                total_edit_distance +=
                    edit_distance(&chars_to_string(left), &chars_to_string(right));
            }
            if total_edit_distance == 1 {
                return (i + 1) * multiplier;
            }
        }
    }
    0
}

fn part_1(input: &str) -> usize {
    let patterns = parse(input);

    let mut total = 0;

    for pattern in &patterns {
        let transposed = transpose(pattern.to_vec());
        total += count_with_multiplier(pattern, 100);
        total += count_with_multiplier(&transposed, 1);
    }

    total
}

fn part_2(input: &str) -> usize {
    let patterns = parse(input);

    let mut total = 0;

    for pattern in &patterns {
        let transposed = transpose(pattern.to_vec());
        let row_res = count_with_multiplier_2(pattern, 100);
        if row_res > 0 {
            total += row_res;
            continue;
        }
        total += count_with_multiplier_2(&transposed, 1);
    }

    total
}

fn main() {
    let input = include_str!("../input.txt");
    println!("part 1: {}, part 2: {}", part_1(input), part_2(input));
}

#[test]
fn example_1() {
    let input = include_str!("../example.txt");
    assert_eq!(405, part_1(input));
}

#[test]
fn example_2() {
    let input = include_str!("../example.txt");
    assert_eq!(400, part_2(input));
}
