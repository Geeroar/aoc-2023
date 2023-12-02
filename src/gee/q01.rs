use crate::gee::parser::FileLines;

fn _part_1(input: &str) -> std::io::Result<u32> {
    let lines = FileLines::new(input)?;
    let mut result: u32 = 0;
    for line in lines {
        let i1 = line.find(|c: char| c.is_ascii_digit()).unwrap();
        let i2 = line.rfind(|c: char| c.is_ascii_digit()).unwrap();
        result += (u32::from(line.as_bytes()[i1]) - 48) * 10;
        result += u32::from(line.as_bytes()[i2]) - 48;
    }
    Ok(result)
}

fn _part_2(input: &str) -> std::io::Result<u32> {
    let lines = FileLines::new(input)?;
    let mut result: u32 = 0;
    for line in lines {
        let items = _match_indexes(line.as_str());
        result += items.first().unwrap().1 * 10 + items.last().unwrap().1;
    }
    Ok(result)
}

fn _match_indexes(s: &str) -> Vec<(usize, u32)> {
    let mut v = vec![];
    let pairs = [
        ("1", 1),
        ("one", 1),
        ("2", 2),
        ("two", 2),
        ("3", 3),
        ("three", 3),
        ("4", 4),
        ("four", 4),
        ("5", 5),
        ("five", 5),
        ("6", 6),
        ("six", 6),
        ("7", 7),
        ("seven", 7),
        ("8", 8),
        ("eight", 8),
        ("9", 9),
        ("nine", 9),
    ];
    for pair in pairs {
        s.match_indices(pair.0)
            .for_each(|(i, _)| v.push((i, pair.1)));
    }
    v.sort_by_key(|k| k.0);
    v
}

#[cfg(test)]
mod tests {
    use super::{_part_1, _part_2};

    const INPUT: &str = "input/gee/input01.txt";
    const INPUT_SAMPLE: &str = "input/gee/input01_sample.txt";
    const INPUT_SAMPLE2: &str = "input/gee/input01_sample2.txt";

    #[test]
    fn q01_part_1_sample() {
        let result = _part_1(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 142);
    }

    #[test]
    fn q01_part_1_main() {
        let result = _part_1(INPUT);
        assert_eq!(result.unwrap(), 57346);
    }

    #[test]
    fn q01_part_2_sample() {
        let result = _part_2(INPUT_SAMPLE2);
        assert_eq!(result.unwrap(), 281);
    }

    #[test]
    fn q01_part_2_main() {
        let result = _part_2(INPUT);
        assert_eq!(result.unwrap(), 57345);
    }
}
