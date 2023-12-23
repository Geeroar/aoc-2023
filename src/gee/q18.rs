#![allow(dead_code, unused_variables)]

use crate::utils::parser::{parse, FileLines};
use std::collections::HashMap;

struct Input {
    dig_plan: Vec<(char, usize, String)>,
}

impl TryFrom<FileLines> for Input {
    type Error = std::io::Error;

    fn try_from(lines: FileLines) -> Result<Self, Self::Error> {
        let dig_plan = lines
            .map(|l| l.split_whitespace().map(String::from).collect::<Vec<_>>())
            .map(|s| {
                (
                    s[0].chars().next().unwrap(),
                    s[1].parse::<usize>().unwrap(),
                    String::from(s[2].strip_prefix("(#").unwrap().strip_suffix(')').unwrap()),
                )
            })
            .collect();
        Ok(Input { dig_plan })
    }
}

fn inner_area(start_point: (usize, usize), dig_plan: &[(char, usize, String)]) -> usize {
    let mut points = HashMap::<usize, Vec<(usize, char)>>::new();
    let (mut r, mut c) = start_point;
    for point in dig_plan {
        match *point {
            ('U', d, _) => {
                (0..=d).for_each(|i| {
                    points
                        .entry(r - i)
                        .and_modify(|v| v.push((c, 'U')))
                        .or_insert_with(|| vec![(c, 'U')]);
                });
                r -= d;
            }
            ('D', d, _) => {
                (0..=d).for_each(|i| {
                    points
                        .entry(r + i)
                        .and_modify(|v| v.push((c, 'D')))
                        .or_insert_with(|| vec![(c, 'D')]);
                });
                r += d;
            }
            ('L', d, _) => {
                (1..d).for_each(|i| {
                    points
                        .entry(r)
                        .and_modify(|v| v.push((c - i, 'L')))
                        .or_insert_with(|| vec![(c - i, 'L')]);
                });
                c -= d;
            }
            ('R', d, _) => {
                (1..d).for_each(|i| {
                    points
                        .entry(r)
                        .and_modify(|v| v.push((c + i, 'R')))
                        .or_insert_with(|| vec![(c + i, 'R')]);
                });
                c += d;
            }
            _ => panic!("Fail"),
        }
    }
    let mut area = 0;
    for (k, mut strip) in points {
        strip.sort();
        area += strip.len();
        let mut inside = false;
        let mut last = 0;
        for item in &strip {
            match *item {
                (d, 'U') => {
                    inside = true;
                    last = d;
                }
                (d, 'D') => {
                    if inside {
                        area += d - last - 1;
                        inside = false;
                    }
                }
                _ => inside = false,
            }
        }
    }
    area
}

fn convert_dig_plan(dig_plan: &[(char, usize, String)]) -> Vec<(char, usize, String)> {
    dig_plan
        .iter()
        .map(|(_, _, s)| {
            let dir = match s.chars().last() {
                Some('0') => 'R',
                Some('1') => 'D',
                Some('2') => 'L',
                Some('3') => 'U',
                x => panic!("Bad direction: {:?}", x),
            };
            let distance = usize::from_str_radix(&s[..5], 16).unwrap();
            (dir, distance, s.clone())
        })
        .collect()
}

fn part_1(input_file: &str) -> std::io::Result<usize> {
    let input: Input = parse(input_file)?;
    Ok(inner_area((333, 150), &input.dig_plan))
}

fn part_2(input_file: &str) -> std::io::Result<usize> {
    let input: Input = parse(input_file)?;
    let converted_dig_plan = convert_dig_plan(&input.dig_plan);
    Ok(inner_area((10000000, 10000000), &converted_dig_plan))
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    const INPUT: &str = "input/gee/q18_input.txt";
    const INPUT_SAMPLE: &str = "input/gee/q18_sample.txt";

    #[test]
    fn gee_q18_p1_sample() {
        let result = part_1(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 62);
    }

    #[test]
    fn gee_q18_p1_main() {
        let result = part_1(INPUT);
        assert_eq!(result.unwrap(), 108909);
    }

    #[ignore = "takes too long"]
    #[test]
    fn gee_q18_p2_sample() {
        let result = part_2(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 952408144115);
    }

    #[ignore = "takes too long"]
    #[test]
    fn gee_q18_p2_main() {
        let result = part_2(INPUT);
        assert_eq!(result.unwrap(), 133125706867777);
    }
}
