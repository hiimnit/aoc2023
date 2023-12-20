use std::{
    collections::{HashMap, VecDeque},
    env, fs,
};

use num::Integer;

fn main() {
    let mut args = env::args();
    args.next();
    let input_file_path = args
        .next()
        .expect("Expected input file path as first argument");

    let input = fs::read_to_string(&input_file_path)
        .expect(&format!("Could not open input file {input_file_path}"));

    let mut modules: HashMap<String, Module> = HashMap::new();

    for line in input.lines() {
        let mut line = line.split(" -> ");

        let module_name = line.next().expect("Expected module name before ->");
        let outputs = line.next().expect("Expected outputs name after ->");
        assert!(line.next().is_none());

        let (module_name, module_type) = if module_name.starts_with('%') {
            (
                module_name.chars().skip(1).collect(),
                ModuleType::FlipFlop(FlipFlopModule::new()),
            )
        } else if module_name.starts_with('&') {
            (
                module_name.chars().skip(1).collect(),
                ModuleType::Conjunction(ConjunctionModule::new()),
            )
        } else if module_name == "broadcaster" {
            (module_name.to_string(), ModuleType::Broadcast)
        } else {
            panic!("Unexpected module name {module_name}");
        };

        let module = Module::new(module_name.clone(), module_type, outputs);

        modules.insert(module_name, module);
    }

    let mut inputs: HashMap<String, Vec<String>> = HashMap::new();

    for (module_name, module) in &modules {
        for output in &module.outputs {
            if let Some(inputs) = inputs.get_mut(output) {
                inputs.push(module_name.clone());
            } else {
                inputs.insert(output.clone(), vec![module_name.clone()]);
            }
        }
    }

    for (module_name, inputs) in inputs {
        for input in inputs {
            modules.get_mut(&module_name).map(|v| v.add_input(input));
        }
    }

    let (low_pulses, high_pulses) = count_pulses(modules.clone());

    let part_1_result = low_pulses * high_pulses;
    println!("Part 1 result {part_1_result}");

    // only input of rx is a conjunction bn
    // bn has four inputs, each from a separate section of the graph (see graph.png)
    // find high pulses from these four sections into bn => lcm

    let part_2_result = 1
        .lcm(&count_button_presses(
            modules.clone(),
            ("pl", "bn", &Pulse::High),
        ))
        .lcm(&count_button_presses(
            modules.clone(),
            ("lz", "bn", &Pulse::High),
        ))
        .lcm(&count_button_presses(
            modules.clone(),
            ("mz", "bn", &Pulse::High),
        ))
        .lcm(&count_button_presses(
            modules.clone(),
            ("zm", "bn", &Pulse::High),
        ));

    println!("Part 2 result {part_2_result}");
}

fn count_pulses(mut modules: HashMap<String, Module>) -> (i32, i32) {
    let mut low_pulses = 0;
    let mut high_pulses = 0;

    let mut queue: VecDeque<_> = VecDeque::new();

    for _ in 0..1_000 {
        queue.push_back(("button".to_string(), "broadcaster".to_string(), Pulse::Low));

        while let Some((from, to, pulse)) = queue.pop_front() {
            match pulse {
                Pulse::Low => low_pulses += 1,
                Pulse::High => high_pulses += 1,
            }

            if let Some(module) = modules.get_mut(&to) {
                let next_pulses = module.send_pulse(&from, pulse);
                queue.extend(next_pulses);
            };
        }
    }

    (low_pulses, high_pulses)
}

fn count_button_presses(
    mut modules: HashMap<String, Module>,
    (find_from, find_to, find_pulse): (&str, &str, &Pulse),
) -> i64 {
    let mut queue: VecDeque<_> = VecDeque::new();
    let mut button_presses = 0;

    loop {
        button_presses += 1;

        queue.push_back(("button".to_string(), "broadcaster".to_string(), Pulse::Low));

        while let Some((from, to, pulse)) = queue.pop_front() {
            if from == find_from && to == find_to && pulse == *find_pulse {
                return button_presses;
            }
            if let Some(module) = modules.get_mut(&to) {
                let next_pulses = module.send_pulse(&from, pulse);
                queue.extend(next_pulses);
            };
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Module {
    name: String,
    module_type: ModuleType,
    outputs: Vec<String>,
}

impl Module {
    fn new(name: String, module_type: ModuleType, outputs: &str) -> Self {
        Self {
            name,
            module_type,
            outputs: outputs.split(", ").map(|e| e.to_string()).collect(),
        }
    }

    fn add_input(&mut self, input: String) {
        match &mut self.module_type {
            ModuleType::Conjunction(conjunction) => {
                conjunction.state.insert(input.to_string(), Pulse::Low);
            }
            ModuleType::Broadcast | ModuleType::FlipFlop(_) => {}
        }
    }

    fn send_pulse(
        &mut self,
        from_module_name: &String,
        pulse: Pulse,
    ) -> Vec<(String, String, Pulse)> {
        let next_pulse = match &mut self.module_type {
            ModuleType::Broadcast => pulse,
            ModuleType::FlipFlop(flip_flop) => {
                let Some(next_pulse ) = flip_flop.handle_pulse(pulse) else {
                    return vec![];
                };
                next_pulse
            }
            ModuleType::Conjunction(conjunction) => {
                conjunction.handle_pulse(from_module_name, pulse)
            }
        };

        self.outputs
            .iter()
            .map(|e| (self.name.clone(), e.clone(), next_pulse))
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum ModuleType {
    Broadcast,
    FlipFlop(FlipFlopModule),
    Conjunction(ConjunctionModule),
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct FlipFlopModule {
    state: bool,
}

impl FlipFlopModule {
    fn new() -> Self {
        Self { state: false }
    }

    fn handle_pulse(&mut self, pulse: Pulse) -> Option<Pulse> {
        match pulse {
            Pulse::High => None,
            Pulse::Low => {
                self.state = !self.state;
                if self.state {
                    Some(Pulse::High)
                } else {
                    Some(Pulse::Low)
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct ConjunctionModule {
    state: HashMap<String, Pulse>,
}

impl ConjunctionModule {
    fn new() -> Self {
        Self {
            state: HashMap::new(),
        }
    }

    fn handle_pulse(&mut self, from_module_name: &String, pulse: Pulse) -> Pulse {
        self.state.insert(from_module_name.clone(), pulse);

        if self.state.values().all(|e| *e == Pulse::High) {
            Pulse::Low
        } else {
            Pulse::High
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Pulse {
    Low,
    High,
}
