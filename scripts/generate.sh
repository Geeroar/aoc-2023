#!/bin/bash

BASE_PATH=src/gee/

if [[ ! "$1" =~ ^[0-9][0-9]$ ]]; then
    echo >&2 "Usage: $0 [ 01 | .. | 25 ]"
    echo >&2
    echo >&2 "Create skeleton file from template in ${BASE_PATH}q<xx>.rs"
    exit
fi

echo "Writing output to ${BASE_PATH}q$1.rs ..."
cat > "${BASE_PATH}q$1.rs" <<EOF
use crate::gee::parser::FileLines;

struct Input {
    _value: u32
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

    const INPUT: &str = "input/gee/input$1.txt";
    const INPUT_SAMPLE: &str = "input/gee/input$1_sample.txt";

    #[test]
    fn q$1_part_1_sample() {
        let result = _part_1(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn q$1_part_1_main() {
        let result = _part_1(INPUT);
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn q$1_part_2_sample() {
        let result = _part_2(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn q$1_part_2_main() {
        let result = _part_2(INPUT);
        assert_eq!(result.unwrap(), 0);
    }
}
EOF
