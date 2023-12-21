#![allow(dead_code, unused_variables)]

use std::collections::HashSet;

use crate::utils::parser::{parse, FileLines};

#[derive(Debug)]
struct Instruction {
    direction: char,
    distance: i128,
    color: String,
}

type Grid = Vec<Vec<String>>;

struct Input {
    instructions: Vec<Instruction>,
    true_instructions: Vec<Instruction>,
    grid_size: (usize, usize, (usize, usize)),
}

fn calculate_grid_size(instructions: &[Instruction]) -> (usize, usize, (usize, usize)) {
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;
    let mut current_x = 0;
    let mut current_y = 0;

    for instruction in instructions {
        match instruction.direction {
            'U' => current_y -= instruction.distance,
            'D' => current_y += instruction.distance,
            'L' => current_x -= instruction.distance,
            'R' => current_x += instruction.distance,
            _ => {}
        }
        min_x = min_x.min(current_x);
        max_x = max_x.max(current_x);
        min_y = min_y.min(current_y);
        max_y = max_y.max(current_y);
    }

    // Adjust the grid size to accommodate the path
    let grid_width = (max_x - min_x).abs() as usize + 1;
    let grid_height = (max_y - min_y).abs() as usize + 1;

    let start_x = if min_x < 0 { min_x.abs() as usize } else { 0 };
    let start_y = if min_y < 0 { min_y.abs() as usize } else { 0 };

    (grid_width, grid_height, (start_x, start_y))
}

fn draw_line(grid: Grid, instructions: &[Instruction], start_point: (usize, usize)) -> Grid {
    let mut grid: Vec<Vec<String>> = grid.clone();
    let (mut x, mut y) = start_point;

    for instruction in instructions {
        for _ in 0..instruction.distance {
            match instruction.direction {
                'U' => y = y.wrapping_sub(1),
                'D' => y += 1,
                'L' => x = x.wrapping_sub(1),
                'R' => x += 1,
                _ => {}
            }
            grid[y][x] = "#".to_owned(); //Some(instruction.color.clone());
        }
    }
    grid
}

fn flood_fill(grid: &mut Grid, start_x: usize, start_y: usize, fill_color: &str) {
    let mut stack = vec![(start_x, start_y)];

    while let Some((x, y)) = stack.pop() {
        if x >= grid.len() || y >= grid[0].len() || grid[x][y] == fill_color.to_string() {
            continue;
        }

        grid[x][y] = fill_color.to_string();

        if x > 0 {
            stack.push((x - 1, y));
        } // Up
        if y > 0 {
            stack.push((x, y - 1));
        } // Left
        if x < grid.len() - 1 {
            stack.push((x + 1, y));
        } // Down
        if y < grid[0].len() - 1 {
            stack.push((x, y + 1));
        } // Right
    }
}

fn get_enclosed_point(
    grid: Grid,
    line_points: &HashSet<(usize, usize)>,
    outside_line_points: &HashSet<(usize, usize)>,
) -> (usize, usize) {
    /*
       Implementation of https://en.wikipedia.org/wiki/Point_in_polygon
    */

    // https://en.wikipedia.org/wiki/Even%E2%80%93odd_rule
    for location in outside_line_points {
        if line_points
            .iter()
            .filter(|p| p.1 == location.1 && p.0 < location.0)
            .count()
            .min(
                line_points
                    .iter()
                    .filter(|p| p.1 == location.1 && p.0 < location.0)
                    .count(),
            )
            % 2
            == 1
        {
            // Just get the first enclosed point
            return *location;
        }
    }
    (0, 0)
}

fn print_grid(grid: &Grid) {
    for row in grid {
        for cell in row {
            print!("{} ", cell);
        }
        println!();
    }
}

fn char_to_hex_digit(c: char) -> Option<i128> {
    match c {
        '0'..='9' => Some(c as i128 - '0' as i128),
        'a'..='f' => Some(10 + c as i128 - 'a' as i128),
        'A'..='F' => Some(10 + c as i128 - 'A' as i128),
        _ => None,
    }
}

