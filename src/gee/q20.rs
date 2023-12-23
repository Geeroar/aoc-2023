#![allow(dead_code, unused_variables)]

use crate::utils::parser::{parse, FileLines};
use std::cmp;
use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
struct Input {
    nodes: HashMap<String, ModuleType>,
    inputs: HashMap<String, Vec<String>>,
    outputs: HashMap<String, Vec<String>>,
}

#[derive(Debug)]
enum ModuleType {
    Broadcast,
    Conjunction,
    FlipFlop,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Pulse {
    Low,
    High,
}

impl TryFrom<FileLines> for Input {
    type Error = std::io::Error;

    fn try_from(lines: FileLines) -> Result<Self, Self::Error> {
        let mut nodes = HashMap::new();
        let mut inputs = HashMap::<String, Vec<String>>::new();
        let mut outputs = HashMap::<String, Vec<String>>::new();
        for line in lines {
            let (m, d) = line.split_once(" -> ").unwrap();
            let name = match m.chars().next() {
                Some('b') => {
                    nodes.insert(String::from(m), ModuleType::Broadcast);
                    m
                }
                Some('%') => {
                    let name = m.strip_prefix('%').unwrap();
                    nodes.insert(String::from(name), ModuleType::FlipFlop);
                    name
                }
                Some('&') => {
                    let name = m.strip_prefix('&').unwrap();
                    nodes.insert(String::from(name), ModuleType::Conjunction);
                    name
                }
                _ => panic!("Bad module type: {}", m),
            };
            for downstream in d.split(", ") {
                if name != "broadcaster" {
                    inputs
                        .entry(String::from(downstream))
                        .and_modify(|v| v.push(String::from(name)))
                        .or_insert_with(|| vec![String::from(name)]);
                }
                outputs
                    .entry(String::from(name))
                    .and_modify(|v| v.push(String::from(downstream)))
                    .or_insert_with(|| vec![String::from(downstream)]);
            }
        }
        Ok(Input {
            nodes,
            inputs,
            outputs,
        })
    }
}

fn press_button(
    input: &Input,
    flip_flops: &mut HashMap<String, bool>,
    conjunctions: &mut HashMap<String, HashMap<String, Pulse>>,
) -> (usize, usize) {
    let mut low_pulses = 0;
    let mut high_pulses = 0;
    let mut queue = VecDeque::<(String, String, Pulse)>::new();
    for s in input.outputs.get("broadcaster").unwrap() {
        queue.push_back((String::from("broadcaster"), s.clone(), Pulse::Low));
    }
    while let Some((src, dest, pulse)) = queue.pop_front() {
        match pulse {
            Pulse::Low => low_pulses += 1,
            Pulse::High => high_pulses += 1,
        }
        match input.nodes.get(&dest) {
            Some(ModuleType::FlipFlop) => {
                if pulse == Pulse::Low {
                    let state = *flip_flops
                        .entry(dest.clone())
                        .and_modify(|s| *s = !*s)
                        .or_insert(true);
                    let next_pulse = if state { Pulse::High } else { Pulse::Low };
                    for downstream in input.outputs.get(&dest).unwrap() {
                        queue.push_back((dest.clone(), downstream.clone(), next_pulse));
                    }
                }
            }
            Some(ModuleType::Conjunction) => {
                let e = conjunctions
                    .entry(dest.clone())
                    .and_modify(|n| {
                        n.insert(src.clone(), pulse);
                    })
                    .or_insert_with(|| {
                        let mut m = HashMap::new();
                        m.insert(src.clone(), pulse);
                        m
                    });
                let next_pulse = match pulse {
                    Pulse::Low => Pulse::High,
                    Pulse::High => {
                        let inputs = input.inputs.get(&dest.clone()).unwrap();
                        if inputs
                            .iter()
                            .all(|i| *e.get(i).unwrap_or(&Pulse::Low) == Pulse::High)
                        {
                            Pulse::Low
                        } else {
                            Pulse::High
                        }
                    }
                };
                for downstream in input.outputs.get(&dest).unwrap() {
                    queue.push_back((dest.clone(), downstream.clone(), next_pulse));
                }
            }
            _ => (),
        }
    }
    (low_pulses, high_pulses)
}

fn count_pulses(input: &Input, count: usize) -> usize {
    let mut low_pulses = count;
    let mut high_pulses = 0;
    let mut flip_flops = HashMap::<String, bool>::new();
    let mut conjunctions = HashMap::<String, HashMap<String, Pulse>>::new();
    for _ in 0..count {
        let (l, h) = press_button(input, &mut flip_flops, &mut conjunctions);
        low_pulses += l;
        high_pulses += h;
    }
    low_pulses * high_pulses
}

fn lcm(a: usize, b: usize) -> usize {
    let (small, big) = (cmp::min(a, b), cmp::max(a, b));
    if big % small == 0 {
        return big;
    }
    for i in 1..small {
        if big * i % small == 0 {
            return big * i;
        }
    }
    small * big
}

fn part_1(input_file: &str) -> std::io::Result<usize> {
    let input: Input = parse(input_file)?;
    Ok(count_pulses(&input, 1000))
}

fn part_2(input_file: &str) -> std::io::Result<usize> {
    let input: Input = parse(input_file)?;
    let mut presses = 0;
    let mut flip_flops = HashMap::<String, bool>::new();
    let mut conjunctions = HashMap::<String, HashMap<String, Pulse>>::new();
    let mut group_1_loop = 0;
    let mut group_2_loop = 0;
    let mut group_3_loop = 0;
    let mut group_4_loop = 0;
    let group_1 = [
        "vl", "lv", "rd", "lk", "hc", "kb", "pc", "rz", "fr", "mr", "jd", "mf",
    ];
    let group_2 = [
        "ml", "xt", "bc", "nb", "qj", "hd", "bq", "tb", "pk", "fq", "bh", "pr",
    ];
    let group_3 = [
        "cn", "sn", "fd", "nt", "kq", "qq", "sq", "kl", "mb", "nh", "rq", "ch",
    ];
    let group_4 = [
        "cs", "zb", "vz", "nd", "jg", "fl", "nf", "sm", "cp", "kk", "bj", "lj",
    ];
    loop {
        presses += 1;
        press_button(&input, &mut flip_flops, &mut conjunctions);
        if group_1
            .iter()
            .all(|&g| !*flip_flops.get(g).unwrap_or(&false))
        {
            group_1_loop = presses;
            println!("Group 1 loop: {}", group_1_loop);
        }
        if group_2
            .iter()
            .all(|&g| !*flip_flops.get(g).unwrap_or(&false))
        {
            group_2_loop = presses;
            println!("Group 2 loop: {}", group_2_loop);
        }
        if group_3
            .iter()
            .all(|&g| !*flip_flops.get(g).unwrap_or(&false))
        {
            group_3_loop = presses;
            println!("Group 3 loop: {}", group_3_loop);
        }
        if group_4
            .iter()
            .all(|&g| !*flip_flops.get(g).unwrap_or(&false))
        {
            group_4_loop = presses;
            println!("Group 4 loop: {}", group_4_loop);
        }

        if group_1_loop > 0 && group_2_loop > 0 && group_3_loop > 0 && group_4_loop > 0 {
            break;
        }
    }
    let mut result = lcm(group_1_loop, group_2_loop);
    result = lcm(result, group_3_loop);
    result = lcm(result, group_4_loop);
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    const INPUT: &str = "input/gee/q20_input.txt";
    const INPUT_SAMPLE: &str = "input/gee/q20_sample.txt";

    #[test]
    fn gee_q20_p1_sample() {
        let result = part_1(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 11687500);
    }

    #[test]
    fn gee_q20_p1_main() {
        let result = part_1(INPUT);
        assert_eq!(result.unwrap(), 666795063);
    }

    #[test]
    fn gee_q20_p2_main() {
        let result = part_2(INPUT);
        assert_eq!(result.unwrap(), 253302889093151);
    }
}
