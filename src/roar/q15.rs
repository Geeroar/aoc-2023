#![allow(dead_code, unused_variables)]

use crate::utils::parser::{parse, FileLines};

struct Input {
    init_sequence: Vec<Vec<char>>,
}

impl TryFrom<FileLines> for Input {
    type Error = std::io::Error;

    fn try_from(lines: FileLines) -> Result<Self, Self::Error> {
        let mut init_sequence = Vec::new();
        for line in lines {
            let sections = line.split(',');
            for section in sections {
                let chars: Vec<char> = section.chars().collect();
                init_sequence.push(chars);
            }
        }
        Ok(Input { init_sequence })
    }
}

fn part_1(input_file: &str) -> std::io::Result<u64> {
    let input: Input = parse(input_file)?;
    let mut all_values: Vec<u64> = vec![];
    for sequence in input.init_sequence {
        let mut current_value: u64 = 0;
        for character in sequence {
            let ascii_code = character as u8;
            current_value += ascii_code as u64;
            current_value *= 17;
            current_value %= 256;
            // println!("{}:{}:{}", character, ascii_code, current_value);
        }
        all_values.push(current_value);
    }
    Ok(all_values.iter().sum::<u64>())
}

fn part_2(input_file: &str) -> std::io::Result<usize> {
    let input: Input = parse(input_file)?;
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    const INPUT: &str = "input/roar/q15_input.txt";
    const INPUT_SAMPLE: &str = "input/roar/q15_sample.txt";

    #[test]
    fn roar_q15_p1_sample() {
        let result = part_1(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 1320);
    }

    #[test]
    fn roar_q15_p1_main() {
        let result = part_1(INPUT);
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn roar_q15_p2_sample() {
        let result = part_2(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn roar_q15_p2_main() {
        let result = part_2(INPUT);
        assert_eq!(result.unwrap(), 0);
    }
}
