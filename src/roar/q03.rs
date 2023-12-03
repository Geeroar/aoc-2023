use crate::utils::parser::FileLines;
use std::{
    fs::File,
    io::{self, BufRead},
};

struct Input {
    _value: u32,
}

impl TryFrom<FileLines> for Input {
    type Error = std::io::Error;

    fn try_from(_lines: FileLines) -> Result<Self, Self::Error> {
        Ok(Input { _value: 0 })
    }
}

fn _build_schematic(input_file: &str) -> std::io::Result<Vec<Vec<char>>> {
    let lines = FileLines::new(input_file)?;

    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in lines {
        let row: Vec<char> = line.chars().collect();
        matrix.push(row);
    }

    return Ok(matrix);
}

fn _char_is_symbol(c: char) -> bool {
    return !c.is_alphanumeric() && c != '.';
}

fn _check_number_has_adjacent_symbol(
    schematic_matrix: &Vec<Vec<char>>,
    row: usize,
    col: usize,
) -> bool {
    let start_row = if row > 0 { row - 1 } else { row };
    let end_row = if row < schematic_matrix.len() - 1 {
        row + 1
    } else {
        row
    };
    let start_col = if col > 0 { col - 1 } else { col };
    let end_col = if col < schematic_matrix[0].len() - 1 {
        col + 1
    } else {
        col
    };
    for i in start_row..end_row + 1 {
        for j in start_col..end_col + 1 {
            if i == row && j == col {
                continue;
            }

            if _char_is_symbol(schematic_matrix[i][j]) {
                return true;
            }
        }
    }
    return false;
}

fn _part_1(input_file: &str) -> std::io::Result<u32> {
    let schematic = _build_schematic(input_file).unwrap();
    let mut result: u32 = 0;
    let mut number_str: String = String::new();
    let mut number_is_valid: bool = false;
    for (i, row) in schematic.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            if val.is_numeric() {
                number_str.push(*val);
                if !number_is_valid {
                    number_is_valid = _check_number_has_adjacent_symbol(&schematic, i, j);
                }
            }

            if !val.is_numeric() || j == row.len() - 1 {
                if !number_str.is_empty() {
                    println!("{}: {}", number_str, number_is_valid);
                    if number_is_valid {
                        result += number_str.parse::<u32>().unwrap();
                        number_is_valid = false;
                    }
                    number_str.clear();
                }
            }

        }
    }
    Ok(result)
}

fn _part_2(input_file: &str) -> std::io::Result<u32> {
    let input = Input::try_from(FileLines::new(input_file)?)?;
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::{_part_1, _part_2};

    const INPUT: &str = "input/roar/q03_input.txt";
    const INPUT_SAMPLE: &str = "input/roar/q03_sample.txt";

    #[test]
    fn roar_q03_p1_sample() {
        let result = _part_1(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 4361);
    }

    #[test]
    fn roar_q03_p1_main() {
        let result = _part_1(INPUT);
        assert_eq!(result.unwrap(), 532331);
    }

    #[test]
    fn roar_q03_p2_sample() {
        let result = _part_2(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn roar_q03_p2_main() {
        let result = _part_2(INPUT);
        assert_eq!(result.unwrap(), 0);
    }
}
