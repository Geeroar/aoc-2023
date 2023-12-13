#![allow(dead_code, unused_variables)]

use crate::utils::parser::{parse, FileLines};
use std::collections::HashMap;

#[derive(Debug)]
struct Input {
    rows: Vec<Row>,
}

#[derive(Debug)]
struct Row {
    springs: Vec<char>,
    groups: Vec<usize>,
}

impl TryFrom<FileLines> for Input {
    type Error = std::io::Error;

    fn try_from(lines: FileLines) -> Result<Self, Self::Error> {
        let rows = lines
            .map(|l| {
                let (a, b) = l.split_once(' ').unwrap();
                let springs = a.chars().collect();
                let groups = b.split(',').map(|n| n.parse().unwrap()).collect();
                Row { springs, groups }
            })
            .collect();
        Ok(Input { rows })
    }
}

impl Row {
    fn expand_row(&mut self, multiplier: usize) {
        let mut new_springs = Vec::new();
        let mut new_groups = Vec::new();
        for i in 0..multiplier {
            for c in &self.springs {
                new_springs.push(*c);
            }
            if i < multiplier - 1 {
                new_springs.push('?');
            }
            for g in &self.groups {
                new_groups.push(*g);
            }
        }
        self.springs = new_springs;
        self.groups = new_groups;
    }

    fn match_groups(&self) -> usize {
        let mut queue = Vec::<(usize, usize)>::new();
        let mut full_matches = 0;
        queue.push((0, 1));
        for (i, &group_size) in self.groups.iter().enumerate() {
            let mut next = HashMap::<usize, usize>::new();
            let remaining_groups_size = self.groups[i + 1..].iter().sum::<usize>();
            let remaining_groups_boundaries = self.groups.len() - (i + 1);
            let space_for_remaining_groups = remaining_groups_size + remaining_groups_boundaries;
            let end_index = self.springs.len() - space_for_remaining_groups;
            while !queue.is_empty() {
                let (start_index, count) = queue.pop().unwrap();
                for index in start_index..end_index {
                    if self.matches(index, group_size) {
                        let next_index = index + group_size + 1;
                        let remaining_damaged_springs: usize = (next_index..self.springs.len())
                            .map(|i| self.springs[i])
                            .filter(|&s| s == '#')
                            .count();
                        if i < self.groups.len() - 1
                            && remaining_damaged_springs <= remaining_groups_size
                        {
                            next.entry(next_index)
                                .and_modify(|c| *c += count)
                                .or_insert(count);
                        } else if remaining_damaged_springs == 0 {
                            full_matches += count;
                        }
                    }
                    if self.springs[index] == '#' {
                        break;
                    }
                }
            }
            queue = next.into_iter().map(|(k, v)| (k, v)).collect();
        }
        full_matches
    }

    fn matches(&self, index: usize, group_size: usize) -> bool {
        let length = self.springs.len();
        let separate_from_next_group =
            |i, g| i + g >= self.springs.len() || self.springs[i + g] != '#';
        let enough_space_remaining = |i, g| i + g <= length;
        let all_damaged = |i, g| (0..g).all(|j| self.springs[i + j] != '.');
        enough_space_remaining(index, group_size)
            && separate_from_next_group(index, group_size)
            && all_damaged(index, group_size)
    }
}

fn part_1(input_file: &str) -> std::io::Result<usize> {
    let input: Input = parse(input_file)?;
    Ok(input.rows.iter().map(|r| r.match_groups()).sum())
}

pub fn part_2(input_file: &str) -> std::io::Result<usize> {
    let mut input: Input = parse(input_file)?;
    input.rows.iter_mut().for_each(|r| r.expand_row(5));
    Ok(input.rows.iter().map(|r| r.match_groups()).sum())
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    const INPUT: &str = "input/gee/q12_input.txt";
    const INPUT_SAMPLE: &str = "input/gee/q12_sample.txt";

    #[test]
    fn gee_q12_p1_sample() {
        let result = part_1(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 21);
    }

    #[test]
    fn gee_q12_p1_main() {
        let result = part_1(INPUT);
        assert_eq!(result.unwrap(), 7490);
    }

    #[test]
    fn gee_q12_p2_sample() {
        let result = part_2(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 525152);
    }

    #[test]
    fn gee_q12_p2_main() {
        let result = part_2(INPUT);
        assert_eq!(result.unwrap(), 65607131946466);
    }
}
