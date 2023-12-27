use anyhow::{Context, Error, Result};
use std::borrow::Borrow;
use std::collections::{HashMap, VecDeque};
use std::hash::{Hash, Hasher};

#[derive(Eq, PartialEq)]
enum ModuleKind<'a> {
    FlipFlop(bool),
    Conjunction(HashMap<&'a str, bool>),
    BroadCaster,
}

impl<'a> From<char> for ModuleKind<'a> {
    fn from(value: char) -> Self {
        match value {
            '%' => Self::FlipFlop(false),
            '&' => Self::Conjunction(HashMap::default()),
            _ => Self::BroadCaster,
        }
    }
}

#[derive(Eq, PartialEq)]
struct Module<'a> {
    kind: ModuleKind<'a>,
    label: &'a str,
    outputs: Vec<&'a str>,
}

impl<'a> Hash for Module<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(self.label.as_bytes());
    }
}

impl<'a> Borrow<&'a str> for Module<'a> {
    fn borrow(&self) -> &&'a str {
        &self.label
    }
}

impl<'a> TryFrom<&'a str> for Module<'a> {
    // Using TryFrom rather than FromStr since FromStr does not support the lifetime as far as I can tell
    // This might indicate that I should just be using TryFrom and ignoring the .parse() method entirely

    type Error = Error;

    fn try_from(input: &'a str) -> Result<Self> {
        let (raw_label, raw_outputs) = input.split_once(" -> ").context("Could not split label and outputs")?;
        let kind = ModuleKind::from(raw_label.chars().next().context("no characters")?);
        let mut label_characters = raw_label.chars();
        if kind != ModuleKind::BroadCaster {
            // Broadcaster does not have a starting symbol, but otherwise we need to trim it from the label
            label_characters.next();
        }
        let label = label_characters.as_str();
        let outputs = raw_outputs.split(", ").collect();
        Ok(Self { kind, label, outputs })
    }
}

impl<'a> Module<'a> {
    fn receive(&mut self, pulse: &Pulse<'a>, queue: &mut VecDeque<Pulse<'a>>) {
        assert_eq!(pulse.receiver, self.label, "Signal sent to {} instead of {}", self.label, pulse.receiver);
        match &mut self.kind {
            ModuleKind::BroadCaster => {
                for &output in &self.outputs {
                    queue.push_back(Pulse { sender: self.label, receiver: output, is_high: pulse.is_high })
                }
            }
            ModuleKind::FlipFlop(is_on) => {
                if !pulse.is_high {
                    *is_on = !*is_on;
                    for &output in &self.outputs {
                        queue.push_back(Pulse { sender: self.label, receiver: output, is_high: *is_on })
                    }
                }
            }
            ModuleKind::Conjunction(previous_pulses) => {
                previous_pulses.insert(pulse.sender, pulse.is_high);
                let signal = !previous_pulses.values().all(|&v| v);
                for &output in &self.outputs {
                    queue.push_back(Pulse { sender: self.label, receiver: output, is_high: signal })
                }
            }
        }
    }
}

struct Pulse<'a> {
    sender: &'a str,
    receiver: &'a str,
    is_high: bool,
}

fn push_button<'a>(modules: &mut HashMap<&'a str, Module<'a>>) -> Result<(usize, usize)> {
    clear_queue(&mut [Pulse { sender: "button", receiver: "broadcaster", is_high: false }].into(), modules)
}

/// Processes the pulses in the queue, returning the total (low, high) pulses sent
fn clear_queue<'a>(
    queue: &mut VecDeque<Pulse<'a>>,
    modules: &mut HashMap<&'a str, Module<'a>>,
) -> Result<(usize, usize)> {
    let mut low = 0;
    let mut high = 0;

    while let Some(pulse) = queue.pop_front() {
        if pulse.is_high {
            high += 1;
        } else {
            low += 1;
        }
        // There are some modules (outputs) that don't send any pulses and can be ignored
        if let Some(module) = modules.get_mut(&pulse.receiver) {
            module.receive(&pulse, queue);
        }
    }

    Ok((low, high))
}

fn parse_input<'a>(input: &'a str) -> Result<HashMap<&'a str, Module<'a>>> {
    let mut modules = HashMap::default();
    for l in input.lines() {
        let module: Module = l.try_into()?;
        modules.insert(module.label, module);
    }

    // We need to do further initalization for the conjunction modules
    let mut conjunction_inputs: HashMap<&'a str, Vec<&'a str>> = HashMap::default();
    // Collect the names of the conjunction modules
    for module in modules.values() {
        if let ModuleKind::Conjunction(_) = module.kind {
            conjunction_inputs.insert(module.label, Vec::default());
        }
    }
    // Collect the inputs for each conjunction module
    for module in modules.values() {
        for &output in &module.outputs {
            if let Some(conjunction_module_inputs) = conjunction_inputs.get_mut(output) {
                conjunction_module_inputs.push(module.label);
            }
        }
    }
    // Set a low signal for each conjunction module input
    for (module_name, module_inputs) in conjunction_inputs {
        let module = modules.get_mut(module_name).unwrap();
        match &mut module.kind {
            ModuleKind::Conjunction(input_pulses) => {
                for input in module_inputs {
                    input_pulses.insert(input, false);
                }
            }
            _ => panic!("There was a non-conjunction module {} in the conjunction_inputs map", module_name),
        }
    }

    Ok(modules)
}

pub fn part1(input: &str) -> Result<usize> {
    let mut modules = parse_input(input)?;
    let mut low = 0;
    let mut high = 0;
    for _ in 0..1000 {
        let (l, h) = push_button(&mut modules)?;
        low += l;
        high += h;
    }
    Ok(low * high)
}

pub fn part2(input: &str) -> Result<usize> {
    let modules = parse_input(input)?;
    let [rxin] = modules.values().filter(|m| m.outputs.contains(&"rx")).map(|m| m.label).collect::<Vec<_>>()[..] else {
        panic!("Expected a single input value to rx");
    };
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
";

    const EXAMPLE2: &str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
";

    #[test]
    fn test_example() {
        assert_eq!(part1(EXAMPLE).unwrap(), 32000000);
        assert_eq!(part1(EXAMPLE2).unwrap(), 11687500);
    }

    #[test]
    fn test_solution() {
        let input = std::fs::read_to_string("inputs/day20.txt").unwrap();
        assert_eq!(part1(&input).unwrap(), 747304011);
        assert_eq!(part2(&input).unwrap(), 0);
    }
}
