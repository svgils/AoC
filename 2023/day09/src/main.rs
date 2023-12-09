use regex::Regex;
use std::fs;

const INPUT: &str = "./input";

fn main() {
    let re = Regex::new(r"-?\d+").unwrap();
    let res = fs::read_to_string(INPUT)
        .unwrap()
        .split("\n")
        .fold((0, 0), |acc, line| {
            if line.is_empty() {
                return acc;
            }
            let nums: Vec<i64> = re
                .find_iter(line)
                .map(|m| m.as_str().parse().unwrap())
                .collect::<Vec<i64>>();
            let mut diffs: Vec<Vec<i64>> = vec![nums];
            while !(diffs.last().unwrap().iter().all(|x| *x == 0)) {
                diffs.push(
                    diffs
                        .last()
                        .unwrap()
                        .windows(2)
                        .map(|x| x[1] - x[0])
                        .collect(),
                );
            }
            diffs.reverse();
            let next = diffs.iter().fold((0, 0), |acc2, x| {
                ((acc2.0 + x.last().unwrap()), (x[0] - acc2.1))
            });
            return (acc.0 + next.0, next.1 + acc.1);
        });

    println!("Part 1: {}", res.0);
    println!("Part 2: {}", res.1);
}
