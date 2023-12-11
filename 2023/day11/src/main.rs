use std::fs;

const INPUT: &str = "./input";

fn solve(
    h_fold: &Vec<bool>,
    v_fold: &Vec<bool>,
    points: &Vec<(usize, usize)>,
    padding: usize,
) -> usize {
    let new_points = points
        .iter()
        .map(|p| {
            let mut empty_v_rows = 0;
            for i in 0..p.0 {
                if !h_fold[i] {
                    empty_v_rows += padding - 1;
                }
            }
            let mut empty_h_rows = 0;
            for i in 0..p.1 {
                if !v_fold[i] {
                    empty_h_rows += padding - 1;
                }
            }
            return (p.0 + empty_v_rows, p.1 + empty_h_rows);
        })
        .collect::<Vec<(usize, usize)>>();

    let mut sum = 0;
    for i in 0..new_points.len() - 1 {
        for j in i + 1..new_points.len() {
            sum += new_points[i].0.abs_diff(new_points[j].0)
                + new_points[i].1.abs_diff(new_points[j].1);
        }
    }

    return sum;
}

fn main() {
    let input: Vec<Vec<char>> = fs::read_to_string(INPUT)
        .unwrap()
        .split("\n")
        .filter_map(|l| {
            if l.is_empty() {
                return None;
            }
            return Some(l.chars().collect());
        })
        .collect();

    let mut h_fold: Vec<bool> = vec![false; input[0].len()];
    let mut v_fold: Vec<bool> = vec![false; input.len()];
    let mut points: Vec<(usize, usize)> = Vec::new();

    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if input[y][x] == '#' {
                h_fold[x] = true;
                v_fold[y] = true;
                points.push((x, y));
            }
        }
    }

    println!("Part 1: {}", solve(&h_fold, &v_fold, &points, 2));
    println!("Part 2: {}", solve(&h_fold, &v_fold, &points, 1000000));
}
