#![allow(dead_code, unused_variables)]

use crate::utils::parser::{parse, FileLines};
use std::collections::HashMap;

struct Input {
    sequence: Vec<String>,
}

struct LensBox<'a> {
    lenses: HashMap<&'a str, (usize, usize)>,
    slots: usize,
}

impl TryFrom<FileLines> for Input {
    type Error = std::io::Error;

    fn try_from(mut lines: FileLines) -> Result<Self, Self::Error> {
        Ok(Input {
            sequence: lines.next().unwrap().split(',').map(String::from).collect(),
        })
    }
}

impl<'a> LensBox<'a> {
    fn new() -> Self {
        LensBox {
            lenses: HashMap::new(),
            slots: 0,
        }
    }

    fn focusing_power(&self, box_number: usize) -> usize {
        let mut lenses: Vec<&(usize, usize)> = self.lenses.values().collect();
        lenses.sort_by_key(|(i, _)| i);
        lenses
            .iter()
            .enumerate()
            .map(|(i, (_, v))| (box_number + 1) * (i + 1) * v)
            .sum()
    }

    fn remove_lens(&mut self, label: &str) {
        self.lenses.remove(label);
    }

    fn set_lens(&mut self, label: &'a str, focal_length: usize) {
        if self.lenses.contains_key(label) {
            self.lenses
                .entry(label)
                .and_modify(|(_, l)| *l = focal_length);
        } else {
            self.lenses.insert(label, (self.slots, focal_length));
            self.slots += 1;
        }
    }
}

fn hash(s: &str) -> usize {
    let mut value = 0;
    for c in s.chars() {
        value += c as usize;
        value *= 17;
        value %= 256;
    }
    value
}

fn hash_map<'a>(s: &'a str, boxes: &mut [LensBox<'a>]) {
    if s.ends_with('-') {
        let (label, _) = s.split_once('-').unwrap();
        let b = &mut boxes[hash(label)];
        b.remove_lens(label);
    } else {
        let (label, focal_length) = s.split_once('=').unwrap();
        let b = &mut boxes[hash(label)];
        b.set_lens(label, focal_length.parse().unwrap());
    }
}

fn part_1(input_file: &str) -> std::io::Result<usize> {
    let input: Input = parse(input_file)?;
    Ok(input.sequence.iter().map(|s| hash(s)).sum())
}

fn part_2(input_file: &str) -> std::io::Result<usize> {
    let input: Input = parse(input_file)?;
    let mut boxes = Vec::<LensBox>::new();
    for _ in 0..256 {
        boxes.push(LensBox::new());
    }
    input.sequence.iter().for_each(|s| hash_map(s, &mut boxes));
    Ok(boxes
        .iter()
        .enumerate()
        .map(|(i, b)| b.focusing_power(i))
        .sum())
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    const INPUT: &str = "input/gee/q15_input.txt";
    const INPUT_SAMPLE: &str = "input/gee/q15_sample.txt";

    #[test]
    fn gee_q15_p1_sample() {
        let result = part_1(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 1320);
    }

    #[test]
    fn gee_q15_p1_main() {
        let result = part_1(INPUT);
        assert_eq!(result.unwrap(), 506891);
    }

    #[test]
    fn gee_q15_p2_sample() {
        let result = part_2(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 145);
    }

    #[test]
    fn gee_q15_p2_main() {
        let result = part_2(INPUT);
        assert_eq!(result.unwrap(), 230462);
    }
}
