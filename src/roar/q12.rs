#![allow(dead_code, unused_variables)]

use crate::utils::parser::{parse, FileLines};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
        let mut condition_record = ConditionRecord {
            records: vec![],
            mappings: vec![],
        };
        for line in lines {
            let records: Vec<&str> = line.split_whitespace().collect();
            let conditions = records[0];
            let mapping: Vec<&str> = records[1].split(',').collect();
            let mut condition_record_row = vec![];
            for c in conditions.chars() {
                match c {
                    '#' => {
                        condition_record_row.push(Condition::Damaged);
                    }
                    '.' => {
                        condition_record_row.push(Condition::Operational);
                    }
                    _ => {
                        condition_record_row.push(Condition::Unknown);
                    }
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

fn is_valid_condition_record(record: Vec<Condition>, mapping: &Vec<u32>) -> bool {
    /*
    Check that the record is valid
    A valid record is a sequence of operational and damaged conditions
    where the number of damaged conditions matches the mapping.
     */
    let seen_damaged = count_damaged_conditions(record);
    seen_damaged == mapping.to_vec()
}

fn count_damaged_conditions(record: Vec<Condition>) -> Vec<u32> {
    let mut current_damaged = 0;
    let mut seen_damaged = Vec::new();
    for (i, condition) in record.iter().enumerate() {
        match condition {
            Condition::Operational => {
                if current_damaged > 0 {
                    seen_damaged.push(current_damaged)
                }
                current_damaged = 0;
            }
            Condition::Damaged => {
                current_damaged += 1;
            }
            _ => {
                panic!("Unknown condition")
            }
        }
    }
    if current_damaged > 0 {
        seen_damaged.push(current_damaged)
    }
    seen_damaged
}

fn count_possible_arrangements_for_row(
    record: Vec<Condition>,
    mapping: &Vec<u32>,
    start: u32,
) -> u32 {
    /*
       Recursively step through the record, trying out all possible arrangements
    */
    if record.len() as u32 == start {
        // If this mapping to record is valid, this means there's one more possible arrangement
        return if is_valid_condition_record(record, mapping) {
            1
        } else {
            0
        };
    }
    if record[start as usize] == Condition::Unknown {
        // Build a new record trying out broken in place of unknown
        let mut new_record_with_broken = record.clone();
        new_record_with_broken[start as usize] = Condition::Damaged;
        // Similarly, build a new record trying out operational in place of unknown
        let mut new_record_with_operational = record.clone();
        new_record_with_operational[start as usize] = Condition::Operational;
        return count_possible_arrangements_for_row(new_record_with_broken, &mapping, start + 1)
            + count_possible_arrangements_for_row(
                new_record_with_operational,
                &mapping,
                start + 1,
            );
    }
    return count_possible_arrangements_for_row(record, mapping, start + 1);
}

fn cound_possible_arrangements(records: Vec<Vec<Condition>>, mappings: Vec<Vec<u32>>) -> u32 {
    let mut count = 0;
    for (row, record) in records.iter().enumerate() {
        let row_mapping = mappings[row].to_vec();
        count += count_possible_arrangements_for_row(record.to_vec(), &row_mapping, 0);
    }
    count
}

fn part_1(input_file: &str) -> std::io::Result<u32> {
    let input: Input = parse(input_file)?;
    Ok(cound_possible_arrangements(
        input.condition_record.records,
        input.condition_record.mappings,
    ))
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
        assert_eq!(result.unwrap(), 7716);
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
