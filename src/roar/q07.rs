use crate::utils::parser::FileLines;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

fn _rank_card(card: char, jokers_wild: bool) -> u32 {
    // What am I doing :D
    let card_ranks = HashMap::from([
        ('A', 13),
        ('K', 12),
        ('Q', 11),
        ('J', if jokers_wild { 1 } else { 10 }),
        ('T', if jokers_wild { 10 } else { 9 }),
        ('9', if jokers_wild { 9 } else { 8 }),
        ('8', if jokers_wild { 8 } else { 7 }),
        ('7', if jokers_wild { 7 } else { 6 }),
        ('6', if jokers_wild { 6 } else { 5 }),
        ('5', if jokers_wild { 5 } else { 4 }),
        ('4', if jokers_wild { 4 } else { 3 }),
        ('3', if jokers_wild { 3 } else { 2 }),
        ('2', if jokers_wild { 2 } else { 1 }),
    ]);

    *card_ranks.get(&card).unwrap()
}

fn _compare_hands_by_cards(hand1: &str, hand2: &str, jokers_wild: bool) -> Ordering {
    let hand1_cards: Vec<char> = hand1.chars().collect();
    let hand2_cards: Vec<char> = hand2.chars().collect();
    for (i, card) in hand1_cards.iter().enumerate() {
        match _rank_card(*card, jokers_wild).cmp(&_rank_card(hand2_cards[i], jokers_wild)) {
            Ordering::Greater => return Ordering::Greater,
            Ordering::Less => return Ordering::Less,
            _ => (),
        }
    }
    Ordering::Equal
}

fn _rank_hand(hand: &str, jokers_wild: bool) -> u32 {
    const FIVE_OF_A_KIND: u32 = 7;
    const FOUR_OF_A_KIND: u32 = 6;
    const FULL_HOUSE: u32 = 5;
    const THREE_OF_A_KIND: u32 = 4;
    const TWO_PAIR: u32 = 3;
    const ONE_PAIR: u32 = 2;
    const HIGH_CARD: u32 = 1;

    let mut counts = HashMap::new();
    let mut joker_count = 0;

    for c in hand.chars() {
        if c == 'J' && jokers_wild {
            joker_count += 1;
        } else {
            *counts.entry(c).or_insert(0) += 1;
        }
    }

    let mut values: Vec<_> = counts.values().cloned().collect();
    values.sort_unstable_by(|a, b| b.cmp(a));

    match joker_count {
        5 => FIVE_OF_A_KIND, // Five of a kind with all jokers
        _ => {
            // Use jokers to form the best hand
            let max_count = match values.first() {
                Some(&x) => x + joker_count,
                None => joker_count,
            };

            match (max_count, values.len()) {
                (5, _) => FIVE_OF_A_KIND,
                (4, _) => FOUR_OF_A_KIND,
                (3, 2) | (3, 1) => FULL_HOUSE,
                (3, _) => THREE_OF_A_KIND,
                (2, 3) | (2, 2) => TWO_PAIR,
                (2, _) => ONE_PAIR,
                _ => HIGH_CARD,
            }
        }
    }
}

#[derive(Eq, Debug, Clone)]
struct HandAndBid {
    jokers_wild: bool,
    data: (String, u32),
}

impl PartialEq for HandAndBid {
    fn eq(&self, other: &Self) -> bool {
        self.data.0 == other.data.0
    }
}

impl Ord for HandAndBid {
    fn cmp(&self, other: &Self) -> Ordering {
        match _rank_hand(&self.data.0, self.jokers_wild)
            .cmp(&_rank_hand(&other.data.0, self.jokers_wild))
        {
            Ordering::Equal => {
                _compare_hands_by_cards(&self.data.0, &other.data.0, self.jokers_wild)
            }
            other_ordering => other_ordering,
        }
    }
}

impl PartialOrd for HandAndBid {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Input {
    _hands_and_bids: Vec<HandAndBid>,
}

impl TryFrom<FileLines> for Input {
    type Error = std::io::Error;

    fn try_from(_lines: FileLines) -> Result<Self, Self::Error> {
        let mut hands_and_bids = Vec::new();
        for line in _lines {
            let hand_and_bid = line.split_once(' ').unwrap();
            let hand = hand_and_bid.0;
            let bid = hand_and_bid.1.parse::<u32>().unwrap();
            let hand_and_bid = HandAndBid {
                jokers_wild: false,
                data: (hand.to_string(), bid),
            };
            hands_and_bids.push(hand_and_bid);
        }

        let mut ordered_hands_and_bids = BinaryHeap::new();
        for hand_and_bid in hands_and_bids.iter() {
            ordered_hands_and_bids.push(hand_and_bid.clone());
        }
        Ok(Input {
            _hands_and_bids: hands_and_bids,
        })
    }
}

fn _get_total_winnings(ordered_hands_and_bids: BinaryHeap<HandAndBid>) -> u32 {
    let mut ordered_hands_and_bids = ordered_hands_and_bids;
    let mut rank = ordered_hands_and_bids.len();
    let mut total = 0;
    while let Some(hand_and_bid) = ordered_hands_and_bids.pop() {
        total += hand_and_bid.data.1 * rank as u32;
        rank -= 1;
    }
    total
}

fn _part_1(input_file: &str) -> std::io::Result<u32> {
    let input = Input::try_from(FileLines::new(input_file)?)?;
    let mut ordered_hands_and_bids = BinaryHeap::new();
    for hand_and_bid in input._hands_and_bids.iter() {
        ordered_hands_and_bids.push(hand_and_bid.clone());
    }
    Ok(_get_total_winnings(ordered_hands_and_bids))
}

fn _part_2(input_file: &str) -> std::io::Result<u32> {
    let input = Input::try_from(FileLines::new(input_file)?)?;
    let mut ordered_hands_and_bids = BinaryHeap::new();
    for hand_and_bid in input._hands_and_bids.iter() {
        let hand_and_bid_with_jokers_wild = HandAndBid {
            jokers_wild: true,
            data: hand_and_bid.data.clone(),
        };
        ordered_hands_and_bids.push(hand_and_bid_with_jokers_wild);
    }
    Ok(_get_total_winnings(ordered_hands_and_bids))
}

#[cfg(test)]
mod tests {
    use super::{_part_1, _part_2};

    const INPUT: &str = "input/roar/q07_input.txt";
    const INPUT_SAMPLE: &str = "input/roar/q07_sample.txt";

    #[test]
    fn roar_q07_p1_sample() {
        let result = _part_1(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 6440);
    }

    #[test]
    fn roar_q07_p1_main() {
        let result = _part_1(INPUT);
        assert_eq!(result.unwrap(), 248453531);
    }

    #[test]
    fn roar_q07_p2_sample() {
        let result = _part_2(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 5905);
    }

    #[test]
    fn roar_q07_p2_main() {
        let result = _part_2(INPUT);
        assert_eq!(result.unwrap(), 248781813);
    }
}
