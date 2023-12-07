use crate::utils::parser::FileLines;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

struct Input {
    _hands: Vec<Hand>,
}

#[derive(Debug)]
struct Hand {
    _cards: Vec<u32>,
    _bid: u32,
}

impl TryFrom<(FileLines, bool)> for Input {
    type Error = std::io::Error;

    fn try_from(_in: (FileLines, bool)) -> Result<Self, Self::Error> {
        let (lines, use_jokers) = _in;
        let _hands = lines
            .map(|line| {
                let (hand, bid) = line.split_once(' ').unwrap();
                let _cards = hand
                    .chars()
                    .map(|c| match c {
                        'A' => 14,
                        'K' => 13,
                        'Q' => 12,
                        'J' if use_jokers => 1,
                        'J' if !use_jokers => 11,
                        'T' => 10,
                        '9' => 9,
                        '8' => 8,
                        '7' => 7,
                        '6' => 6,
                        '5' => 5,
                        '4' => 4,
                        '3' => 3,
                        '2' => 2,
                        _ => 0,
                    })
                    .collect();
                Hand {
                    _cards,
                    _bid: bid.parse().unwrap(),
                }
            })
            .collect();
        Ok(Input { _hands })
    }
}

impl Hand {
    fn _score(&self) -> u32 {
        let mut map: HashMap<u32, u32> = HashMap::new();
        for &card in self._cards.iter() {
            let count = map.get(&card).unwrap_or(&0);
            map.insert(card, count + 1);
        }
        let joker_count = map.get(&1).unwrap_or(&0);
        let n = map.keys().count();
        let values: HashSet<u32> = map.values().copied().collect();
        match (n, joker_count) {
            (1, _) => 7,                        // 5 of a kind
            (2, _) if *joker_count > 0 => 7,    // 5 of a kind
            (2, 0) if values.contains(&4) => 6, // 4 of a kind
            (3, 3) | (3, 2) => 6,               // 4 of a kind
            (3, 1) if values.contains(&3) => 6, // 4 of a kind
            (3, 1) | (2, 0) => 5,               // Full house
            (3, 0) if values.contains(&3) => 4, // 3 of a kind
            (4, 2) | (4, 1) => 4,               // 3 of a kind
            (3, 0) => 3,                        // 2 pair
            (4, 0) | (5, 1) => 2,               // 1 pair
            (5, 0) => 1,                        // Highest card
            _ => 0,
        }
    }
}

fn _compare_hands(a: &Hand, b: &Hand) -> Ordering {
    match (a._score(), b._score()) {
        (x, y) if x < y => Ordering::Less,
        (x, y) if x > y => Ordering::Greater,
        _ => {
            for i in 0..a._cards.len() {
                match (a._cards[i], b._cards[i]) {
                    (x, y) if x < y => return Ordering::Less,
                    (x, y) if x > y => return Ordering::Greater,
                    _ => (),
                }
            }
            Ordering::Equal
        }
    }
}

fn _part_1(input_file: &str) -> std::io::Result<u32> {
    let mut input = Input::try_from((FileLines::new(input_file)?, false))?;
    input._hands.sort_by(_compare_hands);
    Ok(input
        ._hands
        .iter()
        .enumerate()
        .map(|(i, h)| h._bid * (u32::try_from(i).unwrap() + 1))
        .sum())
}

fn _part_2(input_file: &str) -> std::io::Result<u32> {
    let mut input = Input::try_from((FileLines::new(input_file)?, true))?;
    input._hands.sort_by(_compare_hands);
    Ok(input
        ._hands
        .iter()
        .enumerate()
        .map(|(i, h)| h._bid * (u32::try_from(i).unwrap() + 1))
        .sum())
}

#[cfg(test)]
mod tests {
    use super::{_part_1, _part_2};

    const INPUT: &str = "input/gee/q07_input.txt";
    const INPUT_SAMPLE: &str = "input/gee/q07_sample.txt";

    #[test]
    fn gee_q07_p1_sample() {
        let result = _part_1(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 6440);
    }

    #[test]
    fn gee_q07_p1_main() {
        let result = _part_1(INPUT);
        assert_eq!(result.unwrap(), 253638586);
    }

    #[test]
    fn gee_q07_p2_sample() {
        let result = _part_2(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 5905);
    }

    #[test]
    fn gee_q07_p2_main() {
        let result = _part_2(INPUT);
        assert_eq!(result.unwrap(), 253253225);
    }
}
