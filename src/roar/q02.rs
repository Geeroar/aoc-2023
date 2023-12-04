use crate::utils::parser::FileLines;

struct Input {
    _value: u32,
}

const MAX_RED_CUBES: u32 = 12;
const MAX_GREEN_CUBES: u32 = 13;
const MAX_BLUE_CUBES: u32 = 14;

impl TryFrom<FileLines> for Input {
    type Error = std::io::Error;

    fn try_from(_lines: FileLines) -> Result<Self, Self::Error> {
        Ok(Input { _value: 0 })
    }
}

fn _part_1(input: &str) -> std::io::Result<u32> {
    let lines = FileLines::new(input)?;
    let mut result: u32 = 0;
    for line in lines {
        let game_input: Vec<&str> = line.split(':').collect();
        let game_name = game_input[0].trim();
        let game_name_parts: Vec<&str> = game_name.split_whitespace().collect();
        let game_id: u32 = game_name_parts[1].parse().unwrap();
        let game_cubes = game_input[1].trim();
        let game_cubes_sets: Vec<&str> = game_cubes.split(';').collect();

        let mut game_is_possible = true;

        for game_set in game_cubes_sets {
            let set_parts: Vec<&str> = game_set.split(',').collect();
            for set_part in set_parts {
                let game_cube_parts: Vec<&str> = set_part.split_whitespace().collect();
                let game_cube_count: u32 = game_cube_parts[0].parse().unwrap();
                let game_cube_color = game_cube_parts[1];
                match game_cube_color {
                    "red" => {
                        if game_cube_count > MAX_RED_CUBES {
                            game_is_possible = false;
                        }
                    }
                    "green" => {
                        if game_cube_count > MAX_GREEN_CUBES {
                            game_is_possible = false;
                        }
                    }
                    "blue" => {
                        if game_cube_count > MAX_BLUE_CUBES {
                            game_is_possible = false;
                        }
                    }
                    _ => {}
                }
            }
        }
        if game_is_possible {
            println!("Game {} is possible", game_id);
            result += game_id;
        }
    }
    Ok(result)
}

fn _part_2(input: &str) -> std::io::Result<u32> {
    let lines = FileLines::new(input)?;
    let mut result: u32 = 0;
    for line in lines {
        let game_input: Vec<&str> = line.split(':').collect();
        let game_cubes = game_input[1].trim();
        let game_cubes_sets: Vec<&str> = game_cubes.split(';').collect();

        let mut min_blue = 0;
        let mut min_green = 0;
        let mut min_red = 0;

        for game_set in game_cubes_sets {
            let set_parts: Vec<&str> = game_set.split(',').collect();
            for set_part in set_parts {
                let game_cube_parts: Vec<&str> = set_part.split_whitespace().collect();
                let game_cube_count: u32 = game_cube_parts[0].parse().unwrap();
                let game_cube_color = game_cube_parts[1];
                match game_cube_color {
                    "red" => {
                        if game_cube_count > min_red {
                            min_red = game_cube_count;
                        }
                    }
                    "green" => {
                        if game_cube_count > min_green {
                            min_green = game_cube_count;
                        }
                    }
                    "blue" => {
                        if game_cube_count > min_blue {
                            min_blue = game_cube_count;
                        }
                    }
                    _ => {}
                }
            }
        }
        result += min_blue * min_green * min_red;
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::{_part_1, _part_2};

    const INPUT: &str = "input/roar/q02_input.txt";
    const FIRST_INPUT_SAMPLE: &str = "input/roar/q02_p1_sample.txt";
    const SECOND_INPUT_SAMPLE: &str = "input/roar/q02_p2_sample.txt";

    #[test]
    fn roar_q02_p1_sample() {
        let result = _part_1(FIRST_INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 8);
    }

    #[test]
    fn roar_q02_p1_main() {
        let result = _part_1(INPUT);
        assert_eq!(result.unwrap(), 2164);
    }

    #[test]
    fn roar_q02_p2_sample() {
        let result = _part_2(SECOND_INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 2286);
    }

    #[test]
    fn roar_q02_p2_main() {
        let result = _part_2(INPUT);
        assert_eq!(result.unwrap(), 69929);
    }
}
