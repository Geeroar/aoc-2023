#!/bin/bash

if [[ -z "$1" ]] || [[ ! "$2" =~ ^[0-9][0-9]$ ]]; then
    echo >&2 "Usage: $0 [ gee | roar ] [ 01 | .. | 25 ]"
    echo >&2
    echo >&2 "Create skeleton file from template in src/<xxxx>/q<xx>.rs"
    exit
fi

DAY=$2

BASE_SRC_PATH="src/${1}"
BASE_INPUT_PATH="input/${1}"

SAMPLE_1="${BASE_INPUT_PATH}/q${DAY}_p1_sample.txt"
SAMPLE_2="${BASE_INPUT_PATH}/q${DAY}_p2_sample.txt"
PUZZLE_INPUT="${BASE_INPUT_PATH}/q${DAY}_input.txt"

echo "Setting up input files at ${BASE_INPUT_PATH}/ ..."
touch $SAMPLE_1
touch $SAMPLE_2
touch $PUZZLE_INPUT

echo "Writing output to ${BASE_SRC_PATH}/q${DAY}.rs ..."
cat >"${BASE_SRC_PATH}/q${DAY}.rs" <<EOF
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

    const INPUT: &str = "${PUZZLE_INPUT}";
    const FIRST_INPUT_SAMPLE: &str = "${SAMPLE_1}";
    const SECOND_INPUT_SAMPLE: &str = "${SAMPLE_2}";

    #[test]
    fn q${DAY}_part_1_sample() {
        let result = _part_1(FIRST_INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn q${DAY}_part_1_main() {
        let result = _part_1(INPUT);
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn q${DAY}_part_2_sample() {
        let result = _part_2(SECOND_INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn q${DAY}_part_2_main() {
        let result = _part_2(INPUT);
        assert_eq!(result.unwrap(), 0);
    }
}
EOF

echo "pub mod q${DAY};" >>${BASE_SRC_PATH}/mod.rs

echo "Done!"
