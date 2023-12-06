use regex::Regex;
use std::{
    cmp::{max, min},
    fs,
};

const INPUT: &str = "./input";

struct MapItem {
    source: u64,
    dest: u64,
    range: u64,
}

fn main() {
    let input = fs::read_to_string(INPUT).unwrap();
    let mut sections = input.split("\n\n");
    let digit_re = Regex::new(r"\d+").unwrap();
    let key_re = Regex::new(r"[a-z_]+").unwrap();
    let seeds: Vec<u64> = digit_re
        .find_iter(sections.next().unwrap())
        .map(|x| {
            println!("x: {}", x.as_str());
            return x.as_str().parse().unwrap();
        })
        .collect();
    let mut seed_ranges: Vec<(u64, u64)> = Vec::new();
    for i in (0..seeds.len()).step_by(2) {
        seed_ranges.push((seeds[i].clone(), seeds[i + 1].clone()))
    }
    let maps: Vec<(String, Vec<MapItem>)> = sections
        .map(|section| {
            let mut lines = section.split("\n");
            let key = key_re.find(lines.next().unwrap()).unwrap().as_str();
            return (
                String::from(key),
                lines
                    .filter_map(|l| {
                        let vals: Vec<u64> = digit_re
                            .find_iter(l)
                            .map(|m| m.as_str().parse::<u64>().unwrap())
                            .collect();
                        if vals.len() == 0 {
                            return None;
                        }
                        return Some(MapItem {
                            dest: vals[0],
                            source: vals[1],
                            range: vals[2],
                        });
                    })
                    .collect(),
            );
        })
        .collect();

    let res1: Vec<u64> = seeds
        .iter()
        .map(|seed| {
            maps.iter()
                .fold(seed.clone(), |seed, map: &(String, Vec<MapItem>)| {
                    return map
                        .1
                        .iter()
                        .find_map(|map_item| -> Option<u64> {
                            if (map_item.source..(map_item.source + map_item.range + 1))
                                .contains(&seed)
                            {
                                return Some(map_item.dest + seed - map_item.source);
                            } else {
                                return None;
                            }
                        })
                        .unwrap_or(seed);
                })
        })
        .collect();

    let res2: Vec<(u64, u64)> = maps.iter().fold(seed_ranges, |map_acc, map| {
        map.1.iter().fold(map_acc, |item_acc, item| {
            item_acc
                .iter()
                .flat_map(|range| {
                    let mut new_ranges: Vec<(u64, u64)> = Vec::new();

                    println!("Range: {}:{}", range.0, range.1);
                    println!("Item: {}:{}:{}", item.source, item.dest, item.range);
                    // Seeds that fall below the map range.
                    if range.0 < item.source {
                        let new_range = (range.0, min(range.1, item.source - range.0));
                        // if new_range.0 == 0 {
                        //     println!("Adding 0 in lower");
                        // }
                        new_ranges.push(new_range);
                    }
                    // Seeds that fall inside the map range.
                    if range.0 < item.source + item.range && range.0 + range.1 > item.source {
                        let new_range = (
                            item.dest + max(range.0 as i64 - item.source as i64, 0) as u64,
                            (item.range as i64 - (range.0 as i64 - item.source as i64)) as u64,
                        );
                        // if new_range.0 == 0 {
                        //     println!("Adding 0 in inside");
                        // }
                        new_ranges.push(new_range);
                    }
                    // Seeds that fall above the map range.
                    if range.0 + range.1 > item.source + item.range {
                        let new_range = (
                            max(range.0, item.source + item.range),
                            range.1
                                - max(
                                    0,
                                    (item.source + item.range) as i64 - (range.0 + range.1) as i64,
                                ) as u64,
                        );
                        // if new_range.0 == 0 {
                        //     println!("Adding 0 in above");
                        // };
                        new_ranges.push(new_range);
                    }
                    for range in &mut new_ranges {
                        println!("New Range:{}:{}", range.0, range.1);
                    }
                    return new_ranges;
                })
                .collect::<Vec<_>>()
        })
    });

    res2.iter().for_each(|range| {
        println!("{}:{}", range.0, range.1);
    });

    println!("Part 1: {}", res1.iter().min().unwrap());
    println!(
        "Part 2: {}",
        res2.iter()
            .map(|r| r.0)
            .collect::<Vec<u64>>()
            .iter()
            .min()
            .unwrap()
    );
}
