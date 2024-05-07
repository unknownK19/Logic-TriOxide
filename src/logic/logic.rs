use core::panic;
use std::rc::Rc;

pub enum LogicGate {
    AND, // AND Gate
    NOT,
    /*More TODO*/
}

pub struct LogicCircut {
    input: Vec<Rc<bool>>,
    gate_type: LogicGate, // Which Logic Gate
    output: bool,
}

impl ToString for LogicCircut {
    fn to_string(&self) -> String {
        use LogicGate::{AND, NOT};
        let mut input = String::new();
        for i in 1..=self.input.len() {
            input.push_str(match *self.input[i - 1] {
                true => "\x1b[32m \x1b[0m",
                _ => "\x1b[31m \x1b[0m",
            })
        }
        match self.gate_type {
            NOT => {
                format!("{}󰣤 {}", input, {
                    if self.output {
                        "\x1b[32m\x1b[0m"
                    } else {
                        "\x1b[31m\x1b[0m"
                    }
                })
            }
            AND => {
                format!("{}{}󰣡 {}", input, self.input.len(), {
                    if self.output {
                        "\x1b[32m\x1b[0m"
                    } else {
                        "\x1b[31m\x1b[0m"
                    }
                })
            }
        }
    }
}

impl LogicCircut {
    pub fn new(logic_type: LogicGate) -> Self {
        match logic_type {
            LogicGate::NOT => LogicCircut {
                input: vec![Rc::new(false)],
                gate_type: logic_type,
                output: true,
            },
            _ => {
                let mut sample_gate = LogicCircut {
                    input: vec![Rc::new(false), Rc::new(false)],
                    gate_type: logic_type,
                    output: false,
                };
                sample_gate.update();
                sample_gate
            }
        }
    }
    pub fn new_with_pins(logic_type: LogicGate, number: usize) -> Self {
        match number {
            0 | 1 => panic!("Zero or One It sould not Exist"),
            _ => match logic_type {
                LogicGate::NOT => panic!(
                    "NOT Gate Should have only single input pin 
                \n So you can try new instead of new_with_pins"
                ),
                _ => {
                    let mut sample_gate = LogicCircut {
                        input: vec![Rc::new(false); number],
                        gate_type: logic_type,
                        output: false,
                    };
                    sample_gate.update();
                    sample_gate
                }
            },
        }
    }
    pub fn update(&mut self) {
        use LogicGate::{AND, NOT};
        let input = |index: usize| *self.input[index];
        match self.gate_type {
            NOT => match self.input.len() {
                1 => self.output = !input(0),
                _ => panic!("Wrong Input Config"),
            },
            AND => match self.input.len() {
                0 | 1 => panic!("Zero and Single input doesn't exist in AND Gate"),
                _ => {
                    let mut step = 0;
                    while self.input.len() != step + 1 {
                        self.output = input(step) && input(step + 1);
                        step += 1
                    }
                }
            },
        }
    }
    pub fn change_input_config(&mut self, index: usize, change_to: bool) {
        self.input[index] = Rc::new(change_to);
        self.update()
    }
}

pub fn run_example_1() {
    // create AND gate
    println!("*AND Gate*");
    let mut and1 = LogicCircut::new(LogicGate::AND);
    println!("{}", and1.to_string());

    and1.change_input_config(0, true);
    println!("{}", and1.to_string());

    and1.change_input_config(0, false);
    and1.change_input_config(1, true);
    println!("{}", and1.to_string());

    and1.change_input_config(0, true);
    println!("{}", and1.to_string());

    println!("*NOT Gate*");
    let mut not1 = LogicCircut::new(LogicGate::NOT);
    println!("{}", not1.to_string());

    not1.change_input_config(0, true);
    println!("{}", not1.to_string());
}

pub fn run_example_2() {
    for i in 3..=7 {
        println!("AND Gate with {}", i);
        let and2 = LogicCircut::new_with_pins(LogicGate::AND, i);
        println!("{}", and2.to_string())
    }
}
