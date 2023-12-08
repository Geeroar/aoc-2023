use crate::utils::parser::FileLines;

#[derive(Debug)]
struct Input {
    _races: Vec<Race>,
}

#[derive(Debug)]
struct Race {
    _time: u64,
    _distance: u64,
}

impl TryFrom<FileLines> for Input {
    type Error = std::io::Error;

    fn try_from(mut _lines: FileLines) -> Result<Self, Self::Error> {
        let time = _lines.next().unwrap();
        let distance = _lines.next().unwrap();
        let _races = time
            .strip_prefix("Time:")
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .zip(
                distance
                    .strip_prefix("Distance:")
                    .unwrap()
                    .split_whitespace()
                    .map(|s| s.parse::<u64>().unwrap()),
            )
            .map(|(t, d)| Race {
                _time: t,
                _distance: d,
            })
            .collect();

        Ok(Input { _races })
    }
}

impl Input {
    fn _combined_race(&self) -> Race {
        let mut time = String::new();
        let mut distance = String::new();

        for i in 0..self._races.len() {
            time.push_str(format!("{}", self._races[i]._time).as_str());
            distance.push_str(format!("{}", self._races[i]._distance).as_str());
        }
        Race {
            _time: time.parse().unwrap(),
            _distance: distance.parse().unwrap(),
        }
    }
}

impl Race {
    fn _ways_to_win(&self) -> u64 {
        let mut count = 0;
        for i in 1..self._time {
            if (self._time - i) * i > self._distance {
                count += 1;
            }
        }
        count
    }
}

fn _part_1(input_file: &str) -> std::io::Result<u64> {
    let input = Input::try_from(FileLines::new(input_file)?)?;
    Ok(input._races.iter().map(|r| r._ways_to_win()).product())
}

fn _part_2(input_file: &str) -> std::io::Result<u64> {
    let input = Input::try_from(FileLines::new(input_file)?)?;
    Ok(input._combined_race()._ways_to_win())
}

#[cfg(test)]
mod tests {
    use super::{_part_1, _part_2};

    const INPUT: &str = "input/gee/q06_input.txt";
    const INPUT_SAMPLE: &str = "input/gee/q06_sample.txt";

    #[test]
    fn gee_q06_p1_sample() {
        let result = _part_1(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 288);
    }

    #[test]
    fn gee_q06_p1_main() {
        let result = _part_1(INPUT);
        assert_eq!(result.unwrap(), 861300);
    }

    #[test]
    fn gee_q06_p2_sample() {
        let result = _part_2(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 71503);
    }

    #[test]
    fn gee_q06_p2_main() {
        let result = _part_2(INPUT);
        assert_eq!(result.unwrap(), 28101347);
    }
}
