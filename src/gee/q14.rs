#![allow(dead_code, unused_variables)]

use crate::utils::parser::{parse, FileLines};
use std::collections::HashSet;

type Loop = (usize, usize);

#[derive(Debug)]
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
    fn spin(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }

    fn find_loop(&mut self) -> Loop {
        let mut states = Vec::<String>::new();
        let mut spins = 0;
        loop {
            self.spin();
            spins += 1;
            let state = self.as_string();
            if states.contains(&state) {
                let start = states.iter().position(|x| x == &state).unwrap() + 1;
                let length = spins - start;
                return (start, length);
            }
            states.push(state);
        }
    }

    fn spin_to_state(&mut self, target_spins: usize, loop_spec: Loop) {
        let (start, length) = loop_spec;
        let target_state = (target_spins - start + 1) % length;
        let remaining_spins = target_state - 1;
        for _ in 0..remaining_spins {
            self.spin();
        }
    }

    fn find_rolling_rocks(&self) -> HashSet<(usize, usize)> {
        let mut result = HashSet::new();
        for (i, row) in self.grid.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                if self.grid[i][j] == 'O' {
                    result.insert((i, j));
                }
            }
        }
        result
    }

    fn tilt_north(&mut self) {
        for c in 0..self.grid[0].len() {
            let mut current_support = 0;
            for r in 0..self.grid.len() {
                match self.grid[r][c] {
                    'O' => {
                        if r > current_support {
                            self.grid[current_support][c] = 'O';
                            self.grid[r][c] = '.';
                            current_support += 1;
                        } else {
                            current_support += 1;
                        }
                    }
                    '#' => current_support = r + 1,
                    _ => (),
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        let row_count = self.grid.len();
        for c in 0..self.grid[0].len() {
            let mut current_support = row_count;
            for r in 1..=row_count {
                match self.grid[row_count - r][c] {
                    'O' => {
                        if row_count - r + 1 < current_support {
                            self.grid[current_support - 1][c] = 'O';
                            self.grid[row_count - r][c] = '.';
                            current_support -= 1;
                        } else {
                            current_support -= 1;
                        }
                    }
                    '#' => current_support = row_count - r,
                    _ => (),
                }
            }
        }
    }

    fn tilt_west(&mut self) {
        for r in 0..self.grid.len() {
            let mut current_support = 0;
            for c in 0..self.grid[0].len() {
                match self.grid[r][c] {
                    'O' => {
                        if c > current_support {
                            self.grid[r][current_support] = 'O';
                            self.grid[r][c] = '.';
                            current_support += 1;
                        } else {
                            current_support += 1;
                        }
                    }
                    '#' => current_support = c + 1,
                    _ => (),
                }
            }
        }
    }

    fn tilt_east(&mut self) {
        let col_count = self.grid[0].len();
        for r in 0..self.grid.len() {
            let mut current_support = col_count;
            for c in 1..=col_count {
                match self.grid[r][col_count - c] {
                    'O' => {
                        if col_count - c + 1 < current_support {
                            self.grid[r][current_support - 1] = 'O';
                            self.grid[r][col_count - c] = '.';
                            current_support -= 1;
                        } else {
                            current_support -= 1;
                        }
                    }
                    '#' => current_support = col_count - c,
                    _ => (),
                }
            }
        }
    }

    fn total_load(&self) -> usize {
        let mut load = 0;
        let row_count = self.grid.len();
        for (i, row) in self.grid.iter().enumerate() {
            for col in row {
                if col == &'O' {
                    load += row_count - i;
                }
            }
        }
        load
    }

    fn print(&self) {
        for row in &self.grid {
            println!("{}", String::from_iter(row.iter()));
        }
        println!();
    }

    fn as_string(&self) -> String {
        String::from_iter(self.grid.iter().map(|r| String::from_iter(r.iter())))
    }
}

fn part_1(input_file: &str) -> std::io::Result<usize> {
    let mut input: Input = parse(input_file)?;
    input.tilt_north();
    Ok(input.total_load())
}

fn part_2(input_file: &str) -> std::io::Result<usize> {
    let mut input: Input = parse(input_file)?;
    let loop_spec = input.find_loop();
    input.spin_to_state(1_000_000_000, loop_spec);
    Ok(input.total_load())
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    const INPUT: &str = "input/gee/q14_input.txt";
    const INPUT_SAMPLE: &str = "input/gee/q14_sample.txt";

    #[test]
    fn gee_q14_p1_sample() {
        let result = part_1(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 136);
    }

    #[test]
    fn gee_q14_p1_main() {
        let result = part_1(INPUT);
        assert_eq!(result.unwrap(), 105208);
    }

    #[test]
    fn gee_q14_p2_sample() {
        let result = part_2(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 64);
    }

    #[test]
    fn gee_q14_p2_main() {
        let result = part_2(INPUT);
        assert_eq!(result.unwrap(), 102943);
    }
}
