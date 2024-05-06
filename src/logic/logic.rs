use core::panic;
use std::{rc::Rc, sync::Arc};

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
                true => " ",
                _ => " ",
            })
        }
        match self.gate_type {
            NOT => {
                format!("{}󰣤 {}", input, {
                    if self.output {
                        ""
                    } else {
                        ""
                    }
                })
            }
            AND => {
                format!("{}{}󰣡 {}", input, self.input.len(), {
                    if self.output {
                        ""
                    } else {
                        ""
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
            _ => LogicCircut {
                input: vec![Rc::new(false), Rc::new(false)],
                gate_type: logic_type,
                output: false,
            },
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
                _ => LogicCircut {
                    input: vec![Rc::new(false); number],
                    gate_type: logic_type,
                    output: false,
                },
            },
        }
    }
    pub fn update(&self) {}
}