fn parse_hex_instruction(hex_str: &str) -> (char, i128) {
    println!("hex_str: {}", hex_str);
    let mut direction = 'R';
    let mut digits = Vec::new();
    for (i, ch) in hex_str.chars().enumerate() {
        print!("current char: {} ", ch);
        if ch == '#' || ch == '(' || ch == ')' {
            continue;
        }
        if i == hex_str.len() - 1 {
            let direction_code = ch.to_digit(16).unwrap();
            direction = match direction_code {
                0 => 'R',
                1 => 'D',
                2 => 'L',
                3 => 'U',
                _ => panic!("Invalid direction code"),
            };
            continue;
        }
        let digit = char_to_hex_digit(ch).unwrap();
        digits.push(digit);
    }
    println!();
    let number_str: String = digits.iter().map(|&num| num.to_string()).collect();

    (direction, number_str.parse::<i128>().unwrap())
}

impl TryFrom<FileLines> for Input {
    type Error = std::io::Error;

    fn try_from(lines: FileLines) -> Result<Self, Self::Error> {
        let mut instructions = Vec::new();
        let mut true_instructions = Vec::new();
        for line in lines {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let instruction = Instruction {
                direction: parts[0].chars().next().unwrap(),
                distance: parts[1].parse::<i128>().unwrap(),
                color: parts[2].to_string(),
            };
            instructions.push(instruction);

            let (direction, distance) = parse_hex_instruction(parts[2].trim());
            let true_instruction = Instruction {
                direction,
                distance,
                color: "#".to_owned(),
            };
            true_instructions.push(true_instruction);
        }
        let grid_size = calculate_grid_size(instructions.as_slice());
        Ok(Input {
            instructions,
            true_instructions,
            grid_size,
        })
    }
}

fn part_1(input_file: &str) -> std::io::Result<usize> {
    let input: Input = parse(input_file)?;
    let (grid_width, grid_height, start_point) = input.grid_size;

    let grid: Grid = vec![vec![".".to_owned(); grid_width]; grid_height];
    println!("grid_width: {}, grid_height: {}", grid_width, grid_height);
    let mut grid_with_line = draw_line(grid, &input.instructions, start_point);
    let mut line = HashSet::new();
    let mut out_side_line = HashSet::new();
    for (y, row) in grid_with_line.clone().iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell == "#" {
                line.insert((x, y));
            } else {
                out_side_line.insert((x, y));
            }
        }
    }

    let enclosed_points = get_enclosed_point(grid_with_line.clone(), &line, &out_side_line);
    flood_fill(
        &mut grid_with_line,
        enclosed_points.0,
        enclosed_points.1,
        "#",
    );
    print_grid(&grid_with_line);

    // Count all '#' in the grid
    let mut count = 0;
    for row in grid_with_line {
        for cell in row {
            if cell == "#" {
                count += 1;
            }
        }
    }

    Ok(count)
}

fn part_2(input_file: &str) -> std::io::Result<usize> {
    let input: Input = parse(input_file)?;
    let (grid_width, grid_height, start_point) = input.grid_size;

    let grid: Grid = vec![vec![".".to_owned(); grid_width]; grid_height];
    println!("grid_width: {}, grid_height: {}", grid_width, grid_height);
    let mut grid_with_line = draw_line(grid, &input.true_instructions, start_point);
    let mut line = HashSet::new();
    let mut out_side_line = HashSet::new();
    for (y, row) in grid_with_line.clone().iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell == "#" {
                line.insert((x, y));
            } else {
                out_side_line.insert((x, y));
            }
        }
    }

    let enclosed_points = get_enclosed_point(grid_with_line.clone(), &line, &out_side_line);
    flood_fill(
        &mut grid_with_line,
        enclosed_points.0,
        enclosed_points.1,
        "#",
    );
    print_grid(&grid_with_line);

    // Count all '#' in the grid
    let mut count = 0;
    for row in grid_with_line {
        for cell in row {
            if cell == "#" {
                count += 1;
            }
        }
    }

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    const INPUT: &str = "input/roar/q18_input.txt";
    const INPUT_SAMPLE: &str = "input/roar/q18_sample.txt";

    #[ignore]
    #[test]
    fn roar_q18_p1_sample() {
        let result = part_1(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 62);
    }

    #[ignore]
    #[test]
    fn roar_q18_p1_main() {
        let result = part_1(INPUT);
        assert_eq!(result.unwrap(), 52055);
    }

    #[ignore]
    #[test]
    fn roar_q18_p2_sample() {
        let result = part_2(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 0);
    }

    #[ignore]
    #[test]
    fn roar_q18_p2_main() {
        let result = part_2(INPUT);
        assert_eq!(result.unwrap(), 0);
    }
}
