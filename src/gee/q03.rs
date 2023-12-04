use crate::utils::parser::FileLines;
use std::cmp::max;
use std::collections::HashSet;

#[derive(Debug)]
struct Input {
    _numbers: Vec<(u32, Location)>,
    _symbols: Vec<Location>,
}

#[derive(Debug)]
struct Location {
    _line: usize,
    _start: usize,
    _end: usize,
}

impl TryFrom<FileLines> for Input {
    type Error = std::io::Error;

    fn try_from(_lines: FileLines) -> Result<Self, Self::Error> {
        let mut _numbers = Vec::new();
        let mut _symbols = Vec::new();
        for (i, line) in _lines.enumerate() {
            let mut current_number = String::new();
            let mut number_start = 0;
            for (j, c) in line.chars().enumerate() {
                match c {
                    '0'..='9' => {
                        if current_number.is_empty() {
                            number_start = j;
                        }
                        current_number.push(c);
                    }
                    _ => {
                        if !current_number.is_empty() {
                            let location = Location {
                                _line: i,
                                _start: number_start,
                                _end: j - 1,
                            };
                            _numbers.push((current_number.parse().unwrap(), location));
                            current_number.clear();
                        }
                        if c != '.' {
                            _symbols.push(Location {
                                _line: i,
                                _start: j,
                                _end: j,
                            })
                        }
                    }
                }
            }
            if !current_number.is_empty() {
                let location = Location {
                    _line: i,
                    _start: number_start,
                    _end: line.len() - 1,
                };
                _numbers.push((current_number.parse().unwrap(), location));
            }
        }
        Ok(Input { _numbers, _symbols })
    }
}

impl Input {
    fn _symbol_reach(&self) -> HashSet<(usize, usize)> {
        let mut result = HashSet::new();
        for symbol in &self._symbols {
            symbol._span().iter().for_each(|&s| {
                result.insert(s);
            });
        }
        result
    }
}

impl Location {
    fn _overlaps(&self, points: &HashSet<(usize, usize)>) -> bool {
        for i in self._start..=self._end {
            if points.contains(&(i, self._line)) {
                return true;
            }
        }
        false
    }

    fn _span(&self) -> HashSet<(usize, usize)> {
        let mut result = HashSet::new();
        let (x, y) = (self._start, self._line);
        let (x1, x2) = (max(x - 1, 0), x + 1);
        let (y1, y2) = (max(y - 1, 0), y + 1);
        for i in x1..=x2 {
            for j in y1..=y2 {
                result.insert((i, j));
            }
        }
        result
    }

    fn _gear_ratio(&self, numbers: &Vec<(u32, Location)>) -> u32 {
        let span = self._span();
        let results: Vec<&(u32, Location)> =
            numbers.iter().filter(|n| n.1._overlaps(&span)).collect();
        if results.len() == 2 {
            results[0].0 * results[1].0
        } else {
            0
        }
    }
}

fn _part_1(input_file: &str) -> std::io::Result<u32> {
    let input = Input::try_from(FileLines::new(input_file)?)?;
    let symbols = input._symbol_reach();
    Ok(input
        ._numbers
        .iter()
        .filter(|n| n.1._overlaps(&symbols))
        .map(|n| n.0)
        .sum())
}

fn _part_2(input_file: &str) -> std::io::Result<u32> {
    let input = Input::try_from(FileLines::new(input_file)?)?;
    Ok(input
        ._symbols
        .iter()
        .map(|s| s._gear_ratio(&input._numbers))
        .sum())
}

#[cfg(test)]
mod tests {
    use super::{_part_1, _part_2};

    const INPUT: &str = "input/gee/q03_input.txt";
    const INPUT_SAMPLE: &str = "input/gee/q03_sample.txt";

    #[test]
    fn gee_q03_p1_sample() {
        let result = _part_1(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 4361);
    }

    #[test]
    fn gee_q03_p1_main() {
        let result = _part_1(INPUT);
        assert_eq!(result.unwrap(), 507214);
    }

    #[test]
    fn gee_q03_p2_sample() {
        let result = _part_2(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 467835);
    }

    #[test]
    fn gee_q03_p2_main() {
        let result = _part_2(INPUT);
        assert_eq!(result.unwrap(), 72553319);
    }
}
