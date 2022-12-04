fn get_input() -> &'static str {
    include_str!("../input.txt").trim()
}

type Input = Vec<Vec<u32>>;

fn parse_input(file: &str) -> Input {
    println!("{}", file);
    file.lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn part_1(input: Input) -> isize {
    // lowest input = start at 0, 0
    // make your way to m - 1, n - 1
    // only go down or right
    // if you find a visited square, then find the cached version of the value.
    let m = input.len();
    let n = input[0].len();

    let mut dp = vec![vec![u32::MAX; n]; m];
    println!("{:?}", input);
    println!("{:?}", dp);
    0
}

// fn part_2(problem: Input) -> isize {
//     0
// }

fn main() {
    let input = parse_input(get_input());
    println!("part 1: {}", part_1(input));
    //     println!("part 2: {}", part_2(input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let test_input = r#"
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
"#
        .trim();
        let input = parse_input(test_input);
        assert_eq!(part_1(input.clone(), 100), 1656);
        assert_eq!(part_2(input.clone()), 195);
    }
}
