#![allow(dead_code, unused_variables)]

use std::collections::{HashSet, VecDeque};

use crate::utils::parser::{parse, FileLines};

type Point = (i64, i64);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Galaxy {
    id: usize,
    location: Point,
}

const GALAXY_SYMBOL: char = '#';

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
struct Tile {
    symbol: char,
    weight: u32,
}

struct Input {
    grid: Vec<Vec<Tile>>,
}

fn print_grid(grid: &Vec<Vec<Tile>>) {
    for row in grid {
        for tile in row {
            print!("|{}:{}|", tile.symbol, tile.weight);
        }
        println!(); // Newline at the end of each row
    }
}

fn parse_grid(lines: FileLines) -> Vec<Vec<Tile>> {
    let grid: Vec<Vec<Tile>> = lines
        .map(|line| {
            let mut tiles = Vec::new();
            for symbol in line.chars() {
                tiles.push(Tile { symbol, weight: 1 });
            }
            tiles
        })
        .collect();

    return grid;
}

fn expand_grid(grid: Vec<Vec<Tile>>, expansion_magnitude: u32) -> Vec<Vec<Tile>> {
    let mut expanded_grid: Vec<Vec<Tile>> = grid.clone();

    let mut rows_to_expand = vec![false; expanded_grid.len()];
    let mut cols_to_expand = vec![false; expanded_grid[0].len()];

    for (row_index, row) in expanded_grid.iter().enumerate() {
        rows_to_expand[row_index] = row.iter().all(|x| x.symbol == '.');
    }

    for col_index in 0..expanded_grid[0].len() {
        cols_to_expand[col_index] = expanded_grid.iter().all(|row| row[col_index].symbol == '.');
    }

    for (row_index, row) in expanded_grid.iter_mut().enumerate() {
        for (col_index, tile) in row.iter_mut().enumerate() {
            if rows_to_expand[row_index] || cols_to_expand[col_index] {
                tile.weight = expansion_magnitude;
            }
        }
    }

    return expanded_grid;
}

fn get_galaxies(grid: &Vec<Vec<Tile>>) -> Vec<Galaxy> {
    let mut galaxies = Vec::new();
    for (row_index, row) in grid.iter().enumerate() {
        for (col, tile) in row.iter().enumerate() {
            if tile.symbol == GALAXY_SYMBOL {
                let point = (col as i64, row_index as i64); // x, y
                galaxies.push(Galaxy {
                    id: galaxies.len() + 1,
                    location: point,
                });
            }
        }
    }
    return galaxies;
}
fn calculate_shortest_path(grid: &Vec<Vec<Tile>>, start: Point, end: Point) -> i64 {
    // Standard bfs implementation
    let mut queue = VecDeque::new();
    // Setup grid, same size as input grid, but all false
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]; // Right, Down, Left, Up
    queue.push_back((start, 0));
    visited[start.1 as usize][start.0 as usize] = true;

    while let Some((current, dist)) = queue.pop_front() {
        if current == end {
            return dist;
        }

        for (dx, dy) in directions.iter() {
            let x = current.0 + dx;
            let y = current.1 + dy;
            if x >= 0 && x < grid[0].len() as i64
                && y >= 0 && y < grid.len() as i64
                && !visited[y as usize][x as usize]
            {
                // Using weight as distance
                queue.push_back(((x, y), dist + grid[y as usize][x as usize].weight as i64));
                visited[y as usize][x as usize] = true;
            }
        }
    }

    panic!("No path found. Should not happen!")
}

fn get_distance_betwixt_galaxies(grid: &Vec<Vec<Tile>>, expansion_magnitude: u32) -> i64 {
    let expanded_grid = expand_grid(grid.to_vec(), expansion_magnitude);
    let mut total = 0;
    let galaxies = get_galaxies(&expanded_grid);
    let mut seen = HashSet::new();

    for (i, galaxy) in galaxies.iter().enumerate() {
        for other_galaxy in galaxies.iter().skip(i + 1) {
            if seen.insert((galaxy, other_galaxy)) {
                let dist = calculate_shortest_path(&expanded_grid, galaxy.location, other_galaxy.location);
                println!(
                    "Distance between {:?} and {:?} is {}",
                    galaxy, other_galaxy, dist
                );
                total += dist;
            }
        }
    }

    total
}

impl TryFrom<FileLines> for Input {
    type Error = std::io::Error;

    fn try_from(lines: FileLines) -> Result<Self, Self::Error> {
        let grid = parse_grid(lines);

        Ok(Input { grid })
    }
}

fn part_1(input_file: &str) -> std::io::Result<i64> {
    let input = parse::<Input>(input_file)?;
    Ok(get_distance_betwixt_galaxies(&input.grid, 2))
}

fn part_2(input_file: &str) -> std::io::Result<i64> {
    let input = parse::<Input>(input_file)?;
    Ok(get_distance_betwixt_galaxies(&input.grid, 1000000))
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    const INPUT: &str = "input/roar/q11_input.txt";
    const INPUT_SAMPLE: &str = "input/roar/q11_sample.txt";

    #[test]
    fn roar_q11_p1_sample() {
        let result = part_1(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 374);
    }

    #[ignore]
    #[test]
    fn roar_q11_p1_main() {
        let result = part_1(INPUT);
        assert_eq!(result.unwrap(), 9608724);
    }

    #[test]
    fn roar_q11_p2_sample() {
        let result = part_2(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 82000210);
    }

    #[test]
    fn roar_q11_p2_main() {
        let result = part_2(INPUT);
        assert_eq!(result.unwrap(), 904633799472);
    }
}
