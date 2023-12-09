use crate::utils::parser::FileLines;

struct Input {
    _histories: Vec<Vec<i64>>,
}

impl TryFrom<FileLines> for Input {
    type Error = std::io::Error;

    fn try_from(_lines: FileLines) -> Result<Self, Self::Error> {
        let mut histories: Vec<Vec<i64>> = Vec::new();
        for line in _lines {
            // Split line by spaces into numbers which can be negative
            let numbers: Vec<i64> = line
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect();
            histories.push(numbers);
        }
        Ok(Input {
            _histories: histories,
        })
    }
}

fn _build_diffs_sequence(sequence: Vec<i64>) -> Vec<Vec<i64>> {
    let mut sequence = sequence;
    let mut all_diffs = Vec::new();
    all_diffs.push(sequence.clone());
    for _ in 0..sequence.len() - 2 {
        let mut current_diffs = Vec::new();
        for i in 0..sequence.len() - 1 {
            let diff = sequence[i + 1] - sequence[i];
            current_diffs.push(diff);
        }
        all_diffs.push(current_diffs.clone());
        sequence = current_diffs;

        if sequence.iter().all(|&x| x == 0) {
            break;
        }
    }

    return all_diffs;
}

fn _build_predictions(histories: Vec<Vec<i64>>) -> (i64, i64) {
    let mut prediction_sum = 0;
    let mut backwards_sum = 0;
    for history in histories {
        let all_diffs = _build_diffs_sequence(history.clone());
        let all_diffs_backwards = _build_diffs_sequence(history.iter().rev().cloned().collect());
        let mut prediction = 0;
        for diffs in all_diffs.iter().rev().skip(1) {
            prediction += diffs.last().unwrap();
        }
        prediction_sum += prediction;
        let mut backwards = 0;
        for diffs in all_diffs_backwards.iter().rev().skip(1) {
            backwards += diffs.last().unwrap();
        }
        backwards_sum += backwards;
    }
    (prediction_sum, backwards_sum)
}

fn _part_1(input_file: &str) -> std::io::Result<i64> {
    let input = Input::try_from(FileLines::new(input_file)?)?;
    Ok(_build_predictions(input._histories).0)
}

fn _part_2(input_file: &str) -> std::io::Result<i64> {
    let input = Input::try_from(FileLines::new(input_file)?)?;
    Ok(_build_predictions(input._histories).1)
}

#[cfg(test)]
mod tests {
    use super::{_part_1, _part_2};

    const INPUT: &str = "input/roar/q09_input.txt";
    const INPUT_SAMPLE: &str = "input/roar/q09_sample.txt";

    #[test]
    fn roar_q09_p1_sample() {
        let result = _part_1(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 114);
    }

    #[test]
    fn roar_q09_p1_main() {
        let result = _part_1(INPUT);
        assert_eq!(result.unwrap(), 1789635132);
    }

    #[test]
    fn roar_q09_p2_sample() {
        let result = _part_2(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 2);
    }

    #[test]
    fn roar_q09_p2_main() {
        let result = _part_2(INPUT);
        assert_eq!(result.unwrap(), 913);
    }
}
