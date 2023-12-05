use std::{
    cmp::{max, min},
    collections::HashMap,
};

use crate::utils::parser::FileLines;

#[derive(Debug, Clone)]
struct ConversionMap {
    _destination_range_start: u64,
    _source_range_start: u64,
    _range_length: u64,
}

#[derive(Debug)]
struct SeedAlmanac {
    _seeds: Vec<u64>,
    _mappings: HashMap<String, Vec<ConversionMap>>,
}

#[derive(Debug)]
struct Input {
    _almanac: SeedAlmanac,
}

const _MAP_STEPS: [&str; 7] = [
    "seed-to-soil",
    "soil-to-fertilizer",
    "fertilizer-to-water",
    "water-to-light",
    "light-to-temperature",
    "temperature-to-humidity",
    "humidity-to-location",
];

impl TryFrom<FileLines> for Input {
    type Error = std::io::Error;

    fn try_from(_lines: FileLines) -> Result<Self, Self::Error> {
        let mut almanac = SeedAlmanac {
            _seeds: Vec::new(),
            _mappings: HashMap::new(),
        };
        let mut current_mapping_type = String::new();

        for line in _lines {
            if line.starts_with("seeds") {
                almanac._seeds = line
                    .split_once(": ")
                    .unwrap()
                    .1
                    .split_whitespace()
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect();
            } else if line.trim().ends_with("map:") {
                current_mapping_type = line.split_whitespace().next().unwrap().to_string();
                almanac
                    ._mappings
                    .insert(current_mapping_type.clone(), Vec::new());
            } else if !line.is_empty() && line.chars().next().unwrap().is_numeric() {
                // starts with a number
                let nums: Vec<u64> = line
                    .split_whitespace()
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect();

                let conversion_map = ConversionMap {
                    _destination_range_start: nums[0],
                    _source_range_start: nums[1],
                    _range_length: nums[2],
                };
                almanac
                    ._mappings
                    .get_mut(&current_mapping_type)
                    .unwrap()
                    .push(conversion_map);
            }
        }

        Ok(Input { _almanac: almanac })
    }
}

fn _calculate_destination(seed: u64, start: u64, destination: u64, range: u64) -> u64 {
    if seed >= start && seed < start + range {
        return destination + (seed - start);
    }
    return seed;
}

fn _part_1(input_file: &str) -> std::io::Result<u64> {
    let input = Input::try_from(FileLines::new(input_file)?)?;
    let almanac = input._almanac;
    let mut locations = Vec::new();
    for seed in almanac._seeds {
        let mut current = seed;
        for step in _MAP_STEPS {
            let conversion_map = almanac._mappings.get(step).unwrap();
            for conversion_map in conversion_map {
                let current_location = _calculate_destination(
                    current,
                    conversion_map._source_range_start,
                    conversion_map._destination_range_start,
                    conversion_map._range_length,
                );
                if current_location != current {
                    current = current_location;
                    break;
                }
            }

            if step == "humidity-to-location" {
                locations.push(current);
            }
        }
    }
    Ok(locations.iter().min().unwrap().clone())
}

fn _build_maps(almanac: &SeedAlmanac) -> Vec<&Vec<ConversionMap>> {
    // Try to reduce nesting later, I definitely made poor choices on structure here
    let mut maps_list = Vec::new();
    for step in _MAP_STEPS {
        let conversion_maps = almanac._mappings.get(step).unwrap();
        maps_list.push(conversion_maps);
    }
    return maps_list;
}

fn _part_2(input_file: &str) -> std::io::Result<u64> {
    let input = Input::try_from(FileLines::new(input_file)?)?;
    let almanac = input._almanac;
    let mut locations: Vec<u64> = Vec::new();

    for seed_range in almanac._seeds.chunks(2) {
        println!("Seed range {:?}", seed_range);
        let mut ranges = vec![(seed_range[0], seed_range[0] + seed_range[1] - 1)];

        for conversion_maps in _build_maps(&almanac) {
            let mut transformed_ranges = vec![];
            for cm in conversion_maps {
                let source_start = cm._source_range_start;
                let destination = cm._destination_range_start;
                let source_len = cm._range_length;
                let source_end = source_start + source_len;

                // Keep track of this madness
                let mut new_ranges = vec![];

                for (start, end) in ranges {
                    // Attempt a sort of sliding interval thing
                    let left_side = (start, min(end, source_start));
                    let right_side = (max(source_end, start), end);
                    let interval = (max(start, source_start), min(source_end, end));

                    if left_side.1 > left_side.0 {
                        new_ranges.push(left_side);
                    }
                    if right_side.1 > right_side.0 {
                        new_ranges.push(right_side);
                    }
                    if interval.1 > interval.0 {
                        // Basically do _calculate_destination on each side of interval
                        transformed_ranges
                            .push((destination + (interval.0 - source_start), destination + (interval.1 - source_start)));
                    }
                }
                // Update ranges on each line of the conversion map, e.g. for soil-to-fertilizer and so on
                ranges = new_ranges;
            }
            transformed_ranges.extend(ranges);
            ranges = transformed_ranges;
        }
        locations.push(ranges.iter().min().unwrap().0);
    }
    Ok(locations.iter().min().unwrap().clone())
}

#[cfg(test)]
mod tests {
    use super::{_part_1, _part_2};

    const INPUT: &str = "input/roar/q05_input.txt";
    const INPUT_SAMPLE: &str = "input/roar/q05_sample.txt";

    #[test]
    fn roar_q05_p1_sample() {
        let result = _part_1(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 35);
    }

    #[test]
    fn roar_q05_p1_main() {
        let result = _part_1(INPUT);
        assert_eq!(result.unwrap(), 174137457);
    }

    #[test]
    fn roar_q05_p2_sample() {
        let result = _part_2(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 46);
    }

    #[test]
    fn roar_q05_p2_main() {
        let result = _part_2(INPUT);
        assert_eq!(result.unwrap(), 1493866);
    }
}
