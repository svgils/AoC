use std::{collections::HashMap, fs};

const INPUT: &str = "./input";

#[derive(Hash, Eq, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn find_start(map: &Vec<Vec<char>>) -> (usize, usize) {
    for (y, row) in map.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if col == &'S' {
                return (x, y);
            }
        }
    }
    return (0, 0);
}

fn main() {
    let direction_map = HashMap::from([
        ('-', (Direction::East, Direction::West)),
        ('|', (Direction::North, Direction::South)),
        ('L', (Direction::North, Direction::East)),
        ('J', (Direction::North, Direction::West)),
        ('7', (Direction::West, Direction::South)),
        ('F', (Direction::East, Direction::South)),
    ]);

    let inverse_map = HashMap::from([
        (Direction::North, Direction::South),
        (Direction::East, Direction::West),
        (Direction::South, Direction::North),
        (Direction::West, Direction::East),
    ]);

    let next_direction = |direction: &Direction, pipe: &char| -> &Direction {
        return if inverse_map[direction] != direction_map[pipe].0 {
            &direction_map[pipe].0
        } else {
            &direction_map[pipe].1
        };
    };

    let offset_map: HashMap<Direction, (i64, i64)> = HashMap::from([
        (Direction::North, (0, -1)),
        (Direction::East, (1, 0)),
        (Direction::South, (0, 1)),
        (Direction::West, (-1, 0)),
    ]);

    let possible_map = HashMap::from([
        (Direction::North, vec!['|', '7', 'F']),
        (Direction::East, vec!['-', 'J', '7']),
        (Direction::South, vec!['|', 'J', 'L']),
        (Direction::West, vec!['-', 'L', 'F']),
    ]);

    let input: Vec<Vec<char>> = fs::read_to_string(INPUT)
        .unwrap()
        .split("\n")
        .filter_map(|line| {
            if !line.is_empty() {
                Some(line.chars().collect())
            } else {
                None
            }
        })
        .collect();

    let start = find_start(&input);

    let next_point = |direction: &Direction, pos: (usize, usize)| -> (usize, usize) {
        (
            (pos.0 as i64 + offset_map[direction].0) as usize,
            (pos.1 as i64 + offset_map[direction].1) as usize,
        )
    };
    let index_map = |pos: (usize, usize)| -> char { input[pos.1][pos.0] };

    let mut starts: Vec<&Direction> = vec![];
    if start.1 != 0
        && possible_map[&Direction::North]
            .contains(&index_map(next_point(&Direction::North, start)))
    {
        starts.push(&Direction::North);
    }
    if start.0 != input[0].len() - 1
        && possible_map[&Direction::East].contains(&index_map(next_point(&Direction::East, start)))
    {
        starts.push(&Direction::East);
    }
    if start.1 != input.len() - 1
        && possible_map[&Direction::South]
            .contains(&index_map(next_point(&Direction::South, start)))
    {
        starts.push(&Direction::South);
    }
    if start.0 != 0
        && possible_map[&Direction::West].contains(&index_map(next_point(&Direction::West, start)))
    {
        starts.push(&Direction::West);
    }

    let mut walkers = vec![(start, starts[0]), (start, starts[1])];
    // Find 2 possible start directions

    let mut steps = 0;
    loop {
        for walker in &mut walkers {
            let next = next_point(&walker.1, walker.0);
            walker.0 = next;
            walker.1 = next_direction(&walker.1, &index_map(next));
        }
        steps += 1;

        if walkers[0].0 == walkers[1].0 {
            break;
        }
    }

    println!("Part 1: {}", steps);
}
