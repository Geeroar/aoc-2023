use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use crate::utils::parser::FileLines;

fn _rank_card(card: char) -> u32 {
    // What am I doing :D
    let card_ranks = HashMap::from([
        ('A', 13),
        ('K', 12),
        ('Q', 11),
        ('J', 10),
        ('T', 9),
        ('9', 8),
        ('8', 7),
        ('7', 6),
        ('6', 5),
        ('5', 4),
        ('4', 3),
        ('3', 2),
        ('2', 1),
    ]);

    return card_ranks.get(&card).unwrap().clone();
}

fn _compare_hands_by_cards(hand1: &String, hand2: &String) -> Ordering {
    let hand1_cards: Vec<char> = hand1.chars().collect();
    let hand2_cards: Vec<char> = hand2.chars().collect();
    for (i, card) in hand1_cards.iter().enumerate() {
        if _rank_card(*card) > _rank_card(hand2_cards[i]) {
            return Ordering::Greater;
        } else if _rank_card(*card) < _rank_card(hand2_cards[i]) {
            return Ordering::Less;
        }
    }
    return Ordering::Equal;
}

fn _rank_hand(hand: &String) -> u32 {
    let hand_ranks = HashMap::from([
        ("FiveOfAKind", 7),
        ("FourOfAKind", 6),
        ("FullHouse", 5),
        ("ThreeOfAKind", 4),
        ("TwoPair", 3),
        ("OnePair", 2),
        ("HighCard", 1),
    ]);
    // Keep track of counts
    let mut counts = Vec::new();
    let mut current_count = 0;
    // Sorting the string to make it easier to check
    let mut chars: Vec<char> = hand.chars().collect();
    chars.sort();
    let mut current_char =  chars[0];
    for( i, c) in chars.iter().enumerate() {
        if *c == current_char {
            current_count += 1;
        } else {
            counts.push(current_count);
            current_char = *c;
            current_count = 1;
        }
        if i == chars.len() - 1 {
            counts.push(current_count);
        }
    }
    counts.sort_by(|a, b| b.cmp(a));

    // Check for FiveOfAKind
    if counts[0] == 5 {
        return hand_ranks.get("FiveOfAKind").unwrap().clone();
    }
    // Check for FourOfAKind
    if counts[0] == 4 {
        return hand_ranks.get("FourOfAKind").unwrap().clone();
    }
    // Check for FullHouse
    if counts[0] == 3 && counts[1] == 2 {
        return hand_ranks.get("FullHouse").unwrap().clone();
    }
    // Check for ThreeOfAKind
    if counts[0] == 3 {
        return hand_ranks.get("ThreeOfAKind").unwrap().clone();
    }
    // Check for TwoPair
    if counts[0] == 2 && counts[1] == 2 {
        return hand_ranks.get("TwoPair").unwrap().clone();
    }
    // Check for OnePair
    if counts[0] == 2 {
        return hand_ranks.get("OnePair").unwrap().clone();
    }
    // HighCard
    return hand_ranks.get("HighCard").unwrap().clone();
}


#[derive(Eq)]
#[derive(Debug)]
#[derive(Clone)]
struct HandAndBid {
    data: (String, u32),
}

impl PartialEq for HandAndBid {
    fn eq(&self, other: &Self) -> bool {
        self.data.0 == other.data.0
    }
}

impl Ord for HandAndBid {
    fn cmp(&self, other: &Self) -> Ordering {
        match _rank_hand(&self.data.0).cmp(&_rank_hand(&other.data.0)) {
            Ordering::Equal => _compare_hands_by_cards(&self.data.0, &other.data.0),
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
    _ordered_hands_and_bids: BinaryHeap<HandAndBid>,
}

impl TryFrom<FileLines> for Input {
    type Error = std::io::Error;

    fn try_from(_lines: FileLines) -> Result<Self, Self::Error> {
        let mut hands_and_bids = Vec::new();
        for line in _lines {
            let hand_and_bid = line.split_once(" ").unwrap();
            let hand = hand_and_bid.0;
            let bid = hand_and_bid.1.parse::<u32>().unwrap();
            let hand_and_bid = HandAndBid {
                data: (hand.to_string(), bid),
            };
            hands_and_bids.push(hand_and_bid);
        }

        let mut ordered_hands_and_bids = BinaryHeap::new();
        for hand_and_bid in hands_and_bids.iter() {
            ordered_hands_and_bids.push(hand_and_bid.clone());
        }
        Ok(Input { _hands_and_bids: hands_and_bids, _ordered_hands_and_bids: ordered_hands_and_bids })
    }
}

fn _get_total_winnings(ordered_hands_and_bids: BinaryHeap<HandAndBid>) -> u32 {
    let mut ordered_hands_and_bids = ordered_hands_and_bids.clone();
    let mut rank = ordered_hands_and_bids.len();
    let mut total = 0;
    while let Some(hand_and_bid) = ordered_hands_and_bids.pop() {
        println!("{:?}", hand_and_bid);
        total += hand_and_bid.data.1 * rank as u32;
        rank -= 1;
    }
    return total;
}

fn _part_1(input_file: &str) -> std::io::Result<u32> {
    let input = Input::try_from(FileLines::new(input_file)?)?;
    Ok(_get_total_winnings(input._ordered_hands_and_bids))
}

fn _part_2(input_file: &str) -> std::io::Result<u32> {
    let input = Input::try_from(FileLines::new(input_file)?)?;
    Ok(_get_total_winnings(input._ordered_hands_and_bids))
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
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn roar_q07_p2_sample() {
        let result = _part_2(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn roar_q07_p2_main() {
        let result = _part_2(INPUT);
        assert_eq!(result.unwrap(), 0);
    }
}
