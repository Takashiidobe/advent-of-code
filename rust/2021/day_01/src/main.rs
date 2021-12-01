fn get_input() -> Vec<u32> {
    let s = include_str!("../input.txt").trim();

    s.lines().map(|x| x.parse().unwrap()).collect()
}

fn part_1(v: &Vec<u32>) -> usize {
    v.windows(2).filter(|w| w[0] < w[1]).count()
}

fn part_2(v: &Vec<u32>) -> usize {
    let v: Vec<u32> = v.windows(3).map(|w| w.iter().sum()).collect();
    v.windows(2).filter(|w| w[0] < w[1]).count()
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
