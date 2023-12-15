#![allow(dead_code)]

use crate::utils::parser::FileLines;

#[derive(Debug)]
struct Input {
    seeds: Vec<u64>,
    seed_to_soil: Vec<(u64, u64, u64)>,
    soil_to_fertilizer: Vec<(u64, u64, u64)>,
    fertilizer_to_water: Vec<(u64, u64, u64)>,
    water_to_light: Vec<(u64, u64, u64)>,
    light_to_temperature: Vec<(u64, u64, u64)>,
    temperature_to_humidity: Vec<(u64, u64, u64)>,
    humidity_to_location: Vec<(u64, u64, u64)>,
}

impl TryFrom<FileLines> for Input {
    type Error = std::io::Error;

    fn try_from(mut lines: FileLines) -> Result<Self, Self::Error> {
        let seeds = lines
            .next()
            .unwrap()
            .strip_prefix("seeds: ")
            .unwrap()
            .split(' ')
            .map(|s| s.parse().unwrap())
            .collect();
        let mut seed_to_soil = Vec::new();
        let mut soil_to_fertilizer = Vec::new();
        let mut fertilizer_to_water = Vec::new();
        let mut water_to_light = Vec::new();
        let mut light_to_temperature = Vec::new();
        let mut temperature_to_humidity = Vec::new();
        let mut humidity_to_location = Vec::new();
        lines.next();
        let mut map: &mut Vec<(u64, u64, u64)> = &mut seed_to_soil;
        for line in lines {
            match line.as_str() {
                "seed-to-soil map:" => map = &mut seed_to_soil,
                "soil-to-fertilizer map:" => map = &mut soil_to_fertilizer,
                "fertilizer-to-water map:" => map = &mut fertilizer_to_water,
                "water-to-light map:" => map = &mut water_to_light,
                "light-to-temperature map:" => map = &mut light_to_temperature,
                "temperature-to-humidity map:" => map = &mut temperature_to_humidity,
                "humidity-to-location map:" => map = &mut humidity_to_location,
                "" => (),
                s => {
                    let v: Vec<u64> = s.split(' ').map(|s| s.parse().unwrap()).collect();
                    map.push((v[0], v[1], v[2]));
                }
            }
        }
        seed_to_soil.sort_by_key(|x| x.1);
        soil_to_fertilizer.sort_by_key(|x| x.1);
        fertilizer_to_water.sort_by_key(|x| x.1);
        water_to_light.sort_by_key(|x| x.1);
        light_to_temperature.sort_by_key(|x| x.1);
        temperature_to_humidity.sort_by_key(|x| x.1);
        humidity_to_location.sort_by_key(|x| x.1);
        Ok(Input {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        })
    }
}

impl Input {
    fn location(&self, seed: u64) -> u64 {
        let mut result = seed;
        result = self.lookup(result, &self.seed_to_soil);
        result = self.lookup(result, &self.soil_to_fertilizer);
        result = self.lookup(result, &self.fertilizer_to_water);
        result = self.lookup(result, &self.water_to_light);
        result = self.lookup(result, &self.light_to_temperature);
        result = self.lookup(result, &self.temperature_to_humidity);
        result = self.lookup(result, &self.humidity_to_location);
        result
    }

    fn lookup(&self, source: u64, map: &Vec<(u64, u64, u64)>) -> u64 {
        let (mut a, mut b) = (0, map.len() - 1);
        while a <= b {
            let i = (a + b) / 2;
            let (dest, src, range) = map[i];
            if src <= source && source < src + range {
                return dest + source - src;
            } else if source < src {
                if i == 0 {
                    break;
                }
                b = i - 1;
            } else if source > src {
                a = i + 1;
            }
        }
        source
    }
}

fn part_1(input_file: &str) -> std::io::Result<u64> {
    let input = Input::try_from(FileLines::new(input_file)?)?;
    Ok(input
        .seeds
        .iter()
        .map(|&s| input.location(s))
        .min()
        .unwrap())
}

fn part_2(input_file: &str) -> std::io::Result<u64> {
    let input = Input::try_from(FileLines::new(input_file)?)?;
    let mut iter = input.seeds.iter();
    let mut pairs: Vec<(u64, u64)> = Vec::new();
    while let Some(&s) = iter.next() {
        if let Some(&r) = iter.next() {
            pairs.push((s, r));
        }
    }
    let mut min_location = u64::MAX;
    for (i, range) in &pairs {
        for j in 0..*range {
            let location = input.location(i + j);
            if location < min_location {
                min_location = location;
            }
        }
    }
    Ok(min_location)
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    const INPUT: &str = "input/gee/q05_input.txt";
    const INPUT_SAMPLE: &str = "input/gee/q05_sample.txt";

    #[test]
    fn gee_q05_p1_sample() {
        let result = part_1(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 35);
    }

    #[test]
    fn gee_q05_p1_main() {
        let result = part_1(INPUT);
        assert_eq!(result.unwrap(), 457535844);
    }

    #[test]
    fn gee_q05_p2_sample() {
        let result = part_2(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 46);
    }

    #[ignore]
    #[test]
    fn gee_q05_p2_main() {
        let result = part_2(INPUT);
        assert_eq!(result.unwrap(), 41222968);
    }
}
