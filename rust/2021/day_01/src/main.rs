fn get_input() -> Vec<usize> {
    let s = include_str!("../input.txt").trim();

    s.split('\n')
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

fn part_1(v: &Vec<usize>) -> usize {
    let zipped = v.iter().zip(v.iter().skip(1));
    zipped.fold(0, |acc, x| if x.1 > x.0 { acc + 1 } else { acc })
}

fn part_2(v: &Vec<usize>) -> usize {
    let zipped = (0..v.len()).skip(2).zip((0..v.len()).skip(3));
    let mut count = 0;
    for (i, j) in zipped {
        let first = v[i] + v[i - 1] + v[i - 2];
        let second = v[j] + v[j - 1] + v[j - 2];
        if second > first {
            count += 1;
        }
    }
    count
}

fn main() {
    let v = get_input();
    println!("part 1: {}", part_1(&v));
    println!("part 2: {}", part_2(&v));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let v1 = vec![100, 200, 300, 400, 500, 600];
        assert_eq!(part_1(&v1), 5);
        assert_eq!(part_2(&v1), 3);
    }

    #[test]
    fn test_2() {
        let v2 = vec![
            159, 158, 174, 196, 197, 194, 209, 213, 214, 222, 223, 228, 229, 236, 237, 238, 241,
            248, 255, 256,
        ];
        assert_eq!(part_1(&v2), 17);
        assert_eq!(part_2(&v2), 17);
    }
}
