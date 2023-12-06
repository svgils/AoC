use regex::Regex;
use std::{
    cmp::{max, min},
    fs,
};

const INPUT: &str = "./input";

struct MapItem {
    source: i64,
    dest: i64,
    range: i64,
}

fn main() {
    let input = fs::read_to_string(INPUT).unwrap();
    let mut sections = input.split("\n\n");
    let digit_re = Regex::new(r"\d+").unwrap();
    let key_re = Regex::new(r"[a-z_]+").unwrap();
    let seeds: Vec<i64> = digit_re
        .find_iter(sections.next().unwrap())
        .map(|x| x.as_str().parse().unwrap())
        .collect();
    let mut seed_ranges: Vec<(i64, i64)> = Vec::new();
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
                        let vals: Vec<i64> = digit_re
                            .find_iter(l)
                            .map(|m| m.as_str().parse().unwrap())
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

    let res1: Vec<i64> = seeds
        .iter()
        .map(|seed| {
            maps.iter()
                .fold(seed.clone(), |seed, map: &(String, Vec<MapItem>)| {
                    return map
                        .1
                        .iter()
                        .find_map(|map_item| -> Option<i64> {
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

    // Loop over every map and collect back into a list of ranges.
    let res2: Vec<(i64, i64)> = maps.iter().fold(seed_ranges, |map_acc, map| {
        let mut moved_ranges: Vec<(i64, i64)> = Vec::new();
        // Loop over every item in map and add moved ranges to seperate list.
        let mut split_ranges = map.1.iter().fold(map_acc, |item_acc, item| {
            // Loop over every range and split into 0 or more new ranges,
            // to be consumed by the next map item.
            item_acc
                .iter()
                .flat_map(|range| {
                    let mut new_ranges: Vec<(i64, i64)> = Vec::new();

                    // Seeds that fall below the map range.
                    if range.0 < item.source {
                        let new_range = (range.0, min(range.1, item.source - range.0));
                        new_ranges.push(new_range);
                    }
                    // Seeds that fall inside the map range.
                    if range.0 < item.source + item.range && range.0 + range.1 > item.source {
                        let new_range = (
                            item.dest + max(range.0 - item.source, 0),
                            ((range.0 + range.1).clamp(item.source, item.source + item.range)
                                - range.0.clamp(item.source, item.source + item.range)),
                        );
                        // Move to moved_ranges so this range doesnt get evaluated on next iteration.
                        moved_ranges.push(new_range);
                    }
                    // Seeds that fall above the map range.
                    if range.0 + range.1 > item.source + item.range {
                        let new_range = (
                            max(range.0, item.source + item.range),
                            min(range.1, (range.0 + range.1) - (item.source + item.range)),
                        );
                        new_ranges.push(new_range);
                    }
                    return new_ranges;
                })
                .collect::<Vec<_>>()
        });
        // Pass both the moved ranges and the unmoved ranges to the next map.
        split_ranges.append(&mut moved_ranges);
        return split_ranges;
    });

    println!("Part 1: {}", res1.iter().min().unwrap());
    println!(
        "Part 2: {}",
        res2.iter()
            .map(|r| r.0)
            .collect::<Vec<i64>>()
            .iter()
            .min()
            .unwrap()
    );
}
