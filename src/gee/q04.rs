use crate::utils::parser::FileLines;
use std::collections::HashSet;

#[derive(Debug)]
struct Input {
    _cards: Vec<Card>,
}

#[derive(Debug)]
struct Card {
    _winning: HashSet<u32>,
    _numbers: Vec<u32>,
}

impl TryFrom<FileLines> for Input {
    type Error = std::io::Error;

    fn try_from(_lines: FileLines) -> Result<Self, Self::Error> {
        let _cards = _lines
            .map(|line| {
                let (win, have) = line.split_once(": ").unwrap().1.split_once(" | ").unwrap();
                Card {
                    _winning: HashSet::from_iter(
                        win.split_whitespace().map(|s| s.parse::<u32>().unwrap()),
                    ),
                    _numbers: have
                        .split_whitespace()
                        .map(|s| s.parse::<u32>().unwrap())
                        .collect(),
                }
            })
            .collect();
        Ok(Input { _cards })
    }
}

impl Input {
    fn _total_scratchcards(&self) -> usize {
        let won_cards: usize = self
            ._cards
            .iter()
            .enumerate()
            .map(|(i, _)| self._find_won_scratchcards(i))
            .sum();
        self._cards.len() + won_cards
    }

    fn _find_won_scratchcards(&self, index: usize) -> usize {
        let winners = self._cards[index]._winners();
        if winners == 0 {
            0
        } else {
            let others: usize = ((index + 1)..=(index + winners))
                .map(|i| self._find_won_scratchcards(i))
                .sum();
            winners + others
        }
    }
}

impl Card {
    fn _score(&self) -> u32 {
        let winners = self._winners();
        if winners == 0 {
            0
        } else {
            2u32.pow(u32::try_from(winners).unwrap() - 1)
        }
    }

    fn _winners(&self) -> usize {
        self._numbers
            .iter()
            .filter(|n| self._winning.contains(n))
            .count()
    }
}

fn _part_1(input_file: &str) -> std::io::Result<u32> {
    let input = Input::try_from(FileLines::new(input_file)?)?;
    Ok(input._cards.iter().map(|c| c._score()).sum())
}

fn _part_2(input_file: &str) -> std::io::Result<usize> {
    let input = Input::try_from(FileLines::new(input_file)?)?;
    Ok(input._total_scratchcards())
}

#[cfg(test)]
mod tests {
    use super::{_part_1, _part_2};

    const INPUT: &str = "input/gee/q04_input.txt";
    const INPUT_SAMPLE: &str = "input/gee/q04_sample.txt";

    #[test]
    fn gee_q04_p1_sample() {
        let result = _part_1(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 13);
    }

    #[test]
    fn gee_q04_p1_main() {
        let result = _part_1(INPUT);
        assert_eq!(result.unwrap(), 23028);
    }

    #[test]
    fn gee_q04_p2_sample() {
        let result = _part_2(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 30);
    }

    #[ignore]
    #[test]
    fn gee_q04_p2_main() {
        let result = _part_2(INPUT);
        assert_eq!(result.unwrap(), 9236992);
    }
}
