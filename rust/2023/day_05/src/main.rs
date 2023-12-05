use regex::Regex;
use std::collections::BTreeMap;

fn parse_part_1(input: &str) -> (Vec<u64>, BTreeMap<&str, Vec<(u64, u64, u64)>>) {
    let re = Regex::new(r"\b(\d+)\b").unwrap();
    let sections: Vec<_> = input.split("\n\n").collect();

    let mut seeds = vec![];
    let mut maps = BTreeMap::new();

    for section in sections {
        let section = section.trim();

        let split_section: Vec<_> = section.split(':').map(|s| s.trim()).collect();

        let ranges: Vec<_> = split_section[1].split('\n').collect();
        if split_section[0] == "seeds" {
            let parsed_seeds = re
                .find_iter(ranges[0])
                .map(|m| m.as_str().parse().unwrap())
                .collect();
            seeds = parsed_seeds;
        } else {
            let map_name = {
                let split_map: Vec<_> = split_section[0].split_ascii_whitespace().collect();
                split_map[0]
            };
            let mut vec_ranges = vec![];
            for range in ranges {
                let parsed: Vec<u64> = re
                    .find_iter(range)
                    .map(|m| m.as_str().parse().unwrap())
                    .collect();
                let destination_start = parsed[0];
                let source_start = parsed[1];
                let length = parsed[2];

                vec_ranges.push((source_start, destination_start, length));
            }
            maps.insert(map_name, vec_ranges);
        }
    }

    (seeds, maps)
}

fn parse_part_2(input: &str) -> (Vec<(u64, u64)>, BTreeMap<&str, Vec<(u64, u64, u64)>>) {
    let re = Regex::new(r"\b(\d+)\b").unwrap();
    let sections: Vec<_> = input.split("\n\n").collect();

    let mut seeds = vec![];
    let mut maps = BTreeMap::new();

    for section in sections {
        let section = section.trim();

        let split_section: Vec<_> = section.split(':').map(|s| s.trim()).collect();

        let ranges: Vec<_> = split_section[1].split('\n').collect();
        if split_section[0] == "seeds" {
            let parsed_seeds: Vec<u64> = re
                .find_iter(ranges[0])
                .map(|m| m.as_str().parse().unwrap())
                .collect();

            let mut i = 0;
            let mut j = 1;

            while j < parsed_seeds.len() {
                seeds.push((parsed_seeds[i], parsed_seeds[j]));
                i += 2;
                j += 2;
            }
            seeds.sort();
        } else {
            let map_name = {
                let split_map: Vec<_> = split_section[0].split_ascii_whitespace().collect();
                split_map[0]
            };
            let mut vec_ranges = vec![];
            for range in ranges {
                let parsed: Vec<u64> = re
                    .find_iter(range)
                    .map(|m| m.as_str().parse().unwrap())
                    .collect();
                let destination_start = parsed[0];
                let source_start = parsed[1];
                let length = parsed[2];

                vec_ranges.push((source_start, destination_start, length));
            }
            vec_ranges.sort();
            maps.insert(map_name, vec_ranges);
        }
    }

    (seeds, maps)
}

fn map_lookup(seed: u64, s: &str, maps: &BTreeMap<&str, Vec<(u64, u64, u64)>>) -> u64 {
    let ranges = maps.get(s).unwrap();
    for (source_start, destination_start, length) in ranges {
        if (source_start..&(source_start + length)).contains(&&seed) {
            let offset = seed - source_start;
            return destination_start + offset;
        }
    }

    seed
}

fn part_1(input: &str) -> u64 {
    // for all of the seeds, iterate through and find the smallest location number
    let (seeds, maps) = parse_part_1(input);
    let mut min_location = u64::MAX;
    for seed in seeds {
        let seed_to_soil = map_lookup(seed, "seed-to-soil", &maps);
        let soil_to_fertilizer = map_lookup(seed_to_soil, "soil-to-fertilizer", &maps);
        let fertilizer_to_water = map_lookup(soil_to_fertilizer, "fertilizer-to-water", &maps);
        let water_to_light = map_lookup(fertilizer_to_water, "water-to-light", &maps);
        let light_to_temperature = map_lookup(water_to_light, "light-to-temperature", &maps);
        let temperature_to_humidity =
            map_lookup(light_to_temperature, "temperature-to-humidity", &maps);
        let humidity_to_location =
            map_lookup(temperature_to_humidity, "humidity-to-location", &maps);
        min_location = min_location.min(humidity_to_location);
    }

    min_location
}

fn map_lookup_part_2(
    seeds: (u64, u64),
    s: &str,
    maps: &BTreeMap<&str, Vec<(u64, u64, u64)>>,
) -> u64 {
    let ranges = maps.get(s).unwrap();
    let (seed_start, seed_increment) = seeds;
    let seed_end = seed_start + seed_increment;
    for (source_start, destination_start, length) in ranges {
        let (source_end, destination_end) = (source_start + length, destination_start + length);
        // if the left's end is <= right start, we have a match
        // then we have return the max of left start or right start

        if seed_start <= source_end {
            dbg!(seed_start, seed_end, source_start, source_end);
            let minimum_seed = seed_start.max(*source_start);
            let offset = minimum_seed - seed_start;
            dbg!(
                destination_start,
                destination_end,
                offset,
                destination_start + offset
            );
            return destination_start + offset;
        }
    }

    seeds.0
}

fn part_2(input: &str) -> u64 {
    let (seeds, maps) = parse_part_2(input);
    dbg!(&seeds);
    let mut min_location = u64::MAX;
    for seed in seeds {
        let seed_to_soil = map_lookup_part_2(seed, "seed-to-soil", &maps);
        dbg!(seed_to_soil);
        let soil_to_fertilizer = map_lookup(seed_to_soil, "soil-to-fertilizer", &maps);
        let fertilizer_to_water = map_lookup(soil_to_fertilizer, "fertilizer-to-water", &maps);
        let water_to_light = map_lookup(fertilizer_to_water, "water-to-light", &maps);
        let light_to_temperature = map_lookup(water_to_light, "light-to-temperature", &maps);
        let temperature_to_humidity =
            map_lookup(light_to_temperature, "temperature-to-humidity", &maps);
        let humidity_to_location =
            map_lookup(temperature_to_humidity, "humidity-to-location", &maps);
        min_location = min_location.min(humidity_to_location);
    }

    min_location
}

fn main() {
    let input = include_str!("../input.txt");
    let example = include_str!("../example.txt");
    part_2(example);
    // println!("part 1: {}, part_2: {}", part_1(input), part_2(input));
}

#[test]
fn example_pt_1() {
    let example = include_str!("../example.txt");
    let result = part_1(example);
    assert_eq!(result, 35);
}

#[test]
fn example_pt_2() {
    let example = include_str!("../example.txt");
    let result = part_2(example);
    assert_eq!(result, 46);
}
