use std::collections::{HashMap, VecDeque};

fn get_input() -> &'static str {
    include_str!("../input.txt").trim()
}

static D: [(isize, isize); 8] = [
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1),
    (1, 1),
    (-1, -1),
    (1, -1),
    (-1, 1),
];
type Problem = HashMap<(isize, isize), isize>;

fn parse_input(file: &str) -> Problem {
    file.lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.char_indices()
                .map(|(j, c)| ((i as isize, j as isize), c.to_digit(10).unwrap() as isize))
                .to_owned()
                .collect::<Vec<_>>()
        })
        .collect()
}

fn do_step(problem: &mut Problem) -> isize {
    let mut next: VecDeque<_> = problem.keys().cloned().collect();
    while let Some((x, y)) = next.pop_front() {
        if let Some(o) = problem.get_mut(&(x, y)) {
            *o += 1;
            if *o == 10 {
                next.extend(D.iter().map(|(dx, dy)| (x + dx, y + dy)));
            }
        };
    }

    problem.values_mut().fold(0, |flashes, item| {
        if *item >= 10 {
            *item = 0;
            flashes + 1
        } else {
            flashes
        }
    })
}

fn part_1(mut problem: Problem, steps: usize) -> isize {
    (0..steps).map(|_| do_step(&mut problem)).sum()
}

fn part_2(mut problem: Problem) -> isize {
    (1..).find(|_| do_step(&mut problem) == 100).unwrap()
}

fn main() {
    let input = parse_input(get_input());
    println!("part 1: {}", part_1(input.clone(), 100));
    println!("part 2: {}", part_2(input.clone()));
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
