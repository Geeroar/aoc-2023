#![allow(dead_code, unused_variables)]
use crate::utils::parser::{parse, FileLines};
use rayon::prelude::*;

#[derive(PartialEq, Debug, Clone, Copy)]
enum TileType {
    Empty,
    RightAngleMirror,
    LeftAngleMirror,
    HorizontalSplitter,
    VerticalSplitter,
}

#[derive(PartialEq, Debug, Clone, Copy)]
struct Tile {
    x: usize,
    y: usize,
    tile_type: TileType,
    value: char,
    is_energised: bool,
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Debug, Clone, Copy)]
struct Beam {
    x: usize,
    y: usize,
    direction: Direction,
}

struct Input {
    grid: Vec<Vec<Tile>>,
}

impl TryFrom<FileLines> for Input {
    type Error = std::io::Error;

    fn try_from(lines: FileLines) -> Result<Self, Self::Error> {
        let mut grid = Vec::new();

        for (y, line) in lines.enumerate() {
            let mut row = Vec::new();
            for (x, character) in line.chars().enumerate() {
                let tile_type = match character {
                    '.' => TileType::Empty,
                    '/' => TileType::RightAngleMirror,
                    '\\' => TileType::LeftAngleMirror,
                    '-' => TileType::HorizontalSplitter,
                    '|' => TileType::VerticalSplitter,
                    _ => panic!("Invalid character"),
                };
                row.push(Tile {
                    tile_type,
                    is_energised: false,
                    value: character,
                    x,
                    y,
                });
            }
            grid.push(row);
        }

        Ok(Input { grid })
    }
}

fn count_energised_tiles_for_beam(grid: &Vec<Vec<Tile>>, beam: Beam) -> usize {
    let grid = &mut grid.clone();
    let width = grid[0].len();
    let height = grid.len();
    let mut beams = vec![beam];

    let mut counter = 100000;

    while !beams.is_empty() && counter > 0 {
        counter -= 1;
        for i in 0..beams.len() {
            let x = beams[i].x;
            let y = beams[i].y;
            let direction = beams[i].direction;

            // Energise current tile
            let tile = &mut grid[y][x];
            tile.is_energised = true;

            // Check action on tile
            match tile.tile_type {
                TileType::HorizontalSplitter => {
                    if direction == Direction::Up || direction == Direction::Down {
                        beams.remove(i);
                        if x > 0 {
                            let left_beam = Beam {
                                x: x - 1,
                                y,
                                direction: Direction::Left,
                            };
                            beams.push(left_beam);
                        }
                        if x < width - 1 {
                            let right_beam = Beam {
                                x: x + 1,
                                y,
                                direction: Direction::Right,
                            };
                            beams.push(right_beam);
                        }

                        break;
                    }
                }
                TileType::VerticalSplitter => {
                    if direction == Direction::Left || direction == Direction::Right {
                        beams.remove(i);
                        if y > 0 {
                            let up_beam = Beam {
                                x,
                                y: y - 1,
                                direction: Direction::Up,
                            };
                            beams.push(up_beam);
                        }
                        if y < height - 1 {
                            let down_beam = Beam {
                                x,
                                y: y + 1,
                                direction: Direction::Down,
                            };
                            beams.push(down_beam);
                        }
                        break;
                    }
                }
                TileType::RightAngleMirror => {
                    if direction == Direction::Up {
                        beams[i].direction = Direction::Right;
                    } else if direction == Direction::Left {
                        beams[i].direction = Direction::Down;
                    } else if direction == Direction::Down {
                        beams[i].direction = Direction::Left;
                    } else if direction == Direction::Right {
                        beams[i].direction = Direction::Up;
                    }
                }
                TileType::LeftAngleMirror => {
                    if direction == Direction::Up {
                        beams[i].direction = Direction::Left;
                    } else if direction == Direction::Left {
                        beams[i].direction = Direction::Up;
                    } else if direction == Direction::Down {
                        beams[i].direction = Direction::Right;
                    } else if direction == Direction::Right {
                        beams[i].direction = Direction::Down;
                    }
                }
                _ => {}
            }

            // Move
            match beams[i].direction {
                Direction::Up => {
                    if beams[i].y == 0 {
                        beams.remove(i);
                        break;
                    }
                    beams[i].y -= 1;
                }
                Direction::Down => {
                    if beams[i].y == height - 1 {
                        beams.remove(i);
                        break;
                    }
                    beams[i].y += 1;
                }
                Direction::Left => {
                    if beams[i].x == 0 {
                        beams.remove(i);
                        break;
                    }
                    beams[i].x -= 1;
                }
                Direction::Right => {
                    if beams[i].x == width - 1 {
                        beams.remove(i);
                        break;
                    }
                    beams[i].x += 1;
                }
            }
        }
    }

    // Print grid
    // for row in grid {
    //     for tile in row {
    //         if tile.is_energised {
    //             print!("#");
    //         } else {
    //             print!("{}", tile.value);
    //         }
    //     }
    //     println!();
    // }
    // Count all tiles in grid which are energised
    let mut count = 0;
    for row in grid {
        for tile in row {
            if tile.is_energised {
                count += 1;
            }
        }
    }
    count
}

fn part_1(input_file: &str) -> std::io::Result<usize> {
    let input: Input = parse(input_file)?;
    let count = count_energised_tiles_for_beam(
        &input.grid,
        Beam {
            x: 0,
            y: 0,
            direction: Direction::Right,
        },
    );

    Ok(count)
}

fn part_2(input_file: &str) -> std::io::Result<usize> {
    let input: Input = parse(input_file)?;
    let mut start_beams = vec![];
    for row in 0..input.grid.len() {
        start_beams.push((
            Beam {
                x: 0,
                y: row,
                direction: Direction::Right,
            },
            input.grid.clone(),
        ));
        start_beams.push((
            Beam {
                x: input.grid[0].len() - 1,
                y: row,
                direction: Direction::Left,
            },
            input.grid.clone(),
        ));
    }
    for col in 0..input.grid[0].len() {
        start_beams.push((
            Beam {
                x: col,
                y: 0,
                direction: Direction::Down,
            },
            input.grid.clone(),
        ));
        start_beams.push((
            Beam {
                x: col,
                y: input.grid.len() - 1,
                direction: Direction::Up,
            },
            input.grid.clone(),
        ));
    }
    for beam_grid in &mut start_beams {
        println!(
            "beam_grid: (col: {}, row: {}) {:?}",
            beam_grid.0.x, beam_grid.0.y, beam_grid.0.direction
        );
    }

    let results: Vec<_> = start_beams
        .par_iter()
        .map(|beam_grid| count_energised_tiles_for_beam(&beam_grid.1, beam_grid.0))
        .collect();

    // print all results
    for result in &results {
        println!("{}", result);
    }
    Ok(*results.iter().max().unwrap())
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    const INPUT: &str = "input/roar/q16_input.txt";
    const INPUT_SAMPLE: &str = "input/roar/q16_sample.txt";

    #[test]
    fn roar_q16_p1_sample() {
        let result = part_1(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 46);
    }

    #[ignore = "too long"]
    #[test]
    fn roar_q16_p1_main() {
        let result = part_1(INPUT);
        assert_eq!(result.unwrap(), 7067);
    }

    #[test]
    fn roar_q16_p2_sample() {
        let result = part_2(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 51);
    }

    #[ignore = "too long"]
    #[test]
    fn roar_q16_p2_main() {
        let result = part_2(INPUT);
        assert_eq!(result.unwrap(), 0);
    }
}
