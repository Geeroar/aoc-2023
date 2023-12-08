use crate::utils::parser::FileLines;

#[derive(Debug)]
struct Input {
    _seeds: Vec<u64>,
    _seed_to_soil: Vec<(u64, u64, u64)>,
    _soil_to_fertilizer: Vec<(u64, u64, u64)>,
    _fertilizer_to_water: Vec<(u64, u64, u64)>,
    _water_to_light: Vec<(u64, u64, u64)>,
    _light_to_temperature: Vec<(u64, u64, u64)>,
    _temperature_to_humidity: Vec<(u64, u64, u64)>,
    _humidity_to_location: Vec<(u64, u64, u64)>,
}

impl TryFrom<FileLines> for Input {
    type Error = std::io::Error;

    fn try_from(mut _lines: FileLines) -> Result<Self, Self::Error> {
        let _seeds = _lines
            .next()
            .unwrap()
            .strip_prefix("seeds: ")
            .unwrap()
            .split(' ')
            .map(|s| s.parse().unwrap())
            .collect();
        let mut _seed_to_soil = Vec::new();
        let mut _soil_to_fertilizer = Vec::new();
        let mut _fertilizer_to_water = Vec::new();
        let mut _water_to_light = Vec::new();
        let mut _light_to_temperature = Vec::new();
        let mut _temperature_to_humidity = Vec::new();
        let mut _humidity_to_location = Vec::new();
        _lines.next();
        let mut map: &mut Vec<(u64, u64, u64)> = &mut _seed_to_soil;
        for line in _lines {
            match line.as_str() {
                "seed-to-soil map:" => map = &mut _seed_to_soil,
                "soil-to-fertilizer map:" => map = &mut _soil_to_fertilizer,
                "fertilizer-to-water map:" => map = &mut _fertilizer_to_water,
                "water-to-light map:" => map = &mut _water_to_light,
                "light-to-temperature map:" => map = &mut _light_to_temperature,
                "temperature-to-humidity map:" => map = &mut _temperature_to_humidity,
                "humidity-to-location map:" => map = &mut _humidity_to_location,
                "" => (),
                s => {
                    let v: Vec<u64> = s.split(' ').map(|s| s.parse().unwrap()).collect();
                    map.push((v[0], v[1], v[2]));
                }
            }
        }
        _seed_to_soil.sort_by_key(|x| x.1);
        _soil_to_fertilizer.sort_by_key(|x| x.1);
        _fertilizer_to_water.sort_by_key(|x| x.1);
        _water_to_light.sort_by_key(|x| x.1);
        _light_to_temperature.sort_by_key(|x| x.1);
        _temperature_to_humidity.sort_by_key(|x| x.1);
        _humidity_to_location.sort_by_key(|x| x.1);
        Ok(Input {
            _seeds,
            _seed_to_soil,
            _soil_to_fertilizer,
            _fertilizer_to_water,
            _water_to_light,
            _light_to_temperature,
            _temperature_to_humidity,
            _humidity_to_location,
        })
    }
}

impl Input {
    fn _location(&self, seed: u64) -> u64 {
        let mut result = seed;
        result = self._lookup(result, &self._seed_to_soil);
        result = self._lookup(result, &self._soil_to_fertilizer);
        result = self._lookup(result, &self._fertilizer_to_water);
        result = self._lookup(result, &self._water_to_light);
        result = self._lookup(result, &self._light_to_temperature);
        result = self._lookup(result, &self._temperature_to_humidity);
        result = self._lookup(result, &self._humidity_to_location);
        result
    }

    fn _lookup(&self, source: u64, map: &Vec<(u64, u64, u64)>) -> u64 {
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

fn _part_1(input_file: &str) -> std::io::Result<u64> {
    let input = Input::try_from(FileLines::new(input_file)?)?;
    Ok(input
        ._seeds
        .iter()
        .map(|&s| input._location(s))
        .min()
        .unwrap())
}

fn _part_2(input_file: &str) -> std::io::Result<u64> {
    let input = Input::try_from(FileLines::new(input_file)?)?;
    let mut iter = input._seeds.iter();
    let mut pairs: Vec<(u64, u64)> = Vec::new();
    while let Some(&s) = iter.next() {
        if let Some(&r) = iter.next() {
            pairs.push((s, r));
        }
    }
    let mut min_location = u64::MAX;
    for (i, range) in &pairs {
        for j in 0..*range {
            let location = input._location(i + j);
            if location < min_location {
                min_location = location;
            }
        }
    }
    Ok(min_location)
}

#[cfg(test)]
mod tests {
    use super::{_part_1, _part_2};

    const INPUT: &str = "input/gee/q05_input.txt";
    const INPUT_SAMPLE: &str = "input/gee/q05_sample.txt";

    #[test]
    fn gee_q05_p1_sample() {
        let result = _part_1(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 35);
    }

    #[test]
    fn gee_q05_p1_main() {
        let result = _part_1(INPUT);
        assert_eq!(result.unwrap(), 457535844);
    }

    #[test]
    fn gee_q05_p2_sample() {
        let result = _part_2(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 46);
    }

    #[ignore]
    #[test]
    fn gee_q05_p2_main() {
        let result = _part_2(INPUT);
        assert_eq!(result.unwrap(), 41222968);
    }
}
