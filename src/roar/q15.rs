#![allow(dead_code, unused_variables)]

use crate::utils::parser::{parse, FileLines};

#[derive(Debug, PartialEq)]
enum Operation {
    Assign,
    Remove,
}

#[derive(Debug, PartialEq)]
struct Instruction {
    label: String,
    focal_length: u32,
    operation: Operation,
    original_string: String,
}

struct Input {
    init_sequence: Vec<Instruction>,
    boxes: Vec<Vec<Instruction>>,
}

fn parse_instruction(original_string: &str) -> Instruction {
    let mut label = String::new();
    let mut focal_length = 0;
    let mut operation: Operation = Operation::Assign;

    for character in original_string.chars() {
        match character {
            '0'..='9' => {
                focal_length = character.to_digit(10).unwrap();
            }
            '=' | '-' => {
                operation = match character {
                    '=' => Operation::Assign,
                    '-' => Operation::Remove,
                    _ => panic!("Invalid operation"),
                };
            }
            _ => {
                label.push(character);
            }
        }
    }

    Instruction {
        label,
        focal_length,
        operation,
        original_string: original_string.to_string(),
    }
}

impl TryFrom<FileLines> for Input {
    type Error = std::io::Error;

    fn try_from(lines: FileLines) -> Result<Self, Self::Error> {
        let mut init_sequence = Vec::new();
        let mut boxes: Vec<Vec<Instruction>> = Vec::new();
        for line in lines {
            let sections = line.split(',');
            for section in sections {
                init_sequence.push(parse_instruction(section));
            }
        }

        for i in 0..256 {
            boxes.push(vec![])
        }
        Ok(Input {
            init_sequence,
            boxes,
        })
    }
}

fn hash_string(input: &str) -> u64 {
    let mut current_value: u64 = 0;
    for character in input.chars() {
        let ascii_code = character as u8;
        current_value += ascii_code as u64;
        current_value *= 17;
        current_value %= 256;
    }
    current_value
}

fn part_1(input_file: &str) -> std::io::Result<u64> {
    let input: Input = parse(input_file)?;
    let mut all_values: Vec<u64> = vec![];
    for instruction in input.init_sequence {
        all_values.push(hash_string(&instruction.original_string));
    }
    Ok(all_values.iter().sum::<u64>())
}

fn part_2(input_file: &str) -> std::io::Result<u32> {
    let mut input: Input = parse(input_file)?;
    for instruction in input.init_sequence {
        let box_index = hash_string(&instruction.label);
        let current_box = &mut input.boxes[box_index as usize];
        if instruction.operation == Operation::Assign {
            if let Some((index, _)) = current_box
                .iter()
                .enumerate()
                .find(|(_, instruction_in_box)| instruction_in_box.label == instruction.label)
            {
                current_box[index].focal_length = instruction.focal_length;
            } else {
                current_box.push(instruction);
            }
        } else {
            if let Some((index, _)) = current_box
                .iter()
                .enumerate()
                .find(|(_, instruction_in_box)| instruction_in_box.label == instruction.label)
            {
                current_box.remove(index);
            }
        }
    }

    for (i, box_) in input.boxes.iter().enumerate() {
        // print the instructions in the box
        for instruction in box_ {
            println!("  [{} {}]", instruction.label, instruction.focal_length);
        }
    }

    // Calculate focus power, e.g. rn: 1 (box 0) * 1 (first slot) * 1 (focal length) = 1
    let mut total = 0;
    for (i, box_) in input.boxes.iter().enumerate() {
        for (slot, instruction) in box_.iter().enumerate() {
            let power = (i as u32 + 1) * (slot as u32 + 1) * instruction.focal_length;
            println!(
                "  [{} {}] power: {}",
                instruction.label, instruction.focal_length, power
            );
            total += power;
        }
        if total > 0 {
            println!("Box {} has focus power {}", i, total);
        }
    }

    Ok(total)
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
        assert_eq!(result.unwrap(), 514281);
    }

    #[test]
    fn roar_q15_p2_sample() {
        let result = part_2(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 145);
    }

    #[test]
    fn roar_q15_p2_main() {
        let result = part_2(INPUT);
        assert_eq!(result.unwrap(), 244199);
    }
}
