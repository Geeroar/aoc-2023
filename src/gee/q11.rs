#![allow(dead_code, unused_variables)]

use crate::utils::parser::{parse, FileLines};
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

struct Input {
    galaxies: Vec<(usize, usize)>,
}

impl TryFrom<FileLines> for Input {
    type Error = std::io::Error;

    fn try_from(lines: FileLines) -> Result<Self, Self::Error> {
        let mut galaxies = Vec::new();
        for (i, line) in lines.enumerate() {
            for (j, c) in line.chars().enumerate() {
                if c == '#' {
                    galaxies.push((i, j));
                }
            }
        }
        Ok(Input { galaxies })
    }
}

impl Input {
    fn expand_universe(&mut self, expansion_factor: usize) {
        self.expand_rows(expansion_factor);
        self.expand_columns(expansion_factor);
    }

    fn expand_rows(&mut self, expansion_factor: usize) {
        let all_rows: HashSet<usize> = self.galaxies.iter().map(|&(row, _)| row).collect();
        let expansion_map = build_expansion_map(expansion_factor, &all_rows);
        for i in 0..self.galaxies.len() {
            let (r, c) = self.galaxies[i];
            self.galaxies[i] = (r + expansion_map.get(&r).unwrap(), c);
        }
    }

    fn expand_columns(&mut self, expansion_factor: usize) {
        let all_columns: HashSet<usize> = self.galaxies.iter().map(|&(_, col)| col).collect();
        let expansion_map = build_expansion_map(expansion_factor, &all_columns);
        for i in 0..self.galaxies.len() {
            let (r, c) = self.galaxies[i];
            self.galaxies[i] = (r, c + expansion_map.get(&c).unwrap());
        }
    }

    fn shortest_paths(&self) -> Vec<usize> {
        let mut result = Vec::new();
        for i in 0..self.galaxies.len() {
            for j in (i + 1)..self.galaxies.len() {
                result.push(shortest_path(self.galaxies[i], self.galaxies[j]));
            }
        }
        result
    }
}

fn build_expansion_map(
    expansion_factor: usize,
    all_items: &HashSet<usize>,
) -> HashMap<usize, usize> {
    let max_item = *all_items.iter().max().unwrap();
    let empties: HashSet<usize> = (0..max_item)
        .filter(|i| !all_items.contains(i))
        .collect();
    let mut distance = 0;
    let mut expansion_map = HashMap::new();
    for i in 0..=max_item {
        if empties.contains(&i) {
            distance += expansion_factor - 1;
        } else {
            expansion_map.insert(i, distance);
        }
    }
    expansion_map
}

fn shortest_path((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> usize {
    max(x1, x2) - min(x1, x2) + max(y1, y2) - min(y1, y2)
}

fn part_1(input_file: &str) -> std::io::Result<usize> {
    let mut input: Input = parse(input_file)?;
    input.expand_universe(2);
    Ok(input.shortest_paths().into_iter().sum())
}

fn part_2(input_file: &str) -> std::io::Result<usize> {
    let mut input: Input = parse(input_file)?;
    input.expand_universe(1_000_000);
    Ok(input.shortest_paths().into_iter().sum())
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    const INPUT: &str = "input/gee/q11_input.txt";
    const INPUT_SAMPLE: &str = "input/gee/q11_sample.txt";

    #[test]
    fn gee_q11_p1_sample() {
        let result = part_1(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 374);
    }

    #[test]
    fn gee_q11_p1_main() {
        let result = part_1(INPUT);
        assert_eq!(result.unwrap(), 10292708);
    }

    #[test]
    fn gee_q11_p2_sample() {
        let result = part_2(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 82000210);
    }

    #[test]
    fn gee_q11_p2_main() {
        let result = part_2(INPUT);
        assert_eq!(result.unwrap(), 790194712336);
    }
}
