#![allow(dead_code, unused_variables)]

use crate::utils::parser::{parse, FileLines};

#[derive(Clone, Copy, Debug)]
enum Condition {
    Damaged,
    Operational,
    Unknown,
}

#[derive(Clone, Debug)]
struct ConditionRecord {
    records: Vec<Vec<Condition>>,
    mappings: Vec<Vec<u32>>,
}
struct Input {
    condition_record: ConditionRecord,
}

impl TryFrom<FileLines> for Input {
    type Error = std::io::Error;

    fn try_from(lines: FileLines) -> Result<Self, Self::Error> {
        let mut condition_record = ConditionRecord { records: vec![], mappings: vec![] };
        for line in lines {
            let records: Vec<&str> = line.split_whitespace().collect();
            let conditions = records[0];
            let mapping: Vec<&str> = records[1].split(',').collect();
            let mut condition_record_row = vec![];
            for c in conditions.chars() {
                match c {
                    '#' => {condition_record_row.push(Condition::Damaged);}
                    '.' => {condition_record_row.push(Condition::Operational);}
                    _ => {condition_record_row.push(Condition::Unknown);}
                }
            }
            let mut mapping_row = vec![];
            for m in mapping {
                mapping_row.push(m.parse::<u32>().unwrap());
            }
            condition_record.records.push(condition_record_row);
            condition_record.mappings.push(mapping_row);
        }
        Ok(Input { condition_record })
    }
}

fn part_1(input_file: &str) -> std::io::Result<u32> {
    let input: Input = parse(input_file)?;
    println!("{:?}", input.condition_record.records);
    println!("{:?}", input.condition_record.mappings);
    Ok(0)
}

fn part_2(input_file: &str) -> std::io::Result<u32> {
    let input: Input = parse(input_file)?;
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    const INPUT: &str = "input/roar/q12_input.txt";
    const INPUT_SAMPLE: &str = "input/roar/q12_sample.txt";

    #[test]
    fn roar_q12_p1_sample() {
        let result = part_1(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 21);
    }

    #[test]
    fn roar_q12_p1_main() {
        let result = part_1(INPUT);
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn roar_q12_p2_sample() {
        let result = part_2(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn roar_q12_p2_main() {
        let result = part_2(INPUT);
        assert_eq!(result.unwrap(), 0);
    }
}
