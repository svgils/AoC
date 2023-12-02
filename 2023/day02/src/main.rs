const INPUT: &str = "./input";
use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string(INPUT).unwrap();

    let lines = input.split("\n");

    struct Colors {
        red: u32,
        green: u32,
        blue: u32,
    }

    let mut game_map: HashMap<u16, HashMap<&str, u32>> = HashMap::new();

    lines.for_each(|line| {
        if !line.is_empty() {
            let mut split = line.split(": ");
            let game_num = split
                .next()
                .unwrap()
                .split("Game ")
                .last()
                .unwrap()
                .parse::<u16>()
                .unwrap();

            game_map.insert(
                game_num,
                HashMap::from([("red", 0), ("green", 0), ("blue", 0)]),
            );

            split.last().unwrap().split("; ").for_each(|part| {
                part.split(", ").for_each(|subset| {
                    let mut subset_map = HashMap::new();
                    let mut vals = subset.split(" ");
                    subset_map.insert(
                        vals.clone().last().unwrap(),
                        vals.next().unwrap().parse::<u32>().unwrap(),
                    );
                    subset_map.iter().for_each(|element| {
                        let num = element.1.clone();
                        if element.1 > &game_map[&game_num][element.0] {
                            game_map
                                .get_mut(&game_num)
                                .unwrap()
                                .entry(element.0)
                                .and_modify(|e| *e = num);
                        }
                    });
                })
            })
        }
    });

    let mut possible_games_sum = 0;

    game_map.iter().for_each(|entry| {
        if entry.1["red"] <= 12 && entry.1["green"] <= 13 && entry.1["blue"] <= 14 {
            possible_games_sum += entry.0;
        }
    });
    println!("Part 1: {}", possible_games_sum);

    let mut running_set_power = 0;

    game_map.iter().for_each(|entry| {
        running_set_power += entry.1["red"] * entry.1["green"] * entry.1["blue"];
    });

    println!("Part 2: {}", running_set_power);
}
