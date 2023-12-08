use regex::Regex;
use std::{collections::HashMap, fs};

const INPUT: &str = "./input";

fn gcd(mut n1: i64, mut n2: i64) -> i64 {
    while n1 != n2 {
        if n1 > n2 {
            n1 -= n2;
        } else {
            n2 -= n1;
        }
    }
    return n1;
}

fn lcm(set: Vec<i64>) -> i64 {
    return set.iter().fold(set[0], |acc, n| (acc * n) / gcd(acc, *n));
}

fn main() {
    let input = fs::read_to_string(INPUT).unwrap();
    let mut input = input.split("\n\n");
    let directions = input.next().unwrap();
    let re = Regex::new(r"[A-Z]+").unwrap();
    let mut next_nodes: Vec<(&str, i64)> = Vec::new();
    let map: HashMap<&str, (&str, &str)> = input
        .last()
        .unwrap()
        .split("\n")
        .filter_map(|line| {
            if line.is_empty() {
                return None;
            }
            let mut matches = re.find_iter(line);
            let key = matches.next().unwrap().as_str().clone();
            if key.chars().nth(2).unwrap() == 'A' {
                next_nodes.push((key.clone(), 0));
            }
            return Some((
                key,
                (
                    matches.next().unwrap().as_str(),
                    matches.next().unwrap().as_str(),
                ),
            ));
        })
        .collect();

    // Map distance and next Z node for Z node.
    let to_z: HashMap<&str, (&str, i64)> = map
        .iter()
        .filter_map(|node| {
            if node.0.chars().nth(2).unwrap() == 'Z' {
                let mut next_node = node.0;
                let mut steps = 0;
                for direction in directions.chars().cycle() {
                    next_node = if direction == 'L' {
                        &map[next_node].0
                    } else {
                        &map[next_node].1
                    };
                    steps += 1;
                    if next_node.chars().nth(2).unwrap() == 'Z' {
                        return Some((*node.0, (*next_node, steps)));
                    }
                }
                return None;
            } else {
                return None;
            }
        })
        .collect();

    let steps2 = lcm(to_z.iter().map(|i| i.1 .1).collect());

    println!("Part 1: {}", to_z["ZZZ"].1);
    println!("Part 2: {}", steps2);
}
