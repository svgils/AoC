use std::{collections::HashMap, fs};

const INPUT: &str = "./input";

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOAKind,
    FourOAKind,
    FullHouse,
    ThreeOAKind,
    TwoPair,
    OnePair,
    HighCard,
}

struct Hand {
    cards: Vec<i8>,
    hand_type: HandType,
    stake: i64,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        if self.hand_type != other.hand_type {
            return false;
        };
        if self.cards != other.cards {
            return false;
        };
        return true;
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.hand_type > other.hand_type {
            return Some(std::cmp::Ordering::Greater);
        };
        if self.hand_type < other.hand_type {
            return Some(std::cmp::Ordering::Less);
        };

        for (x, y) in self.cards.iter().zip(other.cards.iter()) {
            if x < y {
                return Some(std::cmp::Ordering::Greater);
            }
            if x > y {
                return Some(std::cmp::Ordering::Less);
            }
        }
        return Some(std::cmp::Ordering::Equal);
    }
}

impl Eq for Hand {}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn main() {
    let mut hands_part1: Vec<Hand> = fs::read_to_string(INPUT)
        .unwrap()
        .split("\n")
        .filter_map(|l| {
            if l.is_empty() {
                return None;
            }
            let mut parts = l.split(" ");
            let cards = parts.next().unwrap();
            let stake: i64 = parts.last().unwrap().parse().unwrap();
            let mut card_map: Vec<i32> = cards
                .chars()
                .fold(HashMap::new(), |mut acc, c| {
                    *acc.entry(c).or_insert(0) += 1;
                    return acc;
                })
                .into_values()
                .collect::<Vec<i32>>();
            card_map.sort_unstable();
            card_map.reverse();
            let hand_type = if card_map[0] == 5 {
                HandType::FiveOAKind
            } else if card_map[0] == 4 {
                HandType::FourOAKind
            } else if card_map[0] == 3 && card_map[1] == 2 {
                HandType::FullHouse
            } else if card_map[0] == 3 {
                HandType::ThreeOAKind
            } else if card_map[0] == 2 && card_map[1] == 2 {
                HandType::TwoPair
            } else if card_map[0] == 2 {
                HandType::OnePair
            } else {
                HandType::HighCard
            };
            return Some(Hand {
                cards: cards
                    .chars()
                    .map(|c| match c {
                        '1' => 1,
                        '2' => 2,
                        '3' => 3,
                        '4' => 4,
                        '5' => 5,
                        '6' => 6,
                        '7' => 7,
                        '8' => 8,
                        '9' => 9,
                        'J' => 10,
                        'T' => 11,
                        'Q' => 12,
                        'K' => 13,
                        'A' => 14,
                        _ => -1,
                    })
                    .collect(),
                hand_type,
                stake,
            });
        })
        .collect();

    let mut hands_part2: Vec<Hand> = fs::read_to_string(INPUT)
        .unwrap()
        .split("\n")
        .filter_map(|l| {
            if l.is_empty() {
                return None;
            }
            let mut parts = l.split(" ");
            let cards = parts.next().unwrap();
            let stake: i64 = parts.last().unwrap().parse().unwrap();
            let mut num_j = 0;
            let card_map: HashMap<char, i32> = cards.chars().fold(HashMap::new(), |mut acc, c| {
                if c == 'J' {
                    num_j += 1;
                    // *acc.entry(c).or_insert(0) += 1;
                } else {
                    *acc.entry(c).or_insert(0) += 1;
                }
                return acc;
            });
            // println!("{:?}", card_map);
            let mut card_nums = card_map.into_values().collect::<Vec<i32>>();
            card_nums.sort_unstable();
            card_nums.reverse();
            let hand_type = if num_j == 5 {
                HandType::FiveOAKind
            } else if card_nums[0] + num_j == 5 {
                HandType::FiveOAKind
            } else if card_nums[0] + num_j == 4 {
                HandType::FourOAKind
            } else if card_nums[0] + num_j == 3 && card_nums[1] == 2 {
                HandType::FullHouse
            } else if card_nums[0] + num_j == 3 {
                HandType::ThreeOAKind
            } else if card_nums[0] + num_j == 2 && card_nums[1] == 2 {
                HandType::TwoPair
            } else if card_nums[0] + num_j == 2 {
                HandType::OnePair
            } else {
                HandType::HighCard
            };
            return Some(Hand {
                cards: cards
                    .chars()
                    .map(|c| match c {
                        'J' => 1,
                        '1' => 2,
                        '2' => 3,
                        '3' => 4,
                        '4' => 5,
                        '5' => 6,
                        '6' => 7,
                        '7' => 8,
                        '8' => 9,
                        '9' => 10,
                        'T' => 11,
                        'Q' => 12,
                        'K' => 13,
                        'A' => 14,
                        _ => -1,
                    })
                    .collect(),
                hand_type,
                stake,
            });
        })
        .collect();

    // for hand in &mut hands_part2 {
    //     println!("{:?} {:?}", hand.cards, hand.hand_type);
    // }

    hands_part1.sort_unstable();
    hands_part1.reverse();
    let mut sum = 0;
    for (idx, hand) in hands_part1.iter().enumerate() {
        sum += (idx + 1) as i64 * hand.stake;
    }

    hands_part2.sort_unstable();
    hands_part2.reverse();
    let mut sum2 = 0;
    for (idx, hand) in hands_part2.iter().enumerate() {
        sum2 += (idx + 1) as i64 * hand.stake;
    }

    println!("Part 1: {}", sum);
    println!("Part 2: {}", sum2);
}
