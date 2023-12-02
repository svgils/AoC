use regex::Regex;
use std::{collections::HashMap, fs};

const INPUT: &str = "./input";

fn main() {
    let digit_map: HashMap<&str, &str> = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    let input = fs::read_to_string(INPUT).unwrap();

    let split = input.split("\n");
    let mut digits: Vec<Vec<&str>> = Vec::new();
    let re = Regex::new(r"\d|three|one|two|four|five|six|seven|eight|nine").unwrap();
    split.for_each(|line| {
        if !line.is_empty() {
            let mut results: Vec<&str> = vec![];
            let mut idx = 0;
            while idx < line.len() {
                let part = &line[idx..line.len()];
                let res = re.find(part);
                if let Some(r) = res {
                    let mut res = r.as_str();
                    if digit_map.contains_key(res) {
                        res = digit_map[res];
                    }
                    results.push(res);
                    idx = if r.len() > 1 {
                        idx + r.len() - 1
                    } else {
                        idx + r.len()
                    };
                } else {
                    break;
                };
            }
            digits.push(results)
        }
    });
    let mut sum: u32 = 0;
    digits.iter().for_each(|x| {
        sum += format!("{}{}", x.first().unwrap(), x.last().unwrap())
            .parse::<u32>()
            .unwrap()
    });
    println!("{}", sum);
}
