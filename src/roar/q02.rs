use crate::utils::parser::FileLines;

struct Input {
    _value: u32,
}

impl TryFrom<FileLines> for Input {
    type Error = std::io::Error;

    fn try_from(_lines: FileLines) -> Result<Self, Self::Error> {
        Ok(Input { _value: 0 })
    }
}

fn _part_1(input: &str) -> std::io::Result<u32> {
    let input = Input::try_from(FileLines::new(input)?)?;
    Ok(input._value)
}

fn _part_2(input: &str) -> std::io::Result<u32> {
    let input = Input::try_from(FileLines::new(input)?)?;
    Ok(input._value)
}

#[cfg(test)]
mod tests {
    use super::{_part_1, _part_2};

    const INPUT: &str = "input/roar/q02_input.txt";
    const FIRST_INPUT_SAMPLE: &str = "input/roar/q02_p1_sample.txt";
    const SECOND_INPUT_SAMPLE: &str = "input/roar/q02_p2_sample.txt";

    #[test]
    fn q02_part_1_sample() {
        let result = _part_1(FIRST_INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn q02_part_1_main() {
        let result = _part_1(INPUT);
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn q02_part_2_sample() {
        let result = _part_2(SECOND_INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn q02_part_2_main() {
        let result = _part_2(INPUT);
        assert_eq!(result.unwrap(), 0);
    }
}
