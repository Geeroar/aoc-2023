use crate::utils::parser::FileLines;
use std::collections::HashMap;
use std::collections::HashSet;

struct Input {
    _value: u32,
}

impl TryFrom<FileLines> for Input {
    type Error = std::io::Error;

    fn try_from(_lines: FileLines) -> Result<Self, Self::Error> {
        Ok(Input { _value: 0 })
    }
}

fn _part_1(input_file: &str) -> std::io::Result<u32> {
    let lines = FileLines::new(input_file)?;
    let mut result: u32 = 0;
    for line in lines {
        let parts: Vec<&str> = line.split(':').collect();
        let numbers_parts: Vec<&str> = parts[1].split('|').collect();

        let winning_numbers: Vec<i32> = numbers_parts[0]
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let our_numbers: Vec<i32> = numbers_parts[1]
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let mut winning_numbers_set: HashSet<i32> = HashSet::new();
        for num in &winning_numbers {
            winning_numbers_set.insert(*num);
        }
        let mut score = 0;
        for num in &our_numbers {
            if winning_numbers_set.contains(num) {
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }
            }
        }
        result += score;
    }
    Ok(result)
}

fn _part_2(input_file: &str) -> std::io::Result<u32> {
    let lines = FileLines::new(input_file)?;
    let mut card_counts: HashMap<i32, i32> = HashMap::new();
    for line in lines {
        let parts: Vec<&str> = line.split(':').collect();
        let card_num: i32 = parts[0].split_whitespace().last().unwrap().parse().unwrap();
        *card_counts.entry(card_num).or_insert(0) += 1;
        let numbers_parts: Vec<&str> = parts[1].split('|').collect();

        let winning_numbers: Vec<i32> = numbers_parts[0]
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let our_numbers: Vec<i32> = numbers_parts[1]
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let mut winning_numbers_set: HashSet<i32> = HashSet::new();
        for num in &winning_numbers {
            winning_numbers_set.insert(*num);
        }
        let mut matches = 0;
        for num in &our_numbers {
            if winning_numbers_set.contains(num) {
                matches += 1;
            }
        }
        for _ in 0..*card_counts.entry(card_num).or_insert(1) {
            for i in 1..matches + 1 {
                *card_counts.entry(card_num + i).or_insert(0) += 1;
            }
        }
    }
    let total_count: i32 = card_counts.values().sum();
    let total_count_u32: u32 = total_count as u32;
    Ok(total_count_u32)
}

#[cfg(test)]
mod tests {
    use super::{_part_1, _part_2};

    const INPUT: &str = "input/roar/q04_input.txt";
    const INPUT_SAMPLE: &str = "input/roar/q04_sample.txt";

    #[test]
    fn roar_q04_p1_sample() {
        let result = _part_1(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 13);
    }

    #[test]
    fn roar_q04_p1_main() {
        let result = _part_1(INPUT);
        assert_eq!(result.unwrap(), 32001);
    }

    #[test]
    fn roar_q04_p2_sample() {
        let result = _part_2(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 30);
    }

    #[test]
    fn roar_q04_p2_main() {
        let result = _part_2(INPUT);
        assert_eq!(result.unwrap(), 5037841);
    }
}
