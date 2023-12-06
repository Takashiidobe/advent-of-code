fn parse(input: &str) -> (Vec<u64>, Vec<Vec<Vec<u64>>>) {
    let mut blocks = input.split("\n\n");
    let seeds = blocks
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    (
        seeds,
        blocks
            .map(|s| {
                s.lines()
                    .skip(1)
                    .map(|l| l.split_whitespace().map(|s| s.parse().unwrap()).collect())
                    .collect()
            })
            .collect(),
    )
}

#[aoc(day5, part1)]
fn part1(input: &str) -> u64 {
    let (seeds, maps) = parse(input);
    seeds
        .into_iter()
        .map(|seed| {
            maps.iter().fold(seed, |s, m| {
                for m in m {
                    if s >= m[1] && s < m[1] + m[2] {
                        return s - m[1] + m[0];
                    }
                }
                s
            })
        })
        .min()
        .unwrap()
}

#[aoc(day5, part2)]
fn part2(input: &str) -> u64 {
    let (seeds, mut maps) = parse(input);
    for section in &mut maps {
        section.sort_unstable_by_key(|i| i[1]);
    }
    seeds
        .chunks_exact(2)
        .map(|c| {
            apply_sections((c[0], c[1]), &maps)
                .into_iter()
                .min_by_key(|(l, _)| *l)
                .unwrap()
                .0
        })
        .min()
        .unwrap()
}

fn apply_section((mut ilow, mut ilen): (u64, u64), section: &[Vec<u64>]) -> Vec<(u64, u64)> {
    let mut out = Vec::new();
    for mapping in section {
        let (dest, orig, len) = (mapping[0], mapping[1], mapping[2]);
        if orig >= ilow && orig < ilow + ilen {
            if orig > ilow {
                out.push((ilow, orig - ilow));
                ilen -= orig - ilow;
                ilow = orig;
            }
            let l = len.min(ilen);
            out.push((dest, l));
            ilow += l;
            ilen -= l;
        } else if orig < ilow && orig + len > ilow {
            let l = (orig + len - ilow).min(ilen);
            out.push((ilow - orig + dest, l));
            ilow += l;
            ilen -= l;
        }
    }
    if ilen > 0 {
        out.push((ilow, ilen));
    }
    out
}

fn apply_sections(interval: (u64, u64), sections: &[Vec<Vec<u64>>]) -> Vec<(u64, u64)> {
    if sections.is_empty() {
        vec![interval]
    } else {
        apply_section(interval, &sections[0])
            .into_iter()
            .flat_map(|i| apply_sections(i, &sections[1..]))
            .collect()
    }
}
