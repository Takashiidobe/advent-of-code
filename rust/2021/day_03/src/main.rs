fn part_1() -> u32 {
    let file = include_str!("../input.txt");

    let length: u32 = file.lines().clone().count().try_into().unwrap();

    let solution = file
        .lines()
        .map(|e| e.chars().map(|e| e.to_digit(10).unwrap() as u32).collect())
        .fold(vec![0; 12], |sum, e: Vec<u32>| -> Vec<u32> {
            sum.iter()
                .zip(e)
                .map(|(sum_el, curr_el)| sum_el + curr_el)
                .collect()
        })
        .iter()
        .map(|e| if e >= &(length / 2) { 1 } else { 0 })
        .fold([String::new(), String::new()], |[s1, s2], e| {
            [
                format!("{}{}", &s1, e),
                format!("{}{}", &s2, if e == 1 { 0 } else { 1 }),
            ]
        })
        .map(|e| u32::from_str_radix(&e, 2).unwrap())
        .into_iter()
        .product();

    solution
}

fn part_2() -> u32 {
    let file = include_str!("../input.txt");

    let lines: Vec<Vec<u32>> = file
        .lines()
        .map(|e| e.chars().map(|e| e.to_digit(10).unwrap() as u32).collect())
        .collect();

    let mut oxyco2 = [lines.clone(), lines.clone()];

    let result = oxyco2
        .iter_mut()
        .enumerate()
        .map(|(i, list)| {
            let mut cursor = 0;
            while cursor < 12 {
                let half_length: u32 = (list.len() / 2).try_into().unwrap();
                let common = list.iter().fold(0, |sum, e| sum + e[cursor]) >= half_length;

                let mut common: u32 = common.try_into().unwrap();

                if i == 1 {
                    common = if common == 1 { 0 } else { 1 };
                }

                if list.len() > 1 {
                    list.retain(|number| number[cursor] == common);
                }

                cursor += 1;
            }

            list.iter()
                .map(|e| e.iter().map(|e| e.to_string()).collect())
                .collect()
        })
        .map(|e: Vec<String>| u32::from_str_radix(e.first().unwrap(), 2).unwrap())
        .product();

    result
}

fn main() {
    println!("part 1: {}", part_1());
    println!("part 2: {}", part_2());
}
