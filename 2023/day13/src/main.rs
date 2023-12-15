use std::{cmp::min, fs};

const INPUT: &str = "./input";

fn main() {
    let grids: Vec<Vec<Vec<bool>>> = fs::read_to_string(INPUT)
        .unwrap()
        .split("\n\n")
        .filter_map(|section| {
            if section.is_empty() {
                return None;
            }
            return Some(
                section
                    .split("\n")
                    .filter_map(|l| {
                        if l.is_empty() {
                            return None;
                        }
                        return Some(
                            l.chars()
                                .map(|c| if c == '#' { true } else { false })
                                .collect(),
                        );
                    })
                    .collect(),
            );
        })
        .collect();

    let (res1, res2) = grids.iter().fold((0, 0), |acc, grid| {
        let mut new_res = (0, 0);
        let mut p2_satisfied = false;
        for x in 0..(grid[0].len() - 1) {
            let mut equal = true;
            let mut differrent = 0;
            for i in 0..min(x + 1, grid[0].len() - x - 1) {
                for y in 0..grid.len() {
                    let eq = !(grid[y][x - i] ^ grid[y][x + i + 1]);
                    if !eq {
                        differrent += 1;
                    }
                    equal &= eq;
                }
            }
            if differrent == 1 && !p2_satisfied {
                new_res.1 = acc.1 + x + 1;
                p2_satisfied = true;
            }
            if equal {
                new_res.0 = acc.0 + x + 1;
            }
        }

        for y in 0..(grid.len() - 1) {
            let mut equal = true;
            let mut differrent = 0;
            for i in 0..min(y + 1, grid.len() - y - 1) {
                for x in 0..grid[0].len() {
                    let eq = !(grid[y - i][x] ^ grid[y + i + 1][x]);
                    if !eq {
                        differrent += 1;
                    }
                    equal &= eq;
                }
            }
            if differrent == 1 && !p2_satisfied {
                new_res.1 = acc.1 + ((y + 1) * 100);
                p2_satisfied = true;
            }
            if equal {
                new_res.0 = acc.0 + ((y + 1) * 100);
            }
        }
        return new_res;
    });

    println!("Part 1: {}", res1);
    println!("Part 2: {}", res2);
}
