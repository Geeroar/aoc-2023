use crate::utils::parser::FileLines;

struct Input {
    _games: Vec<Game>,
}

struct Game {
    _id: u32,
    _views: Vec<CubeSet>,
}

struct CubeSet {
    _red: u32,
    _green: u32,
    _blue: u32,
}

impl TryFrom<FileLines> for Input {
    type Error = std::io::Error;

    fn try_from(_lines: FileLines) -> Result<Self, Self::Error> {
        let mut games: Vec<Game> = Vec::new();
        for line in _lines {
            if let Some((id, view_line)) = line.split_once(": ") {
                let mut _views: Vec<CubeSet> = Vec::new();
                for view in view_line.split("; ") {
                    let mut _red = 0;
                    let mut _green = 0;
                    let mut _blue = 0;
                    for color in view.split(", ") {
                        if color.ends_with(" red") {
                            _red = color.strip_suffix(" red").unwrap().parse().unwrap();
                        } else if color.ends_with(" green") {
                            _green = color.strip_suffix(" green").unwrap().parse().unwrap();
                        } else if color.ends_with(" blue") {
                            _blue = color.strip_suffix(" blue").unwrap().parse().unwrap();
                        }
                    }
                    _views.push(CubeSet {
                        _red,
                        _green,
                        _blue,
                    });
                }
                games.push(Game {
                    _id: id.strip_prefix("Game ").unwrap().parse().unwrap(),
                    _views,
                })
            }
        }
        Ok(Input { _games: games })
    }
}

fn _is_possible(limits: &CubeSet, game: &Game) -> bool {
    game._views
        .iter()
        .all(|g| g._red <= limits._red && g._green <= limits._green && g._blue <= limits._blue)
}

fn _min_cubes(game: &Game) -> CubeSet {
    let mut _red = 0;
    let mut _green = 0;
    let mut _blue = 0;
    for view in &game._views {
        _red = std::cmp::max(view._red, _red);
        _green = std::cmp::max(view._green, _green);
        _blue = std::cmp::max(view._blue, _blue);
    }
    CubeSet { _red, _green, _blue, }
}

fn _power(cube_set: &CubeSet) -> u32 {
    cube_set._red * cube_set._green * cube_set._blue
}

fn _part_1(input: &str) -> std::io::Result<u32> {
    let input = Input::try_from(FileLines::new(input)?)?;
    const LIMITS: CubeSet = CubeSet {
        _red: 12,
        _green: 13,
        _blue: 14,
    };
    Ok(input
        ._games
        .iter()
        .filter(|&g| _is_possible(&LIMITS, g))
        .map(|g| g._id)
        .sum())
}

fn _part_2(input: &str) -> std::io::Result<u32> {
    let input = Input::try_from(FileLines::new(input)?)?;
    Ok(input
        ._games
        .iter()
        .map(|g| _min_cubes(g))
        .map(|c| _power(&c))
        .sum())
}

#[cfg(test)]
mod tests {
    use super::{_part_1, _part_2};

    const INPUT: &str = "input/gee/q02_input.txt";
    const INPUT_SAMPLE: &str = "input/gee/q02_sample.txt";

    #[test]
    fn gee_q02_p1_sample() {
        let result = _part_1(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 8);
    }

    #[test]
    fn gee_q02_p1_main() {
        let result = _part_1(INPUT);
        assert_eq!(result.unwrap(), 2105);
    }

    #[test]
    fn gee_q02_p2_sample() {
        let result = _part_2(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 2286);
    }

    #[test]
    fn gee_q02_p2_main() {
        let result = _part_2(INPUT);
        assert_eq!(result.unwrap(), 72422);
    }
}
