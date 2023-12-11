#!/bin/bash

if [[ -z "$1" ]] || [[ ! "$2" =~ ^[0-9][0-9]$ ]]; then
    echo >&2 "Usage: $0 [ gee | roar ] [ 01 | .. | 25 ]"
    echo >&2
    echo >&2 "Create skeleton file from template in src/<xxxx>/q<xx>.rs"
    exit
fi

MASTER_PROGRAMMER=$1
DAY=$2

BASE_SRC_PATH="src/${MASTER_PROGRAMMER}"
BASE_INPUT_PATH="input/${MASTER_PROGRAMMER}"

SAMPLE_INPUT="${BASE_INPUT_PATH}/q${DAY}_sample.txt"
PUZZLE_INPUT="${BASE_INPUT_PATH}/q${DAY}_input.txt"

echo "Setting up input files at ${BASE_INPUT_PATH}/ ..."
touch "$SAMPLE_INPUT"
touch "$PUZZLE_INPUT"

echo "Writing output to ${BASE_SRC_PATH}/q${DAY}.rs ..."
cat >"${BASE_SRC_PATH}/q${DAY}.rs" <<EOF
#![allow(dead_code, unused_variables)]

use crate::utils::parser::{parse, FileLines};

struct Input {
    value: u32
}

impl TryFrom<FileLines> for Input {
    type Error = std::io::Error;

    fn try_from(lines: FileLines) -> Result<Self, Self::Error> {
        Ok(Input { value: 0 })
    }
}

fn part_1(input_file: &str) -> std::io::Result<u32> {
    let input: Input = parse(input_file)?;
    Ok(0)
}

fn part_2(input_file: &str) -> std::io::Result<u32> {
    let input: Input = parse(input_file)?;
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    const INPUT: &str = "${PUZZLE_INPUT}";
    const INPUT_SAMPLE: &str = "${SAMPLE_INPUT}";

    #[test]
    fn ${MASTER_PROGRAMMER}_q${DAY}_p1_sample() {
        let result = part_1(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn ${MASTER_PROGRAMMER}_q${DAY}_p1_main() {
        let result = part_1(INPUT);
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn ${MASTER_PROGRAMMER}_q${DAY}_p2_sample() {
        let result = part_2(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn ${MASTER_PROGRAMMER}_q${DAY}_p2_main() {
        let result = part_2(INPUT);
        assert_eq!(result.unwrap(), 0);
    }
}
EOF

echo "Updating ${BASE_SRC_PATH}/mod.rs ..."
echo "mod q${DAY};" >> "${BASE_SRC_PATH}/mod.rs"

echo "Done!"
