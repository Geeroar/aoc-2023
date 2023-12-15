#![allow(dead_code)]

use crate::utils::parser::FileLines;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Input {
    cards: Vec<Card>,
}

#[derive(Debug)]
struct Card {
    winning: HashSet<u32>,
    numbers: Vec<u32>,
}

impl TryFrom<FileLines> for Input {
    type Error = std::io::Error;

    fn try_from(lines: FileLines) -> Result<Self, Self::Error> {
        let cards = lines
            .map(|line| {
                let (win, have) = line.split_once(": ").unwrap().1.split_once(" | ").unwrap();
                Card {
                    winning: HashSet::from_iter(
                        win.split_whitespace().map(|s| s.parse::<u32>().unwrap()),
                    ),
                    numbers: have
                        .split_whitespace()
                        .map(|s| s.parse::<u32>().unwrap())
                        .collect(),
                }
            })
            .collect();
        Ok(Input { cards })
    }
}

impl Input {
    fn total_scratchcards(&self) -> usize {
        let mut cards = HashMap::<usize, usize>::new();
        for (i, c) in self.cards.iter().enumerate() {
            let card_count = 1 + *cards.get(&i).unwrap_or(&0);
            let winners = c.winners();
            (1..=winners).for_each(|n| {
                cards
                    .entry(i + n)
                    .and_modify(|v| *v += card_count)
                    .or_insert(card_count);
            });
        }
        cards.values().sum::<usize>() + self.cards.len()
    }

    fn find_won_scratchcards(&self, index: usize) -> usize {
        let winners = self.cards[index].winners();
        if winners == 0 {
            0
        } else {
            let others: usize = ((index + 1)..=(index + winners))
                .map(|i| self.find_won_scratchcards(i))
                .sum();
            winners + others
        }
    }
}

impl Card {
    fn score(&self) -> u32 {
        let winners = self.winners();
        if winners == 0 {
            0
        } else {
            2u32.pow(u32::try_from(winners).unwrap() - 1)
        }
    }

    fn winners(&self) -> usize {
        self.numbers
            .iter()
            .filter(|n| self.winning.contains(n))
            .count()
    }
}

fn part_1(input_file: &str) -> std::io::Result<u32> {
    let input = Input::try_from(FileLines::new(input_file)?)?;
    Ok(input.cards.iter().map(|c| c.score()).sum())
}

fn part_2(input_file: &str) -> std::io::Result<usize> {
    let input = Input::try_from(FileLines::new(input_file)?)?;
    Ok(input.total_scratchcards())
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    const INPUT: &str = "input/gee/q04_input.txt";
    const INPUT_SAMPLE: &str = "input/gee/q04_sample.txt";

    #[test]
    fn gee_q04_p1_sample() {
        let result = part_1(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 13);
    }

    #[test]
    fn gee_q04_p1_main() {
        let result = part_1(INPUT);
        assert_eq!(result.unwrap(), 23028);
    }

    #[test]
    fn gee_q04_p2_sample() {
        let result = part_2(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 30);
    }

    #[test]
    fn gee_q04_p2_main() {
        let result = part_2(INPUT);
        assert_eq!(result.unwrap(), 9236992);
    }
}
