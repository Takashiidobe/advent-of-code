use std::collections::HashMap;

fn parse() -> Vec<u32> {
    vec![0, 6, 1, 7, 2, 19, 20]
}

fn main() {
    let mut nums = parse();

    let mut map = HashMap::new();

    for (i, num) in nums.iter().enumerate() {
        map.insert(num, (-1, i));
    }
    println!("{:?}", map);

    for i in 0..(2020 - nums.len()) {
        if map.contains_key(&i) {
           if map.get(&i).unwrap().1 == -1 {
                map.get(&i).unwrap().1 = i;
           }
        } else {
            map.insert(
        }
    }
}
