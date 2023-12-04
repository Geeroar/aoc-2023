use crate::utils::parser::FileLines;

struct Input {
    _value: u32
}

impl TryFrom<FileLines> for Input {
    type Error = std::io::Error;

    fn try_from(_lines: FileLines) -> Result<Self, Self::Error> {
        Ok(Input { _value: 0 })
    }
}

fn _part_1(input_file: &str) -> std::io::Result<u32> {
    let input = Input::try_from(FileLines::new(input_file)?)?;
    Ok(0)
}

fn _part_2(input_file: &str) -> std::io::Result<u32> {
    let input = Input::try_from(FileLines::new(input_file)?)?;
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::{_part_1, _part_2};

    const INPUT: &str = "input/roar/q04_input.txt";
    const INPUT_SAMPLE: &str = "input/roar/q04_sample.txt";

    #[test]
    fn roar_q04_p1_sample() {
        let result = _part_1(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn roar_q04_p1_main() {
        let result = _part_1(INPUT);
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn roar_q04_p2_sample() {
        let result = _part_2(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn roar_q04_p2_main() {
        let result = _part_2(INPUT);
        assert_eq!(result.unwrap(), 0);
    }
}
