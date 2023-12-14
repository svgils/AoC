use std::{collections::HashMap, fs};

const INPUT: &str = "./input";

fn tilt_platform(platform: &mut Vec<Vec<char>>, direction: u8) {
    match direction {
        0 => {
            for y in 1..platform.len() {
                for x in 0..platform[0].len() {
                    if platform[y][x] == 'O' {
                        for i in (0..y).rev() {
                            if platform[i][x] == '.' {
                                platform[i][x] = 'O';
                                platform[i + 1][x] = '.';
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
        }
        3 => {
            for x in (0..(platform[0].len() - 1)).rev() {
                for y in 0..platform.len() {
                    if platform[y][x] == 'O' {
                        for i in (x + 1)..platform[0].len() {
                            if platform[y][i] == '.' {
                                platform[y][i] = 'O';
                                platform[y][i - 1] = '.';
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
        }
        2 => {
            for y in (0..(platform.len() - 1)).rev() {
                for x in 0..platform[0].len() {
                    if platform[y][x] == 'O' {
                        for i in (y + 1)..platform.len() {
                            if platform[i][x] == '.' {
                                platform[i][x] = 'O';
                                platform[i - 1][x] = '.';
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
        }
        1 => {
            for x in 1..platform[0].len() {
                for y in 0..platform.len() {
                    if platform[y][x] == 'O' {
                        for i in (0..x).rev() {
                            if platform[y][i] == '.' {
                                platform[y][i] = 'O';
                                platform[y][i + 1] = '.';
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
}

fn main() {
    let platform: Vec<Vec<char>> = fs::read_to_string(INPUT)
        .unwrap()
        .split("\n")
        .filter_map(|l| {
            if l.is_empty() {
                return None;
            }
            Some(l.chars().collect())
        })
        .collect();

    let calculate_beam_weight = |p: &Vec<Vec<char>>| {
        p.iter().rev().enumerate().fold(0, |acc, row| {
            acc + (row
                .1
                .iter()
                .fold(0, |acc2, tile| acc2 + (1 & ((*tile == 'O') as i32)))
                * (row.0 + 1) as i32)
        })
    };

    let mut platform1 = platform.clone();
    tilt_platform(&mut platform1, 0);
    let weight1 = calculate_beam_weight(&platform1);

    let mut tilt_cache: HashMap<Vec<Vec<char>>, (usize, usize)> = HashMap::new();

    let mut platform2 = platform.clone();
    let mut i = 0usize;
    let cycles: usize = 1000000000 * 4;
    while i < cycles {
        tilt_platform(&mut platform2, (i % (4 as usize)) as u8);
        if tilt_cache.contains_key(&platform2) && tilt_cache[&platform2].0 == i % 4 {
            let occ_1 = tilt_cache[&platform2].1 as i64;
            let remaining_cycles = (cycles - i) as i64;
            i = i + (remaining_cycles - (remaining_cycles % (i as i64 - occ_1))) as usize + 1;
        } else {
            tilt_cache.insert(platform2.clone(), (i % 4, i));
            i += 1;
        }
    }

    let weight2 = calculate_beam_weight(&platform2);

    println!("Part 1: {}", weight1);
    println!("Part 2: {}", weight2);
}
