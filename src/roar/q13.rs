#![allow(dead_code, unused_variables)]

use crate::utils::parser::{parse, FileLines};
use crate::utils::transposer::transpose_vec_of_strings;

struct Input {
    patterns: Vec<Vec<String>>,
}

fn is_perfect_reflection_from_point(pattern: Vec<String>, point: usize) -> bool {
    let mut right = point + 1;
    let mut left = point - 2;
    while right < pattern.len() {
        let row = &pattern[right];
        let reflection_row = &pattern[left];
        if row != reflection_row {
            return false;
        }
        if left == 0 {
            break;
        }
        left -= 1;
        right += 1;
    }
    return true;
}

fn find_reflection(pattern: Vec<String>) -> Option<usize> {
    let size = pattern.len();
    for (i, pat) in pattern.iter().enumerate() {
        println!("Checking pattern: {}", pat);
        if i == size - 1 {
            break;
        }
        if *pat == pattern[i + 1] {
            if i == 0 {
                return Some(i + 1);
            }
            // Check the reflection holds from this point by scanning out from here
            if is_perfect_reflection_from_point(pattern.clone(), i + 1) {
                return Some(i + 1);
            }
        }
    }
    return None;
}

fn calculate_pattern_summary(pattern: &Vec<String>) -> usize {
    let reflection_row = find_reflection(pattern.to_vec());

    if reflection_row.is_some() {
        let row = reflection_row.unwrap();
        println!("Found reflection at row {}", row);
        return row * 100;
    }
    let transposed_pattern = transpose_vec_of_strings(pattern.to_vec());
    let reflection_col = find_reflection(transposed_pattern).unwrap();
    println!("Found reflection at col {}", reflection_col);

    return reflection_col;
}

impl TryFrom<FileLines> for Input {
    type Error = std::io::Error;

    fn try_from(lines: FileLines) -> Result<Self, Self::Error> {
        let mut patterns = Vec::new();
        let mut current_pattern = Vec::new();
        for line in lines {
            if line.is_empty() {
                if !current_pattern.is_empty() {
                    patterns.push(current_pattern.clone());
                    current_pattern.clear();
                }
            } else {
                current_pattern.push(line);
            }
        }
        if !current_pattern.is_empty() {
            patterns.push(current_pattern);
        }

        Ok(Input { patterns })
    }
}

fn part_1(input_file: &str) -> std::io::Result<u32> {
    let input: Input = parse(input_file)?;
    let sum = input
        .patterns
        .iter()
        .map(|pattern| calculate_pattern_summary(pattern))
        .sum::<usize>();
    println!("Total sum: {}", sum);
    Ok(sum as u32)
}

fn part_2(input_file: &str) -> std::io::Result<u32> {
    let input: Input = parse(input_file)?;
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    const INPUT: &str = "input/roar/q13_input.txt";
    const INPUT_SAMPLE: &str = "input/roar/q13_sample.txt";

    #[test]
    fn roar_q13_p1_sample() {
        let result = part_1(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 405);
    }

    #[test]
    fn roar_q13_p1_main() {
        let result = part_1(INPUT);
        // TODO: don't know this answer yet
        assert_eq!(result.unwrap(), 34202);
    }

    #[test]
    fn roar_q13_p2_sample() {
        let result = part_2(INPUT_SAMPLE);
        // TODO: don't know this answer yet
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn roar_q13_p2_main() {
        let result = part_2(INPUT);
        // TODO: don't know this answer yet
        assert_eq!(result.unwrap(), 0);
    }
}
