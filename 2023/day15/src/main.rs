use std::{collections::HashMap, fs};

fn main() {
    let hash_func = |s: &str| s.chars().fold(0, |acc, c| ((acc + c as u16) * 17) % 256) as u8;
    let mut boxes: HashMap<u8, Vec<(&str, u8)>> = HashMap::new();
    let input = fs::read_to_string("./input").unwrap();
    let res = input.trim_end().split(",").fold(0usize, |acc, section| {
        let mut parts: Vec<&str> = Vec::new();
        let mut last = 0;
        for (index, matched) in section.match_indices(|c| c == '-' || c == '=') {
            if last != index {
                parts.push(&section[last..index]);
            }
            parts.push(matched);
            last = index + matched.len();
        }
        if last < section.len() {
            parts.push(&section[last..]);
        }

        if parts[1] == "=" {
            let _box = boxes.entry(hash_func(parts[0])).or_insert(Vec::new());
            if let Some(entry) = _box.iter_mut().find(|e| e.0 == parts[0]) {
                entry.1 = parts[2].parse::<u8>().unwrap();
            } else {
                _box.push((parts[0], parts[2].parse::<u8>().unwrap()));
            }
        } else {
            let key = hash_func(parts[0]);
            if boxes.contains_key(&key) {
                if let Some(p) = boxes[&key].iter().position(|e| e.0 == parts[0]) {
                    boxes.get_mut(&key).unwrap().remove(p);
                }
            }
        }

        acc + hash_func(section) as usize
    });

    let res2 = boxes.iter_mut().fold(0, |acc, (i, _box)| {
        acc + _box.iter().enumerate().fold(0, |acc2, (j, lens)| {
            acc2 + (1 + *i as usize) * (j as usize + 1) * lens.1 as usize
        })
    });

    println!("Part 1: {}", res);
    println!("Part 2: {}", res2);
}
