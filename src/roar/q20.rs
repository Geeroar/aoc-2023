#![allow(dead_code, unused_variables)]

use crate::utils::parser::{parse, FileLines};

struct Input {
    value: usize,
}

impl TryFrom<FileLines> for Input {
    type Error = std::io::Error;

    fn try_from(lines: FileLines) -> Result<Self, Self::Error> {
        Ok(Input { value: 0 })
    }
}

fn part_1(input_file: &str) -> std::io::Result<usize> {
    let input: Input = parse(input_file)?;
    Ok(0)
}

fn part_2(input_file: &str) -> std::io::Result<usize> {
    let input: Input = parse(input_file)?;
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    const INPUT: &str = "input/roar/q20_input.txt";
    const INPUT_SAMPLE: &str = "input/roar/q20_sample.txt";

    #[test]
    fn roar_q20_p1_sample() {
        let result = part_1(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn roar_q20_p1_main() {
        let result = part_1(INPUT);
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn roar_q20_p2_sample() {
        let result = part_2(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn roar_q20_p2_main() {
        let result = part_2(INPUT);
        assert_eq!(result.unwrap(), 0);
    }
}
