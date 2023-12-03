use regex::Regex;
use std::{collections::HashMap, fs};

const INPUT: &str = "./input";

fn find_adj(input: &str, m: regex::Match, length: usize, pattern: &str) -> Vec<usize> {
    let mut symbols = Vec::<usize>::new();

    let re = Regex::new(pattern).unwrap();
    let line_start = m.start() as i64 / length as i64 * length as i64;
    let line_end = line_start + length as i64 - 2;

    // Search above
    let start = (m.start() as i64 - length as i64 - 1).clamp(
        (line_start - length as i64).clamp(0, input.len() as i64),
        (line_end - length as i64).clamp(0, input.len() as i64),
    ) as usize;
    let end = (m.end() as i64 - length as i64).clamp(
        (line_start - length as i64).clamp(0, input.len() as i64),
        (line_end - length as i64).clamp(0, input.len() as i64),
    ) as usize;

    if let Some(s) = re.find(&input[start..end + 1]) {
        symbols.push(s.start() + start);
    }

    // Search below
    if m.start() + length < input.len() {
        let start = (m.start() as i64 + length as i64 - 1)
            .clamp(line_start + length as i64, line_end + length as i64)
            as usize;
        let end = (m.end() as i64 + length as i64)
            .clamp(line_start + length as i64, line_end + length as i64) as usize;

        if let Some(s) = re.find(&input[start..end + 1]) {
            symbols.push(s.start() + start);
        }
    }

    // Search left
    if m.start() > line_start as usize {
        if re
            .find(&input[(m.start() as i64 - 1) as usize..m.start()])
            .is_some()
        {
            symbols.push(m.start() - 1);
        }
    }

    // Search right
    if m.end() < line_end as usize {
        if re.find(&input[m.end()..m.end() + 1]).is_some() {
            symbols.push(m.end());
        }
    }

    return symbols;
}

fn part1(input: &str) -> usize {
    let length = input.find("\n").unwrap() + 1;
    let re = Regex::new(r"\d+").unwrap();
    let mut number_sum: usize = 0;
    re.find_iter(&input).for_each(|m| {
        let matches = find_adj(input, m, length, r"[^\.\d]");

        if matches.len() > 0 {
            number_sum += m.as_str().parse::<usize>().unwrap()
        }
    });
    return number_sum;
}

fn part2(input: &str) -> usize {
    let mut gear_map: HashMap<usize, Vec<u16>> = HashMap::new();
    let length = input.find("\n").unwrap() + 1;
    let re = Regex::new(r"\d+").unwrap();
    re.find_iter(input).for_each(|m| {
        let symbols = find_adj(input, m, length, r"\*");
        symbols.iter().for_each(|s| {
            let entry = gear_map.entry(*s).or_insert(vec![]);
            entry.push(m.as_str().parse::<u16>().unwrap());
        });
    });

    let mut sum: usize = 0;

    gear_map.iter().for_each(|entry| {
        let mut ratio: usize = 0;
        if entry.1.len() > 1 {
            entry.1.iter().for_each(|r| {
                ratio = if ratio == 0 {
                    *r as usize
                } else {
                    ratio * *r as usize
                };
            });
        }
        sum += ratio;
    });

    return sum;
}

fn main() {
    let input = fs::read_to_string(INPUT).unwrap();
    let number_sum = part1(&input);

    println!("Part 1: {}", number_sum);
    println!("Part 2: {}", part2(&input));
}
