use crate::utils::parser::FileLines;

struct Input {
    _races: Vec<(u64, u64)>,
    _real_race: (u128, u128),
}

impl TryFrom<FileLines> for Input {
    type Error = std::io::Error;

    fn try_from(_lines: FileLines) -> Result<Self, Self::Error> {
        let mut races = Vec::new();
        let mut times = Vec::new();
        let mut distances = Vec::new();
        let mut real_race = (0, 0);

        for (i, line) in _lines.enumerate() {
            if i == 0 {
                times = line
                    .split_once(": ")
                    .unwrap()
                    .1
                    .split_whitespace()
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect();
                let full_time: String = line
                    .split_once(": ")
                    .unwrap()
                    .1
                    .split_whitespace()
                    .collect();
                real_race.0 = full_time.parse::<u128>().unwrap();
            }

            if i == 1 {
                distances = line
                    .split_once(": ")
                    .unwrap()
                    .1
                    .split_whitespace()
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect();
                let full_distance: String = line
                    .split_once(": ")
                    .unwrap()
                    .1
                    .split_whitespace()
                    .collect();
                real_race.1 = full_distance.parse::<u128>().unwrap();
            }
        }

        for (i, time) in times.iter().enumerate() {
            races.push((*time, distances[i]));
        }

        Ok(Input {
            _races: races,
            _real_race: real_race,
        })
    }
}

fn _part_1(input_file: &str) -> std::io::Result<u32> {
    let input = Input::try_from(FileLines::new(input_file)?)?;
    println!("{:?}", input._races);
    let mut total = 1;
    for (time, distance) in input._races {
        let mut ways_to_win = 0;
        // go from 1 to time - 1 so we always move within the time limit
        for speed in 1..=time - 1 {
            // speed = mps as it increases for each second
            // if speed * time left < distance this is a possible win
            let time_left = time - speed;
            let possible_distance = speed * time_left;

            if possible_distance > distance {
                println!("winning speed: {}", speed);
                ways_to_win += 1;
            }
        }
        println!("ways to win {}", ways_to_win);
        total *= ways_to_win;
        println!("total: {}", total);
    }
    Ok(total)
}

fn _part_2(input_file: &str) -> std::io::Result<u32> {
    let input = Input::try_from(FileLines::new(input_file)?)?;

    println!("{}, {}", input._real_race.0, input._real_race.1);
    let mut total = 1;
    let mut ways_to_win = 0;
    let time = input._real_race.0;
    let distance = input._real_race.1;
    // go from 1 to time - 1 so we always move within the time limit
    for speed in 1..=time - 1 {
        // speed = mps as it increases for each second
        // if speed * time left < distance this is a possible win
        let time_left = time - speed;
        let possible_distance = speed * time_left;

        if possible_distance > distance {
            ways_to_win += 1;
        }
    }
    println!("ways to win {}", ways_to_win);
    total *= ways_to_win;
    println!("total: {}", total);
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::{_part_1, _part_2};

    const INPUT: &str = "input/roar/q06_input.txt";
    const INPUT_SAMPLE: &str = "input/roar/q06_sample.txt";

    #[test]
    fn roar_q06_p1_sample() {
        let result = _part_1(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 288);
    }

    #[test]
    fn roar_q06_p1_main() {
        let result = _part_1(INPUT);
        assert_eq!(result.unwrap(), 293046);
    }

    #[test]
    fn roar_q06_p2_sample() {
        let result = _part_2(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 71503);
    }

    #[test]
    fn roar_q06_p2_main() {
        let result = _part_2(INPUT);
        assert_eq!(result.unwrap(), 35150181);
    }
}
