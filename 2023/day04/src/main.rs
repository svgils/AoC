use regex::Regex;
use std::fs;

const INPUT: &str = "./input";

fn stack_cards(card: usize, orig_cards: &Vec<i32>) -> i32 {
    let mut sum = 1;

    for i in 1..orig_cards[card] + 1 {
        sum += stack_cards(card + i as usize, orig_cards);
    }

    return sum;
}

fn part2(input: &str) -> i32 {
    let re = Regex::new(r"\d+").unwrap();
    let orig_cards: Vec<i32> = input
        .split("\n")
        .filter_map(|line| -> Option<i32> {
            if line.is_empty() {
                return None;
            };
            let mut numbers = line.split(":").last().unwrap().split("|");
            let winning: Vec<i32> = re
                .find_iter(numbers.next().unwrap())
                .map(|x| x.as_str().parse::<i32>().unwrap())
                .collect();
            let mut hits = 0;
            re.find_iter(numbers.last().unwrap()).for_each(|m| {
                let n = m.as_str().parse::<i32>().unwrap();
                if winning.iter().any(|x| *x == n) {
                    hits += 1;
                }
            });
            Some(hits)
        })
        .collect();

    let mut res = 0;
    for i in 0..orig_cards.len() {
        res += stack_cards(i, &orig_cards);
    }

    return res;
}

fn main() {
    let input = fs::read_to_string(INPUT).unwrap();

    let lines = input.split("\n");

    let mut res = 0;

    lines.for_each(|line| {
        if !line.is_empty() {
            let mut numbers = line.split(":").last().unwrap().split("|");
            let re = Regex::new(r"\d+").unwrap();
            let winning: Vec<i32> = re
                .find_iter(numbers.next().unwrap())
                .map(|x| x.as_str().parse::<i32>().unwrap())
                .collect();
            let mut hits = 0;
            re.find_iter(numbers.last().unwrap()).for_each(|m| {
                let n = m.as_str().parse::<i32>().unwrap();
                if winning.iter().any(|x| *x == n) {
                    hits += 1;
                }
            });
            let mut running = if hits > 0 { 1 } else { 0 };
            for _i in 1..hits {
                running *= 2;
            }
            res += running;
        }
    });
    println!("Part1: {}", res);
    println!("Part2: {}", part2(&input));
}
