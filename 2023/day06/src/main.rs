use libm::{ceil, floor};
use regex::Regex;
use std::fs;

const INPUT1: &str = "./input1";
const INPUT2: &str = "./input2";

fn solve(input: &str) -> i64 {
    let digit_re = Regex::new(r"\d+").unwrap();
    let input: Vec<Vec<i64>> = fs::read_to_string(input)
        .unwrap()
        .split("\n")
        .map(|line| {
            digit_re
                .find_iter(line)
                .map(|x| x.as_str().parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect();

    let races: Vec<(i64, i64)> = input[0]
        .iter()
        .zip(input[1].iter())
        .map(|pair| (pair.0.clone(), pair.1.clone()))
        .collect();

    return races.iter().fold(1, |acc, race| {
        let time = race.0 as f64;
        let record = race.1 as f64;
        return acc
            * ((ceil((time + f64::sqrt((-time).powf(2f64) - (4f64 * record))) / 2f64) as i64)
                - (floor((time - f64::sqrt((-time).powf(2f64) - (4f64 * record))) / 2f64) as i64)
                - 1);
    });
}

fn main() {
    println!("Part 1: {}", solve(INPUT1));
    println!("Part 2: {}", solve(INPUT2));
}
