#![allow(dead_code, unused_variables)]

use crate::utils::parser::{parse, FileLines};

struct Input {
    patterns: Vec<Vec<String>>,
}

fn find_reflection_line(pattern: &Vec<String>) -> usize {
    // First try to find a horizontal reflection
    match find_horizontal_reflection(pattern) {
        Some(row) => {
            println!("Found horizontal reflection at row {}", row);
            return 100 * (row + 1);
        } // 100 times the rows above the line
        None => {
            // If no horizontal reflection, then check for vertical
            match find_vertical_reflection(pattern) {
                Some(col) => {
                    println!("Found vertical reflection at column {}", col);
                    return col + 1;
                } // Columns to the left of the line
                None => 0, // If no reflection line found (should not happen in this puzzle)
            }
        }
    }
}

fn find_horizontal_reflection(pattern: &Vec<String>) -> Option<usize> {
    let n_rows = pattern.len();

    for row in 0..n_rows / 2 {
        let mirror_row = n_rows - row - 1;
        if (0..pattern[0].len())
            .all(|col| pattern[row].chars().nth(col) == pattern[mirror_row].chars().nth(col))
        {
            return Some(row);
        }
    }
    None
}

fn find_vertical_reflection(pattern: &Vec<String>) -> Option<usize> {
    let n_cols = pattern[0].len();

    for col in 0..n_cols / 2 {
        let mirror_col = n_cols - col - 1;
        if pattern
            .iter()
            .all(|row| row.chars().nth(col) == row.chars().nth(mirror_col))
        {
            return Some(col);
        }
    }
    None
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
        .map(|pattern| find_reflection_line(pattern))
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
        assert_eq!(result.unwrap(), 0);
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
