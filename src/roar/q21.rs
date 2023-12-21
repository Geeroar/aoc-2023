#![allow(dead_code, unused_variables)]

use std::collections::{HashSet, VecDeque};

use crate::utils::parser::{parse, FileLines};

#[derive(PartialEq, Eq, Hash)]
struct Point {
    row: usize,
    column: usize,
}

#[derive(PartialEq)]
enum PlotType {
    GARDEN,
    ROCKS,
}

#[derive(PartialEq)]
struct GardenPlot {
    plot_type: PlotType,
}

struct Input {
    starting_position: Point,
    garden: Vec<Vec<GardenPlot>>,
}

impl TryFrom<FileLines> for Input {
    type Error = std::io::Error;

    fn try_from(lines: FileLines) -> Result<Self, Self::Error> {
        let mut starting_position = Point { row: 0, column: 0 };
        let mut garden = Vec::new();
        for line in lines {
            let mut row = Vec::new();
            for character in line.chars() {
                let plot_type = match character {
                    '.' => PlotType::GARDEN,
                    '#' => PlotType::ROCKS,
                    'S' => {
                        starting_position = Point {
                            row: garden.len(),
                            column: row.len(),
                        };
                        PlotType::GARDEN
                    }
                    _ => panic!("Invalid character in input"),
                };
                row.push(GardenPlot { plot_type });
            }
            garden.push(row);
        }
        Ok(Input {
            starting_position,
            garden,
        })
    }
}

fn get_reachable_plots(
    garden_map: &Vec<Vec<GardenPlot>>,
    start: (usize, usize),
    steps: usize,
) -> HashSet<Point> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut points = HashSet::new();

    queue.push_back((start, 0));
    visited.insert((start, 0));

    while let Some((position, step)) = queue.pop_front() {
        if step == steps {
            points.insert(Point {
                row: position.0,
                column: position.1,
            });
            continue;
        }

        for neighbor in get_neighbors(position, garden_map) {
            let next_step = step + 1;
            if !visited.contains(&(neighbor, next_step)) && next_step <= steps {
                visited.insert((neighbor, next_step));
                queue.push_back((neighbor, next_step));
            }
        }
    }
    points
}

fn get_neighbors(
    position: (usize, usize),
    garden_map: &Vec<Vec<GardenPlot>>,
) -> Vec<(usize, usize)> {
    let (x, y) = position;
    let mut neighbors = Vec::new();

    if x > 0 && garden_map[x - 1][y].plot_type != PlotType::ROCKS {
        neighbors.push((x - 1, y));
    }
    if x < garden_map.len() - 1 && garden_map[x + 1][y].plot_type != PlotType::ROCKS {
        neighbors.push((x + 1, y));
    }
    if y > 0 && garden_map[x][y - 1].plot_type != PlotType::ROCKS {
        neighbors.push((x, y - 1));
    }
    if y < garden_map[0].len() - 1 && garden_map[x][y + 1].plot_type != PlotType::ROCKS {
        neighbors.push((x, y + 1));
    }

    neighbors
}

fn part_1(steps: usize, input_file: &str) -> std::io::Result<usize> {
    let input: Input = parse(input_file)?;
    let points = get_reachable_plots(
        &input.garden,
        (input.starting_position.row, input.starting_position.column),
        steps,
    );

    // print garden
    for (i, row) in input.garden.iter().enumerate() {
        for (j, plot) in row.iter().enumerate() {
            if points.contains(&Point { row: i, column: j }) {
                print!("O");
            } else {
                match plot.plot_type {
                    PlotType::GARDEN => print!("."),
                    PlotType::ROCKS => print!("#"),
                }
            }
        }
        println!();
    }

    Ok(points.len())
}

fn part_2(input_file: &str) -> std::io::Result<usize> {
    let input: Input = parse(input_file)?;
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    const INPUT: &str = "input/roar/q21_input.txt";
    const INPUT_SAMPLE: &str = "input/roar/q21_sample.txt";

    #[test]
    fn roar_q21_p1_sample() {
        let result = part_1(6, INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 16);
    }

    #[test]
    fn roar_q21_p1_main() {
        let result = part_1(64, INPUT);
        assert_eq!(result.unwrap(), 3658);
    }

    #[test]
    fn roar_q21_p2_sample() {
        let result = part_2(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn roar_q21_p2_main() {
        let result = part_2(INPUT);
        assert_eq!(result.unwrap(), 0);
    }
}
