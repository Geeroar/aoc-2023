#![allow(dead_code, unused_variables)]

use crate::utils::parser::{parse, FileLines};
use std::cmp::min;

struct Input {
    patterns: Vec<Pattern>,
}

struct Pattern {
    grid: Vec<Vec<char>>,
}

impl TryFrom<FileLines> for Input {
    type Error = std::io::Error;

    fn try_from(lines: FileLines) -> Result<Self, Self::Error> {
        let mut patterns = Vec::new();
        let mut grid: Vec<Vec<char>> = Vec::new();
        for line in lines {
            if line.is_empty() {
                patterns.push(Pattern { grid: grid.clone() });
                grid.clear();
            } else {
                grid.push(line.chars().collect());
            }
        }
        patterns.push(Pattern { grid: grid.clone() });
        Ok(Input { patterns })
    }
}

impl Pattern {
    fn find_reflection_value(&self) -> usize {
        let columns: Vec<String> = (0..self.grid[0].len()).map(|i| self.column(i)).collect();
        let rows: Vec<String> = (0..self.grid.len()).map(|i| self.row(i)).collect();
        find_reflection_values(&columns, &rows)[0]
    }

    fn find_reflection_value_smudge(&self) -> usize {
        let original_reflection_value = self.find_reflection_value();
        for row in 0..self.grid.len() {
            for col in 0..self.grid[0].len() {
                let columns: Vec<String> = (0..self.grid[0].len())
                    .map(|i| self.column_with_smudge(i, (row, col)))
                    .collect();
                let rows: Vec<String> = (0..self.grid.len())
                    .map(|i| self.row_with_smudge(i, (row, col)))
                    .collect();
                let reflection_values: Vec<usize> = find_reflection_values(&columns, &rows)
                    .into_iter()
                    .filter(|&v| v != original_reflection_value)
                    .collect();
                if !reflection_values.is_empty() {
                    return reflection_values[0];
                }
            }
        }
        0
    }

    fn column(&self, col: usize) -> String {
        (0..self.grid.len())
            .map(|row| self.grid[row][col])
            .collect()
    }

    fn row(&self, row: usize) -> String {
        (0..self.grid[0].len())
            .map(|col| self.grid[row][col])
            .collect()
    }

    fn column_with_smudge(&self, col: usize, smudge: (usize, usize)) -> String {
        (0..self.grid.len())
            .map(|row| {
                if (row, col) == smudge {
                    match self.grid[row][col] {
                        '#' => '.',
                        _ => '#',
                    }
                } else {
                    self.grid[row][col]
                }
            })
            .collect()
    }

    fn row_with_smudge(&self, row: usize, smudge: (usize, usize)) -> String {
        (0..self.grid[0].len())
            .map(|col| {
                if (row, col) == smudge {
                    match self.grid[row][col] {
                        '#' => '.',
                        _ => '#',
                    }
                } else {
                    self.grid[row][col]
                }
            })
            .collect()
    }
}

fn find_reflection_values(columns: &Vec<String>, rows: &Vec<String>) -> Vec<usize> {
    let mut results = Vec::new();
    let mut mirror: bool = false;
    for i in 1..columns.len() {
        let min_distance_to_edge = min(i, columns.len() - i);
        for j in 0..min_distance_to_edge {
            if columns[i - 1 - j] == columns[i + j] {
                mirror = true;
            } else {
                mirror = false;
                break;
            }
        }
        if mirror {
            results.push(i);
            mirror = false;
        }
    }
    for i in 1..rows.len() {
        let min_distance_to_edge = min(i, rows.len() - i);
        for j in 0..min_distance_to_edge {
            if rows[i - 1 - j] == rows[i + j] {
                mirror = true;
            } else {
                mirror = false;
                break;
            }
        }
        if mirror {
            results.push(100 * i);
            mirror = false;
        }
    }
    results
}

fn part_1(input_file: &str) -> std::io::Result<usize> {
    let input: Input = parse(input_file)?;
    Ok(input
        .patterns
        .iter()
        .map(|p| p.find_reflection_value())
        .sum())
}

fn part_2(input_file: &str) -> std::io::Result<usize> {
    let input: Input = parse(input_file)?;
    Ok(input
        .patterns
        .iter()
        .map(|p| p.find_reflection_value_smudge())
        .sum())
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    const INPUT: &str = "input/gee/q13_input.txt";
    const INPUT_SAMPLE: &str = "input/gee/q13_sample.txt";

    #[test]
    fn gee_q13_p1_sample() {
        let result = part_1(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 405);
    }

    #[test]
    fn gee_q13_p1_main() {
        let result = part_1(INPUT);
        assert_eq!(result.unwrap(), 35521);
    }

    #[test]
    fn gee_q13_p2_sample() {
        let result = part_2(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 400);
    }

    #[test]
    fn gee_q13_p2_main() {
        let result = part_2(INPUT);
        assert_eq!(result.unwrap(), 34795);
    }
}
