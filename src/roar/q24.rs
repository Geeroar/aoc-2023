#![allow(dead_code, unused_variables)]

use crate::utils::parser::{parse, FileLines};

#[derive(PartialEq, Debug)]
struct Hailstone {
    px: i128,
    py: i128,
    vx: i128,
    vy: i128,
}

impl Hailstone {
    fn will_intersect_within_area(
        &self,
        other: &Hailstone,
        min_x: i128,
        max_x: i128,
        min_y: i128,
        max_y: i128,
    ) -> bool {
        // Check if the paths are parallel
        if self.vx * other.vy == self.vy * other.vx {
            return false;
        }

        // https://brilliant.org/wiki/linear-equations-intersection-of-lines/
        let t = (other.py - self.py) as f64 / self.vy as f64
            - (other.px - self.px) as f64 / self.vx as f64;
        let intersect_x = self.px as f64 + self.vx as f64 * t;
        let intersect_y = self.py as f64 + self.vy as f64 * t;

        // Check range
        intersect_x >= min_x as f64
            && intersect_x <= max_x as f64
            && intersect_y >= min_y as f64
            && intersect_y <= max_y as f64
    }
}

struct Input {
    hailstone: Vec<Hailstone>,
}

impl TryFrom<FileLines> for Input {
    type Error = std::io::Error;

    fn try_from(lines: FileLines) -> Result<Self, Self::Error> {
        let mut points = Vec::new();
        for line in lines {
            let parts: Vec<i128> = line
                .split(&[',', '@'][..])
                .map(|s| s.trim().parse::<i128>().unwrap())
                .collect();
            points.push(Hailstone {
                px: parts[0],
                py: parts[1],
                vx: parts[2],
                vy: parts[3],
            });
        }
        Ok(Input { hailstone: points })
    }
}

fn part_1(input_file: &str, min: i128, max: i128) -> std::io::Result<i128> {
    let input: Input = parse(input_file)?;
    let mut total_intersections = 0;

    for stone in input.hailstone.iter() {
        for other_stone in input.hailstone.iter() {
            if stone == other_stone {
                continue;
            }
            if stone.will_intersect_within_area(other_stone, min, max, min, max) {
                println!("{} {:?}", stone.px, stone);
                total_intersections += 1;
            }
        }
    }
    Ok(total_intersections)
}

fn part_2(input_file: &str) -> std::io::Result<i128> {
    let input: Input = parse(input_file)?;
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2, Hailstone};

    const INPUT: &str = "input/roar/q24_input.txt";
    const INPUT_SAMPLE: &str = "input/roar/q24_sample.txt";

    #[test]
    fn roar_q24_p1_sample() {
        let result = part_1(INPUT_SAMPLE, 7, 27);
        assert_eq!(result.unwrap(), 2);
    }

    #[ignore = "aaaargh!!!"]
    #[test]
    fn roar_q24_p1_main() {
        let result = part_1(INPUT, 200000000000000, 400000000000000);
        // -1 is not the answer - need to figure the answer out
        assert_eq!(result.unwrap(), -1);
    }

    #[test]
    fn roar_q24_p2_sample() {
        let result = part_2(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn roar_q24_p2_main() {
        let result = part_2(INPUT);
        assert_eq!(result.unwrap(), 0);
    }
    #[test]
    fn test_hailstones_intersect_within_area_1() {
        let hailstone_a = Hailstone {
            px: 19,
            py: 13,
            vx: -2,
            vy: 1,
        };
        let hailstone_b = Hailstone {
            px: 18,
            py: 19,
            vx: -1,
            vy: -1,
        };

        let min_x = 7;
        let max_x = 27;
        let min_y = 7;
        let max_y = 27;

        assert!(hailstone_a.will_intersect_within_area(&hailstone_b, min_x, max_x, min_y, max_y));
    }

    #[test]
    fn test_hailstones_intersect_outside_area() {
        let hailstone_a = Hailstone {
            px: 19,
            py: 13,
            vx: -2,
            vy: 1,
        };
        let hailstone_b = Hailstone {
            px: 12,
            py: 31,
            vx: -1,
            vy: -2,
        };

        let min_x = 7;
        let max_x = 27;
        let min_y = 7;
        let max_y = 27;

        assert!(!hailstone_a.will_intersect_within_area(&hailstone_b, min_x, max_x, min_y, max_y));
    }

    #[test]
    fn test_hailstones_past_intersection() {
        let hailstone_a = Hailstone {
            px: 19,
            py: 13,
            vx: -2,
            vy: 1,
        };
        let hailstone_b = Hailstone {
            px: 20,
            py: 19,
            vx: 1,
            vy: -5,
        };

        let min_x = 7;
        let max_x = 27;
        let min_y = 7;
        let max_y = 27;

        assert!(!hailstone_a.will_intersect_within_area(&hailstone_b, min_x, max_x, min_y, max_y));
    }
}
