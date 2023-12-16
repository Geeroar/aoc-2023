#![allow(dead_code, unused_variables)]

use crate::utils::parser::{parse, FileLines};
use std::collections::HashSet;

type Beam = (usize, usize, Direction);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Input {
    grid: Vec<Vec<char>>,
}

impl TryFrom<FileLines> for Input {
    type Error = std::io::Error;

    fn try_from(lines: FileLines) -> Result<Self, Self::Error> {
        let grid = lines.map(|l| l.chars().collect()).collect();
        Ok(Input { grid })
    }
}

impl Input {
    fn find_activations(&self, starting_point: Beam) -> usize {
        let mut beams = vec![starting_point];
        let mut visited = HashSet::<Beam>::new();
        loop {
            let mut new_beams = HashSet::<Beam>::new();
            while let Some(beam) = beams.pop() {
                if visited.contains(&beam) {
                    continue;
                }
                visited.insert(beam.clone());
                let (r, c, direction) = (beam.0, beam.1, &beam.2);
                match (self.grid[r][c], direction) {
                    ('/', Direction::Up) | ('\\', Direction::Down) => {
                        if c < self.grid[0].len() - 1 {
                            new_beams.insert((r, c + 1, Direction::Right));
                        }
                    }
                    ('/', Direction::Down) | ('\\', Direction::Up) => {
                        if c > 0 {
                            new_beams.insert((r, c - 1, Direction::Left));
                        }
                    }
                    ('/', Direction::Left) | ('\\', Direction::Right) => {
                        if r < self.grid.len() - 1 {
                            new_beams.insert((r + 1, c, Direction::Down));
                        }
                    }
                    ('/', Direction::Right) | ('\\', Direction::Left) => {
                        if r > 0 {
                            new_beams.insert((r - 1, c, Direction::Up));
                        }
                    }
                    ('-', Direction::Up) | ('-', Direction::Down) => {
                        if c > 0 {
                            new_beams.insert((r, c - 1, Direction::Left));
                        }
                        if c < self.grid[0].len() - 1 {
                            new_beams.insert((r, c + 1, Direction::Right));
                        }
                    }
                    ('|', Direction::Left) | ('|', Direction::Right) => {
                        if r > 0 {
                            new_beams.insert((r - 1, c, Direction::Up));
                        }
                        if r < self.grid.len() - 1 {
                            new_beams.insert((r + 1, c, Direction::Down));
                        }
                    }
                    ('.', Direction::Up) | ('|', Direction::Up) => {
                        if r > 0 {
                            new_beams.insert((r - 1, c, Direction::Up));
                        }
                    }
                    ('.', Direction::Down) | ('|', Direction::Down) => {
                        if r < self.grid.len() - 1 {
                            new_beams.insert((r + 1, c, Direction::Down));
                        }
                    }
                    ('.', Direction::Left) | ('-', Direction::Left) => {
                        if c > 0 {
                            new_beams.insert((r, c - 1, Direction::Left));
                        }
                    }
                    ('.', Direction::Right) | ('-', Direction::Right) => {
                        if c < self.grid[0].len() - 1 {
                            new_beams.insert((r, c + 1, Direction::Right));
                        }
                    }
                    (x, _) => panic!("Invalid grid item: {}", x),
                }
            }
            if new_beams.is_empty() {
                break;
            } else {
                beams = new_beams.into_iter().collect();
            }
        }
        visited
            .into_iter()
            .map(|(r, c, _)| (r, c))
            .collect::<HashSet<_>>()
            .len()
    }
}

fn part_1(input_file: &str) -> std::io::Result<usize> {
    let input: Input = parse(input_file)?;
    Ok(input.find_activations((0, 0, Direction::Right)))
}

fn part_2(input_file: &str) -> std::io::Result<usize> {
    let input: Input = parse(input_file)?;
    let mut result = 0;
    for i in 0..input.grid.len() {
        let l = input.find_activations((i, input.grid[0].len() - 1, Direction::Left));
        if l > result {
            result = l;
        }
        let r = input.find_activations((i, 0, Direction::Right));
        if r > result {
            result = r;
        }
    }
    for i in 0..input.grid[0].len() {
        let u = input.find_activations((input.grid.len() - 1, i, Direction::Up));
        if u > result {
            result = u;
        }
        let d = input.find_activations((0, i, Direction::Down));
        if d > result {
            result = d;
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    const INPUT: &str = "input/gee/q16_input.txt";
    const INPUT_SAMPLE: &str = "input/gee/q16_sample.txt";

    #[test]
    fn gee_q16_p1_sample() {
        let result = part_1(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 46);
    }

    #[test]
    fn gee_q16_p1_main() {
        let result = part_1(INPUT);
        assert_eq!(result.unwrap(), 7472);
    }

    #[test]
    fn gee_q16_p2_sample() {
        let result = part_2(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 51);
    }

    #[ignore]
    #[test]
    fn gee_q16_p2_main() {
        let result = part_2(INPUT);
        assert_eq!(result.unwrap(), 7716);
    }
}
