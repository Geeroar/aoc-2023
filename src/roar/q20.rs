#![allow(dead_code, unused_variables)]

use crate::utils::parser::{parse, FileLines};
use std::collections::{HashMap, VecDeque};
use std::convert::TryFrom;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, Clone)]
enum ModuleType {
    FlipFlop(bool),                      // Stores state: on/off - false = off, true = on
    Conjunction(HashMap<String, Pulse>), // Stores state of inputs
    Broadcaster,
}

#[derive(Debug, Clone)]
struct Module {
    module_type: ModuleType,
    destinations: Vec<String>,
}

#[derive(Debug)]
struct Counter {
    low_pulse_count: usize,
    high_pulse_count: usize,
    pulses_sent_to_rx: usize,
}

struct SystemState {
    modules: HashMap<String, Module>,
    pulse_queue: VecDeque<(String, String, Pulse)>, // Queue for pulses: (module_name, pulse_type)
    counter: Counter,
}

impl SystemState {
    fn map_inputs(&mut self) {
        let modules_clone = self.modules.clone();
        // Scan the modules to find inputs to the Conjunction modules
        for (module_name, module) in modules_clone {
            for dest in module.destinations {
                if self.modules.contains_key(&dest) {
                    let dest_module = self.modules.get_mut(&dest).unwrap();
                    match &mut dest_module.module_type {
                        ModuleType::Conjunction(inputs) => {
                            inputs.insert(module_name.clone(), Pulse::Low);
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    fn propagate(&mut self) {
        while let Some((module_name, source, pulse)) = self.pulse_queue.pop_front() {
            if pulse == Pulse::Low {
                self.counter.low_pulse_count += 1;
            } else {
                self.counter.high_pulse_count += 1;
            }

            if module_name == "rx" && pulse == Pulse::Low {
                println!("rx: {:?}", pulse);
                self.counter.pulses_sent_to_rx += 1;
            }
            if !self.modules.contains_key(&module_name) {
                continue;
            }

            let module = self.modules.get_mut(&module_name).unwrap();

            match &mut module.module_type {
                ModuleType::FlipFlop(state) => {
                    /*
                        Flip-flop modules (prefix %) are either on or off; they are initially off.
                        If a flip-flop module receives a high pulse, it is ignored and nothing happens.
                        However, if a flip-flop module receives a low pulse, it flips between on and off.
                        If it was off, it turns on and sends a high pulse.
                        If it was on, it turns off and sends a low pulse.
                    */
                    if pulse == Pulse::Low {
                        *state = !*state; // Toggle state
                        let pulse_to_send = if *state { Pulse::High } else { Pulse::Low };
                        for dest in &module.destinations {
                            self.pulse_queue.push_back((
                                dest.clone(),
                                module_name.clone(),
                                pulse_to_send,
                            ));
                        }
                    }
                }
                ModuleType::Conjunction(inputs) => {
                    /*
                     * Conjunction modules (prefix &) remember the type of the most recent pulse received from each of their connected input modules;
                     * they initially default to remembering a low pulse for each input.
                     * When a pulse is received, the conjunction module first updates its memory for that input.
                     * Then, if it remembers high pulses for all inputs, it sends a low pulse; otherwise, it sends a high pulse.
                     */
                    inputs.insert(source.clone(), pulse);
                    let pulse_to_send = if inputs.values().all(|&p| Pulse::High == p) {
                        Pulse::Low
                    } else {
                        Pulse::High
                    };
                    for dest in &module.destinations {
                        self.pulse_queue.push_back((
                            dest.clone(),
                            module_name.clone(),
                            pulse_to_send,
                        ));
                    }
                }
                ModuleType::Broadcaster => {
                    /*
                     * There is a single broadcast module (named broadcaster).
                     * When it receives a pulse, it sends the same pulse to all of its destination modules.
                     */
                    for dest in &module.destinations {
                        self.pulse_queue
                            .push_back((dest.clone(), module_name.clone(), pulse));
                    }
                }
            }
        }
    }

    fn press_button(&mut self) {
        self.pulse_queue.push_back((
            "broadcaster".to_string(),
            "broadcaster".to_string(),
            Pulse::Low,
        ));
        self.propagate();
    }
}

struct Input {
    state: SystemState,
}

impl TryFrom<FileLines> for Input {
    type Error = std::io::Error;

    fn try_from(lines: FileLines) -> Result<Self, Self::Error> {
        let mut modules = HashMap::new();
        for line in lines {
            let (module, destinations) = line.split_once(" -> ").unwrap();
            if module == "broadcaster" {
                modules.insert(
                    module.to_string(),
                    Module {
                        module_type: ModuleType::Broadcaster,
                        destinations: destinations.split(", ").map(|s| s.to_string()).collect(),
                    },
                );
            } else if module.starts_with('%') {
                let module_name = module[1..].trim().to_string();
                modules.insert(
                    module_name.clone(),
                    Module {
                        module_type: ModuleType::FlipFlop(false),
                        destinations: destinations.split(", ").map(|s| s.to_string()).collect(),
                    },
                );
            } else if module.starts_with('&') {
                let module_name = module[1..].trim().to_string();
                let destinations: Vec<String> =
                    destinations.split(", ").map(|s| s.to_string()).collect();
                modules.insert(
                    module_name.clone(),
                    Module {
                        module_type: ModuleType::Conjunction(HashMap::new()),
                        destinations,
                    },
                );
            }
        }

        Ok(Input {
            state: SystemState {
                modules,
                pulse_queue: VecDeque::new(),
                counter: Counter {
                    low_pulse_count: 0,
                    high_pulse_count: 0,
                    pulses_sent_to_rx: 0,
                },
            },
        })
    }
}

fn part_1(input_file: &str) -> std::io::Result<usize> {
    let input: Input = parse(input_file)?;
    let mut system_state: SystemState = input.state;
    system_state.map_inputs();

    // Print state.modules hashmap
    for (module_name, module) in &system_state.modules {
        println!("{}: {:?}", module_name, module);
    }

    for _ in 0..1000 {
        system_state.press_button();
    }
    println!("{:?}", system_state.counter);
    Ok(system_state.counter.low_pulse_count * system_state.counter.high_pulse_count)
}

fn part_2(input_file: &str) -> std::io::Result<u64> {
    let input: Input = parse(input_file)?;
    let mut system_state: SystemState = input.state;
    system_state.map_inputs();
    system_state.press_button();

    let mut button_count = 1;
    while system_state.counter.pulses_sent_to_rx != 1 {
        system_state.counter.pulses_sent_to_rx = 0;
        system_state.press_button();
        button_count += 1;
    }
    Ok(button_count)
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    const INPUT: &str = "input/roar/q20_input.txt";
    const INPUT_SAMPLE: &str = "input/roar/q20_sample.txt";

    #[test]
    fn roar_q20_p1_sample() {
        let result = part_1(INPUT_SAMPLE);
        assert_eq!(result.unwrap(), 11687500);
    }

    #[test]
    fn roar_q20_p1_main() {
        let result = part_1(INPUT);
        assert_eq!(result.unwrap(), 925955316);
    }

    #[ignore = "Heat death of the universe runtime"]
    #[test]
    fn roar_q20_p2_main() {
        let result = part_2(INPUT);
        assert_eq!(result.unwrap(), 0);
    }
}
