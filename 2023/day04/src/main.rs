use regex::Regex;
use std::fs;

const INPUT: &str = "./input";

fn stack_cards(card: usize, orig_cards: &Vec<u32>) -> u32 {
    let mut sum = 1;

    for i in 1..orig_cards[card] + 1 {
        sum += stack_cards(card + i as usize, orig_cards);
    }

    return sum;
}

fn main() {
    let input = fs::read_to_string(INPUT).unwrap();
    let mut part1_res = 0;
    let mut part2_res = 0;
    let re = Regex::new(r"\d+").unwrap();

    let orig_cards: Vec<u32> = input
        .split("\n")
        .filter_map(|line| -> Option<u32> {
            if line.is_empty() {
                return None;
            };
            let mut numbers = line.split(":").last().unwrap().split("|");
            let winning: Vec<u32> = re
                .find_iter(numbers.next().unwrap())
                .map(|x| x.as_str().parse::<u32>().unwrap())
                .collect();
            let mut hits = 0;
            re.find_iter(numbers.last().unwrap()).for_each(|m| {
                let n = m.as_str().parse::<u32>().unwrap();
                if winning.iter().any(|x| *x == n) {
                    hits += 1;
                }
            });
            Some(hits)
        })
        .collect();

    for i in 0..orig_cards.len() {
        part2_res += stack_cards(i, &orig_cards);
        if orig_cards[i] > 0 {
            part1_res += u32::pow(2, orig_cards[i] - 1);
        }
    }

    println!("Part1: {}", part1_res);
    println!("Part2: {}", part2_res);
}
