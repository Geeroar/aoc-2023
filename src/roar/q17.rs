#![allow(dead_code, unused_variables)]
use crate::utils::parser::{parse, FileLines};
use std::{
    collections::{BinaryHeap, HashMap},
    isize,
};

const MAX_CONSECUTIVE_STEPS: usize = 3;

struct Block {
    heat_loss: usize,
}

// Basically a graph
struct City {
    blocks: Vec<Vec<Block>>,
}

struct Input {
    city: City,
}

impl TryFrom<FileLines> for Input {
    type Error = std::io::Error;

    fn try_from(lines: FileLines) -> Result<Self, Self::Error> {
        let mut blocks = Vec::new();
        for line in lines {
            let mut row = Vec::new();
            for character in line.chars() {
                let heat_loss = character.to_digit(10).unwrap() as usize;
                row.push(Block { heat_loss });
            }
            blocks.push(row);
        }
        Ok(Input {
            city: City { blocks },
        })
    }
}

struct State {
    position: (usize, usize), // (row, column)
    direction: isize,
    steps: isize,
    distance: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .distance
            .cmp(&self.distance)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for State {}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance && self.position == other.position
    }
}

fn check_not_reversing(new_direction: isize, direction: isize) -> bool {
    (new_direction + 2) % 4 != direction
}

fn validate_part_1(new_steps: isize, _: isize, direction: isize, new_direction: isize) -> bool {
    new_steps <= 3 && check_not_reversing(new_direction, direction)
}

fn validate_part_2(new_steps: isize, steps: isize, direction: isize, new_direction: isize) -> bool {
    new_steps <= 10
        && (new_direction == direction || steps >= 4 || steps == -1)
        && check_not_reversing(new_direction, direction)
}

fn calculate_least_heat_loss(
    city: &City,
    validate_move: fn(isize, isize, isize, isize) -> bool,
) -> usize {
    let mut queue = BinaryHeap::new();
    queue.push(State {
        position: (0, 0),
        direction: -1,
        steps: -1,
        distance: 0,
    });

    let mut distances: HashMap<(usize, usize, isize, isize), usize> = HashMap::new();
    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    // https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm
    while let Some(State {
        position,
        direction,
        steps,
        distance,
    }) = queue.pop()
    {
        if distances.contains_key(&(position.0, position.1, direction, steps)) {
            continue;
        }

        distances.insert((position.0, position.1, direction, steps), distance);

        for (i, &(row_moves, colum_moves)) in directions.iter().enumerate() {
            let new_position = (
                position.0 as isize + row_moves,
                position.1 as isize + colum_moves,
            );
            if new_position.0 >= 0
                && new_position.0 < city.blocks.len() as isize
                && new_position.1 >= 0
                && new_position.1 < city.blocks[0].len() as isize
            {
                let new_direction = i as isize;
                let new_steps = if new_direction != direction {
                    1
                } else {
                    steps + 1
                };
                if validate_move(new_steps, steps, direction, new_direction) {
                    let cost =
                        city.blocks[new_position.0 as usize][new_position.1 as usize].heat_loss;
                    queue.push(State {
                        position: (new_position.0 as usize, new_position.1 as usize),
                        direction: new_direction,
                        steps: new_steps,
                        distance: distance + cost,
                    });
                }
            }
        }
    }

    distances
        .iter()
        .filter(|&(&(r, c, _, _), _)| r == city.blocks.len() - 1 && c == city.blocks[0].len() - 1)
        .map(|(_, &v)| v)
        .min()
        .unwrap_or(usize::MAX)
}

fn part_1(input_file: &str) -> std::io::Result<usize> {
    let input: Input = parse(input_file)?;
    Ok(calculate_least_heat_loss(&input.city, validate_part_1))
}

fn part_2(input_file: &str) -> std::io::Result<usize> {
    let input: Input = parse(input_file)?;
    Ok(calculate_least_heat_loss(&input.city, validate_part_2))
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    const INPUT: &str = "input/roar/q17_input.txt";
    const INPUT_SAMPLE: &str = "input/roar/q17_sample.txt";

    #[test]
    fn roar_q17_p1_sample() {
        let result = part_1(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 102);
    }

    #[test]
    fn roar_q17_p1_main() {
        let result = part_1(INPUT);
        assert_eq!(result.unwrap(), 665);
    }

    #[test]
    fn roar_q17_p2_sample() {
        let result = part_2(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 94);
    }

    #[test]
    fn roar_q17_p2_main() {
        let result = part_2(INPUT);
        assert_eq!(result.unwrap(), 809);
    }
}
