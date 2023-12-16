use std::{
    collections::{HashMap, HashSet},
    fs,
    ops::{Add, AddAssign},
};

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct Pos {
    x: i64,
    y: i64,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Beam {
    pos: Pos,
    dir: u8, // 0: N, 1: E, 2: S, 3: W
}

impl Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Pos { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

fn main() {
    let map: Vec<Vec<char>> = fs::read_to_string("./input")
        .unwrap()
        .split("\n")
        .filter_map(|l| {
            if l.is_empty() {
                return None;
            }
            Some(l.chars().collect())
        })
        .collect();

    let get_offset = |dir| match dir {
        0 => Pos { x: 0, y: -1 },
        1 => Pos { x: 1, y: 0 },
        2 => Pos { x: 0, y: 1 },
        3 => Pos { x: -1, y: 0 },
        _ => Pos { x: 0, y: 0 },
    };

    let width = map[0].len() as i64 - 1;
    let height = map.len() as i64 - 1;

    let mut starts: Vec<Beam> = vec![
        // Top left
        Beam { pos: Pos { x: 0, y: 0 }, dir: 1 },
        Beam { pos: Pos { x: 0, y: 0 }, dir: 2 },
        // Top right
        Beam { pos: Pos { x: width, y: 0 }, dir: 3 },
        Beam { pos: Pos { x: width, y: 0 }, dir: 2 },
        // Bottom right
        Beam { pos: Pos { x: width, y: height }, dir: 0 },
        Beam { pos: Pos { x: width, y: height }, dir: 3 },
        // Bottom left
        Beam { pos: Pos { x: 0, y: height }, dir: 0 },
        Beam { pos: Pos { x: 0, y: height }, dir: 1 },
    ];

    for i in 1..width {
        starts.push(Beam { pos: Pos { x: i, y: 0 }, dir: 2 });
        starts.push(Beam { pos: Pos { x: i, y: height }, dir: 0 });
    }
    for i in 1..height {
        starts.push(Beam { pos: Pos { x: 0, y: i }, dir: 1 });
        starts.push(Beam { pos: Pos { x: width, y: i }, dir: 3 });
    }

    let results: Vec<usize> = starts
        .iter()
        .map(|start| {
            let mut hit: HashSet<Pos> = HashSet::new();
            let mut beams = vec![start.clone()];

            let mut beam_cache: HashSet<Beam> = HashSet::new();

            while beams.len() > 0 {
                beams = beams
                    .iter()
                    .filter_map(|beam| {
                        let mut beam = beam.clone();
                        if beam.pos.x > width
                            || beam.pos.x < 0
                            || beam.pos.y > height
                            || beam.pos.y < 0
                            || beam_cache.contains(&beam)
                        {
                            // If beam goes out of bounds, delete it.
                            return None;
                        } else {
                            hit.insert(beam.pos);
                            beam_cache.insert(beam);

                            let mut new_beams: Vec<Beam> = Vec::new();
                            let tile = map[beam.pos.y as usize][beam.pos.x as usize];

                            if tile == '/' {
                                beam.dir = (beam.dir + 1 + (beam.dir % 2) * 2) % 4;
                            } else if tile == '\\' {
                                beam.dir = (beam.dir + 1 + ((beam.dir % 2) + 1 % 2) * 2) % 4;
                            } else if tile == '|' && beam.dir % 2 == 1 {
                                new_beams.append(&mut vec![
                                    Beam { pos: beam.pos + get_offset(0), dir: 0 },
                                    Beam { pos: beam.pos + get_offset(2), dir: 2 },
                                ]);
                                return Some(new_beams);
                            } else if tile == '-' && beam.dir % 2 == 0 {
                                new_beams.append(&mut vec![
                                    Beam { pos: beam.pos + get_offset(1), dir: 1 },
                                    Beam { pos: beam.pos + get_offset(3), dir: 3 },
                                ]);
                                return Some(new_beams);
                            }

                            beam.pos += get_offset(beam.dir);
                            new_beams.push(beam);

                            return Some(new_beams);
                        }
                    })
                    .flatten()
                    .collect();
            }
            return hit.len();
        })
        .collect();

    println!("Part 1: {}", results[0]);
    println!("Part 2: {}", results.iter().max().unwrap());
}
