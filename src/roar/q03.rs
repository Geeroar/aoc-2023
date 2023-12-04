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

fn _build_schematic(input_file: &str) -> std::io::Result<Vec<Vec<char>>> {
    let lines = FileLines::new(input_file)?;

    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in lines {
        let row: Vec<char> = line.chars().collect();
        matrix.push(row);
    }

    Ok(matrix)
}

fn _char_is_symbol(c: char) -> bool {
    !c.is_alphanumeric() && c != '.'
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
    for (i, row_chars) in schematic_matrix.iter().enumerate().take(end_row + 1).skip(start_row) {
        for j in start_col..end_col + 1 {
            if i == row && j == col {
                continue;
            }

            if _char_is_symbol(row_chars[j]) {
                return true;
            }
        }
    }
    false
}

fn _parse_number_at_index(row: &Vec<char>, col: usize) -> u32 {
    let mut number_str = row[col].to_string();
    let mut i = col;
    while i > 0 && row[i - 1].is_numeric() {
        number_str.insert(0, row[i - 1]);
        i -= 1;
    }
    i = col;
    while i < row.len() - 1 && row[i + 1].is_numeric() {
        number_str.push(row[i + 1]);
        i += 1;
    }
    number_str.parse::<u32>().unwrap()
}

fn _part_1(input_file: &str) -> std::io::Result<u32> {
    let schematic = _build_schematic(input_file).unwrap();
    let mut result: u32 = 0;
    let mut number_str: String = String::new();
    let mut number_is_valid: bool = false;
    for (row_index, row) in schematic.iter().enumerate() {
        for (col_index, col) in row.iter().enumerate() {
            if col.is_numeric() {
                number_str.push(*col);
                if !number_is_valid {
                    number_is_valid =
                        _check_number_has_adjacent_symbol(&schematic, row_index, col_index);
                }
            }

            if !col.is_numeric() || col_index == row.len() - 1 && !number_str.is_empty() {
                if number_is_valid {
                    result += number_str.parse::<u32>().unwrap();
                    number_is_valid = false;
                }
                number_str.clear();
            }
        }
    }
    Ok(result)
}

fn _part_2(input_file: &str) -> std::io::Result<u32> {
    let schematic = _build_schematic(input_file).unwrap();
    let mut result: u32 = 0;

    for (row_index, row) in schematic.iter().enumerate() {
        for (col_index, _) in row.iter().enumerate() {
            if schematic[row_index][col_index] == '*' {
                let mut joined_numbers: Vec<u32> = Vec::new();
                // scan above
                if row_index > 0 {
                    let i = row_index - 1;
                    if schematic[i][col_index].is_numeric() {
                        joined_numbers.push(_parse_number_at_index(&schematic[i], col_index));
                    } else {
                        if col_index > 0 {
                            let j = col_index - 1;
                            if schematic[i][j].is_numeric() {
                                joined_numbers.push(_parse_number_at_index(&schematic[i], j));
                            }
                        }
                        if col_index < schematic[0].len() - 1 {
                            let j = col_index + 1;
                            if schematic[i][j].is_numeric() {
                                joined_numbers.push(_parse_number_at_index(&schematic[i], j));
                            }
                        }
                    }
                }
                // scan below
                if row_index < schematic.len() - 1 {
                    let i = row_index + 1;
                    if schematic[i][col_index].is_numeric() {
                        joined_numbers.push(_parse_number_at_index(&schematic[i], col_index));
                    } else {
                        if col_index > 0 {
                            let j = col_index - 1;
                            if schematic[i][j].is_numeric() {
                                joined_numbers.push(_parse_number_at_index(&schematic[i], j));
                            }
                        }
                        if col_index < schematic[0].len() - 1 {
                            let j = col_index + 1;
                            if schematic[i][j].is_numeric() {
                                joined_numbers.push(_parse_number_at_index(&schematic[i], j));
                            }
                        }
                    }
                }
                // scan left
                if col_index > 0 {
                    let i = col_index - 1;
                    if schematic[row_index][i].is_numeric() {
                        joined_numbers.push(_parse_number_at_index(&schematic[row_index], i));
                    }
                }
                // scan right
                if col_index < schematic[0].len() - 1 {
                    let i = col_index + 1;
                    if schematic[row_index][i].is_numeric() {
                        joined_numbers.push(_parse_number_at_index(&schematic[row_index], i));
                    }
                }

                if joined_numbers.len() == 2 {
                    result += joined_numbers[0] * joined_numbers[1];
                }
            }
        }
    }

    Ok(result)
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
        assert_eq!(result.unwrap(), 467835);
    }

    #[test]
    fn roar_q03_p2_main() {
        let result = _part_2(INPUT);
        assert_eq!(result.unwrap(), 82301120);
    }
}
