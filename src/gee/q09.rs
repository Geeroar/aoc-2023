#![allow(dead_code, unused_variables)]

use crate::utils::parser::{parse, FileLines};

struct Input {
    histories: Vec<History>,
}

struct History {
    sequence: Vec<i32>,
}

impl TryFrom<FileLines> for Input {
    type Error = std::io::Error;

    fn try_from(lines: FileLines) -> Result<Self, Self::Error> {
        let histories = lines
            .map(|l| History {
                sequence: l.split_whitespace().map(|n| n.parse().unwrap()).collect(),
            })
            .collect();

        Ok(Input { histories })
    }
}

impl History {
    fn next_value(&self) -> i32 {
        let levels = self.build_levels();
        let mut n = 0;
        for i in 1..=levels.len() {
            let index = levels.len() - i;
            n += levels[index].last().unwrap();
        }
        n
    }

    fn previous_value(&self) -> i32 {
        let levels = self.build_levels();
        let mut n = 0;
        for i in 1..=levels.len() {
            let index = levels.len() - i;
            n = levels[index].first().unwrap() - n;
        }
        n
    }

    fn build_levels(&self) -> Vec<Vec<i32>> {
        let mut levels: Vec<Vec<i32>> = vec![self.sequence.to_vec()];
        let mut size = levels[0].len();
        while levels.last().unwrap().iter().any(|&n| n != 0) {
            let mut next_level: Vec<i32> = Vec::new();
            for i in 0..(size - 1) {
                let v = &levels.last().unwrap();
                next_level.push(v[i + 1] - v[i]);
            }
            size = next_level.len();
            levels.push(next_level);
        }
        levels
    }
}

fn part_1(input_file: &str) -> std::io::Result<i32> {
    let input = parse::<Input>(input_file)?;
    Ok(input.histories.iter().map(|h| h.next_value()).sum())
}

fn part_2(input_file: &str) -> std::io::Result<i32> {
    let input = parse::<Input>(input_file)?;
    Ok(input.histories.iter().map(|h| h.previous_value()).sum())
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    const INPUT: &str = "input/gee/q09_input.txt";
    const INPUT_SAMPLE: &str = "input/gee/q09_sample.txt";

    #[test]
    fn gee_q09_p1_sample() {
        let result = part_1(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 114);
    }

    #[test]
    fn gee_q09_p1_main() {
        let result = part_1(INPUT);
        assert_eq!(result.unwrap(), 1762065988);
    }

    #[test]
    fn gee_q09_p2_sample() {
        let result = part_2(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 2);
    }

    #[test]
    fn gee_q09_p2_main() {
        let result = part_2(INPUT);
        assert_eq!(result.unwrap(), 1066);
    }
}
