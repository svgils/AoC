#[macro_use]
extern crate lazy_static;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

const INPUT: &str = "./input";

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn from_u8(value: u8) -> Direction {
        match value {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            3 => Direction::West,
            _ => Direction::North,
        }
    }
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

fn next_direction(
    direction: Direction,
    pipe: &char,
    map: &mut HashMap<char, (Direction, Direction)>,
) -> Direction {
    lazy_static! {
        static ref INVERSE_MAP: HashMap<Direction, Direction> = HashMap::from([
            (Direction::North, Direction::South),
            (Direction::East, Direction::West),
            (Direction::South, Direction::North),
            (Direction::West, Direction::East),
        ]);
    }

    return if INVERSE_MAP[&direction] != map[pipe].0 {
        map[pipe].0
    } else {
        map[pipe].1
    };
}

fn main() {
    // Map pipe sections input and output heading and rotational value.
    let mut direction_map = HashMap::from([
        ('-', (Direction::East, Direction::West)),
        ('|', (Direction::North, Direction::South)),
        ('L', (Direction::North, Direction::East)),
        ('J', (Direction::North, Direction::West)),
        ('7', (Direction::West, Direction::South)),
        ('F', (Direction::East, Direction::South)),
    ]);

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

    direction_map.entry('S').or_insert((*starts[0], *starts[1]));

    struct Walker {
        pos: (usize, usize),
        dir: Direction,
        rotation: i16,
        inside_tiles: HashSet<(usize, usize)>,
    }

    let mut walkers = vec![
        Walker {
            pos: start,
            dir: *starts[0],
            rotation: 0,
            inside_tiles: HashSet::new(),
        },
        Walker {
            pos: start,
            dir: *starts[1],
            rotation: 0,
            inside_tiles: HashSet::new(),
        },
    ];

    let mut visited_tiles: HashSet<(usize, usize)> = HashSet::new();
    let mut steps = 0;
    loop {
        for walker in &mut walkers {
            let old_dir = walker.dir;
            let next = next_point(&walker.dir, walker.pos);
            walker.pos = next;
            walker.dir = next_direction(walker.dir, &index_map(next), &mut direction_map);
            walker.rotation += (walker.dir as i16 * 90 - old_dir as i16 * 90) % 180;
            // Add tile to right to inside tiles list
            let right1 = next_point(&Direction::from_u8((walker.dir as u8 + 1) % 4), walker.pos);
            let right2 = next_point(&Direction::from_u8((old_dir as u8 + 1) % 4), walker.pos);
            if right1.0 < (input[0].len() - 1) && right1.1 < (input.len() - 1) {
                walker.inside_tiles.insert(right1);
            }
            if right2.0 < (input[0].len() - 1) && right2.1 < (input.len() - 1) {
                walker.inside_tiles.insert(right2);
            }
            visited_tiles.insert(walker.pos);
        }
        steps += 1;
        if walkers[0].pos == start {
            break;
        }

        if walkers[0].pos == walkers[1].pos {
            println!("Part 1: {}", steps);
        }
    }

    let clockwise_walker = if walkers[0].rotation > 0 {
        &mut walkers[0]
    } else {
        &mut walkers[1]
    };

    let mut inside_tiles = clockwise_walker.inside_tiles.clone();
    loop {
        let mut missing: HashSet<(usize, usize)> = HashSet::new();
        for tile in inside_tiles {
            let north = next_point(&Direction::North, tile);
            let east = next_point(&Direction::East, tile);
            let south = next_point(&Direction::South, tile);
            let west = next_point(&Direction::West, tile);

            if !visited_tiles.contains(&north) && clockwise_walker.inside_tiles.contains(&north) {
                missing.insert(north);
            }
            if !visited_tiles.contains(&east) && clockwise_walker.inside_tiles.contains(&east) {
                missing.insert(north);
            }
            if !visited_tiles.contains(&south) && clockwise_walker.inside_tiles.contains(&south) {
                missing.insert(north);
            }
            if !visited_tiles.contains(&west) && clockwise_walker.inside_tiles.contains(&west) {
                missing.insert(north);
            }
        }

        if missing.is_empty() {
            break;
        } else {
            clockwise_walker.inside_tiles.extend(&missing);
            inside_tiles = missing.clone();
        }
    }

    let unique: HashSet<&(usize, usize)> = clockwise_walker
        .inside_tiles
        .difference(&visited_tiles)
        .collect();

    println!("Part 2: {}", unique.len());
}
