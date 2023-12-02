use std::{
    fs::File,
    io::{self, BufRead},
};

fn read_lines(file_path: &str) -> std::io::Result<Vec<String>> {
    let file: File = File::open(file_path).expect("Unable to open file");

    let reader: io::BufReader<File> = io::BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line: String = line.expect("Unable to read line");
        lines.push(line);
    }

    Ok(lines)
}

fn get_number_as_word_if_exists(word_to_check: &str, words: &[&str]) -> Option<usize> {
    words.iter().enumerate().find_map(|(index, &word)| {
        if word_to_check.contains(word) {
            Some(index + 1)
        } else {
            None
        }
    })
}

pub fn part_1(file_path: &str) -> std::io::Result<u32> {
    let lines: Vec<String> = read_lines(file_path).unwrap();
    let mut numbers: Vec<u32> = Vec::new();

    for line in lines.iter() {
        let first_number = line.chars().find(|&c| c.is_ascii_digit()).unwrap();

        let second_number = line.chars().rev().find(|&c| c.is_ascii_digit()).unwrap();

        let combined_num = format!("{}{}", first_number, second_number);
        let number = combined_num.parse::<u32>().unwrap();
        numbers.push(number);
    }

    // sum numbers
    let sum: u32 = numbers.iter().sum();

    Ok(sum)
}

pub fn part_2(file_path: &str) -> std::io::Result<u32> {
    let lines: Vec<String> = read_lines(file_path).unwrap();
    let mut numbers: Vec<u32> = Vec::new();
    let number_lookup = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for line in lines.iter() {
        let mut word_to_check = String::new();
        let mut first_number = 0;
        let mut second_number = 0;
        for c in line.chars() {
            if c.is_ascii_digit() {
                let number = c.to_digit(10).unwrap();
                first_number = number;
                break;
            }
            word_to_check.push(c);
            match get_number_as_word_if_exists(&word_to_check, &number_lookup) {
                Some(number) => {
                    first_number = number as u32;
                    break;
                }
                None => continue,
            }
        }

        word_to_check.clear();

        for c in line.chars().rev() {
            if c.is_ascii_digit() {
                let number = c.to_digit(10).unwrap();
                second_number = number;
                break;
            }
            word_to_check.insert(0, c);
            match get_number_as_word_if_exists(&word_to_check, &number_lookup) {
                Some(number) => {
                    second_number = number as u32;
                    break;
                }
                None => continue,
            }
        }

        let combined_num = format!("{}{}", first_number, second_number);
        let number = combined_num.parse::<u32>().unwrap();
        numbers.push(number);
    }

    let sum: u32 = numbers.iter().sum();

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    #[test]
    fn test_part_1_sample() {
        assert_eq!(142, part_1("input/rtob/q1-p1-sample.txt").unwrap());
    }

    #[test]
    fn test_part_1() {
        assert_eq!(54968, part_1("input/rtob/q1-p1-input.txt").unwrap());
    }

    #[test]
    fn test_part_2_sample() {
        assert_eq!(281, part_2("input/rtob/q1-p2-sample.txt").unwrap());
    }

    #[test]
    fn test_part_2() {
        assert_eq!(54094, part_2("input/rtob/q1-p2-input.txt").unwrap());
    }
}
