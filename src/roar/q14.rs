#![allow(dead_code, unused_variables)]

use crate::utils::{
    parser::{parse, FileLines},
    transposer::transpose_vec_of_strings,
};

struct Input {
    platform: Vec<String>,
}

impl TryFrom<FileLines> for Input {
    type Error = std::io::Error;

    fn try_from(lines: FileLines) -> Result<Self, Self::Error> {
        let mut platform = Vec::new();
        for line in lines {
            platform.push(line);
        }
        Ok(Input { platform })
    }
}

#[derive(PartialEq)]
enum TiltDirection {
    Left,
    Right,
}

fn tilt_row(row: String, direction: TiltDirection) -> String {
    /*
      This is a mess but what I'm trying to do is split the string by # so
      I can sort each segment by '0' and '.' and then join them back together.
    */
    let start_len = row.chars().take_while(|&c| c == '#').count();
    let start = &row[..start_len];
    let end_len = row.chars().rev().take_while(|&c| c == '#').count();
    let end = &row[row.len() - end_len..];
    let middle_start = start_len;
    let middle_end = row.len() - end_len;
    let middle = &row[middle_start..middle_end];
    let sections: Vec<&str> = middle.split('#').collect();
    let mut sorted_sections: Vec<String> = Vec::new();
    for section in sections {
        let mut sorted_section = section.chars().collect::<Vec<char>>();
        if direction == TiltDirection::Left {
            // The string only has the character . and O - O should move to the front
            sorted_section.sort_by(|a, b| {
                if *a == *b {
                    std::cmp::Ordering::Equal
                } else if *a == 'O' {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            });
        } else {
            // The string only has the character . and O - O should move to the back
            sorted_section.sort_by(|a, b| {
                if *a == *b {
                    std::cmp::Ordering::Equal
                } else if *a == 'O' {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Less
                }
            });
        }

        sorted_sections.push(sorted_section.into_iter().collect());
    }

    format!("{}{}{}", start, sorted_sections.join("#"), end)
}


fn part_1(input_file: &str) -> std::io::Result<usize> {
    let input: Input = parse(input_file)?;
    let mut transposed_platform = transpose_vec_of_strings(input.platform);
    for row in transposed_platform.iter_mut() {
        *row = tilt_row(row.to_string(), TiltDirection::Left);
    }
    let tilted_platform = transpose_vec_of_strings(transposed_platform);
    let mut total = 0;
    for (i, row) in tilted_platform.iter().enumerate() {
        let value = tilted_platform.len() - i;
        println!("Row value: {}", value);
        // count instances of O in string
        let count = row.chars().filter(|&c| c == 'O').count();
        total += count * value;
    }
    Ok(total)
}

fn part_2(input_file: &str) -> std::io::Result<usize> {
    let input: Input = parse(input_file)?;
    let mut total = 0;
    let mut tilted_platform = Vec::new();
    for _ in 0..1000000000 {
        let platform = input.platform.clone();
        let mut transposed_platform = transpose_vec_of_strings(platform);
        for row in transposed_platform.iter_mut() {
            *row = tilt_row(row.to_string(), TiltDirection::Left);
        }
        let mut tilted_north = transpose_vec_of_strings(transposed_platform);
        for row in tilted_north.iter_mut() {
            *row = tilt_row(row.to_string(), TiltDirection::Left);
        }
        let mut tilted_west = transpose_vec_of_strings(tilted_north);
        for row in tilted_west.iter_mut() {
            *row = tilt_row(row.to_string(), TiltDirection::Right);
        }
        let mut tilted_south = transpose_vec_of_strings(tilted_west);
        for row in tilted_south.iter_mut() {
            *row = tilt_row(row.to_string(), TiltDirection::Right);
        }
        let tilted_east = transpose_vec_of_strings(tilted_south);

        tilted_platform = transpose_vec_of_strings(tilted_east);


    }
    for (i, row) in tilted_platform.iter().enumerate() {
        let value = tilted_platform.len() - i;
        println!("Row value: {}", value);
        // count instances of O in string
        let count = row.chars().filter(|&c| c == 'O').count();
        total += count * value;
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    const INPUT: &str = "input/roar/q14_input.txt";
    const INPUT_SAMPLE: &str = "input/roar/q14_sample.txt";

    #[test]
    fn roar_q14_p1_sample() {
        let result = part_1(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 136);
    }

    #[test]
    fn roar_q14_p1_main() {
        let result = part_1(INPUT);
        assert_eq!(result.unwrap(), 108840);
    }

    #[ignore = "doesn't work"]
    #[test]
    fn roar_q14_p2_sample() {
        let result = part_2(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 64);
    }

    #[ignore = "doesn't work"]
    #[test]
    fn roar_q14_p2_main() {
        let result = part_2(INPUT);
        assert_eq!(result.unwrap(), 0);
    }
}
