use core::panic;
use std::{cell::RefCell, rc::Rc};

pub struct Circuit {
    component: Vec<LogicCircuit>,                // Vec[and1, or1, not1]
    connection_path: Vec<(usize, usize, usize)>, // Vev[(from_id, to_id, to_input_index)]
    input: Vec<Rc<RefCell<(bool, Vec<(usize, usize)>)>>>, // Vec[(Input_Binary , Vec[(to_id, to_input_index)])]
    output: Vec<Rc<RefCell<(Vec<usize>, bool)>>>,         // Vec[(from_id, Output_Binary)]
                                                          // update method require for change output
}

impl Circuit {
    pub fn new() -> Self {
        Self {
            component: vec![],
            connection_path: vec![],
            input: vec![],
            output: vec![],
        }
    }
    pub fn add_logic_gate(&mut self, new_comp: LogicCircuit) {
        self.component.push(new_comp)
    }
    pub fn connect_scheme(&mut self, connection: (usize, usize, usize)) {
        self.connection_path.push(connection)
    }
    pub fn add_input(&mut self, no_of_input: usize) {
        match no_of_input {
            0 => dbg!(println!("WARNING: Number of Input Cannot be Zero")),
            _ => {
                for _ in 1..=no_of_input {
                    self.input.push(Rc::new(RefCell::new((false, vec![]))))
                }
            }
        }
    }
    //TODO
}

pub enum LogicGate {
    AND, // AND Gate
    NOT,
    OR,
    /*More TODO*/
}

pub struct LogicCircuit {
    input: Vec<Rc<RefCell<bool>>>,
    gate_type: LogicGate, // Which Logic Gate
    output: Rc<RefCell<bool>>,
}

impl ToString for LogicCircuit {
    fn to_string(&self) -> String {
        use LogicGate::{AND, NOT, OR};
        let mut input = String::new();
        // let input01 = *self.input[0].deref().borrow();
        for i in 1..=self.input.len() {
            input.push_str(match *(*self.input[i - 1]).borrow() {
                true => "\x1b[32m \x1b[0m",
                _ => "\x1b[31m \x1b[0m",
            })
        }
        match self.gate_type {
            NOT => {
                format!("{}󰣤 {}", input, {
                    if *(*self.output).borrow() {
                        "\x1b[32m\x1b[0m"
                    } else {
                        "\x1b[31m\x1b[0m"
                    }
                })
            }
            AND => {
                format!("{}{}󰣡 {}", input, self.input.len(), {
                    if *(*self.output).borrow() {
                        "\x1b[32m\x1b[0m"
                    } else {
                        "\x1b[31m\x1b[0m"
                    }
                })
            }
            OR => {
                format!("{}{}󰣥 {}", input, self.input.len(), {
                    if *(*self.output).borrow() {
                        "\x1b[32m\x1b[0m"
                    } else {
                        "\x1b[31m\x1b[0m"
                    }
                })
            }
        }
    }
}

impl LogicCircuit {
    pub fn new(logic_type: LogicGate) -> Self {
        match logic_type {
            LogicGate::NOT => LogicCircuit {
                input: vec![Rc::new(RefCell::new(false))],
                gate_type: logic_type,
                output: RefCell::new(true).into(),
            },
            _ => {
                let mut sample_gate = LogicCircuit {
                    input: vec![Rc::new(RefCell::new(false)); 2],
                    gate_type: logic_type,
                    output: RefCell::new(false).into(),
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
                    let mut sample_gate = LogicCircuit {
                        input: vec![Rc::new(RefCell::new(false)); number],
                        gate_type: logic_type,
                        output: RefCell::new(false).into(),
                    };
                    sample_gate.update();
                    sample_gate
                }
            },
        }
    }
    pub fn update(&mut self) {
        use LogicGate::{AND, NOT, OR};
        let input = |index: usize| *(*self.input[index]).borrow_mut();
        let input_len = self.input.len();
        match self.gate_type {
            NOT => match input_len {
                1 => *(*self.output).borrow_mut() = !input(0),
                _ => panic!("Wrong Input Config"),
            },
            AND => match input_len {
                0 | 1 => panic!("Zero and Single input doesn't exist in AND Gate"),
                _ => {
                    let mut step = 0;
                    while self.input.len() != step + 1 {
                        *(*self.output).borrow_mut() = input(step) && input(step + 1);
                        step += 1
                    }
                }
            },
            OR => match input_len {
                0 | 1 => panic!("Zero and Single input doesn't exist in AND Gate"),
                _ => {
                    let mut step = 0;
                    while self.input.len() != step + 1 {
                        *(*self.output).borrow_mut() = input(step) || input(step + 1);
                        step += 1
                    }
                }
            },
        }
    }
    pub fn change_input_config(&mut self, index: usize, change_to: bool) {
        self.input[index] = Rc::new(change_to.into());
        self.update()
    }
    pub fn connect_head_to(&mut self, other: &mut Self, index: usize) {
        other.input[index] = Rc::clone(&self.output);
        self.update();
        other.update()
    }
}
