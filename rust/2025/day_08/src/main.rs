use std::{cmp::Reverse, collections::BinaryHeap};

#[allow(dead_code)]
const EXAMPLE: &str = include_str!("example.txt");
#[allow(dead_code)]
const INPUT: &str = include_str!("input.txt");

#[derive(Debug, PartialEq, Clone, Copy)]
struct Point3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Point3 {
    fn distance(self, other: Point3) -> i64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx * dx + dy * dy + dz * dz
    }
}

impl From<&str> for Point3 {
    fn from(value: &str) -> Self {
        let mut parts = value.split(',').map(str::trim);

        let x: i64 = parts.next().unwrap().parse().unwrap();
        let y: i64 = parts.next().unwrap().parse().unwrap();
        let z: i64 = parts.next().unwrap().parse().unwrap();

        Point3 { x, y, z }
    }
}

#[derive(Debug)]
struct UF {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UF {
    fn new(n: usize) -> Self {
        let parent = (0..n).collect();
        let size = vec![1; n];
        Self { parent, size }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let root = self.find(self.parent[x]);
            self.parent[x] = root;
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let mut ra = self.find(a);
        let mut rb = self.find(b);
        if ra == rb {
            return false;
        }
        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }
        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
        true
    }

    fn component_sizes(&mut self) -> Vec<usize> {
        let n = self.parent.len();
        for i in 0..n {
            let _ = self.find(i);
        }
        let mut sizes = vec![0; n];
        for i in 0..n {
            let r = self.parent[i];
            sizes[r] += 1;
        }
        sizes.into_iter().filter(|&s| s > 0).collect()
    }
}

fn three_largest(points: &[Point3], rounds: i32) -> usize {
    let mut rounds = rounds;
    let n = points.len();

    let mut heap = BinaryHeap::<(Reverse<i64>, usize, usize)>::new();

    for i in 0..n {
        for j in (i + 1)..n {
            let w2 = points[i].distance(points[j]);
            heap.push((Reverse(w2), i, j));
        }
    }

    let mut uf = UF::new(n);

    while let Some((Reverse(_w2), u, v)) = heap.pop()
        && rounds > 0
    {
        let _merged = uf.union(u, v);
        rounds -= 1;
    }

    let mut sizes = uf.component_sizes();
    sizes.sort_by(|a, b| b.cmp(a));
    sizes.iter().take(3).copied().product()
}

fn last_merge(points: &[Point3]) -> i64 {
    let n = points.len();

    let mut heap = BinaryHeap::<(Reverse<i64>, usize, usize)>::new();

    for i in 0..n {
        for j in (i + 1)..n {
            let w2 = points[i].distance(points[j]);
            heap.push((Reverse(w2), i, j));
        }
    }

    let mut uf = UF::new(n);
    let mut components = n;

    let mut last_pair = (0, 0);

    while let Some((Reverse(_w2), u, v)) = heap.pop() {
        if uf.union(u, v) {
            components -= 1;
            last_pair = (u, v);

            if components == 1 {
                break;
            }
        }
    }
    points[last_pair.0].x * points[last_pair.1].x
}

fn parse(input: &str) -> Vec<Point3> {
    input.lines().map(Point3::from).collect()
}

fn part_1(input: &str, rounds: i32) -> usize {
    let parsed = parse(input);
    three_largest(&parsed, rounds)
}

fn part_2(input: &str) -> i64 {
    let parsed = parse(input);
    last_merge(&parsed)
}

fn main() {
    println!("Part 1 Example: {}", part_1(EXAMPLE, 10));
    println!("Part 1: {}", part_1(INPUT, 1000));
    println!("Part 2 Example: {}", part_2(EXAMPLE));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        assert_eq!(parse(EXAMPLE), vec![]);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE, 10), 40);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE), 25272);
    }
}
