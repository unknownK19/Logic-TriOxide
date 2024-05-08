use core::panic;
use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    ops::Deref,
    rc::Rc,
};

struct Circuit {
    component: Vec<LogicCircuit>,
    connection_path: Vec<(LogicCircuit, LogicCircuit, usize)>, // (from, to, to_input_index)
                                                               //power: Vec<Rc<RefCell<bool>>>,
                                                               // TODO
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

pub fn test01() {
    // create AND gate
    println!("*AND Gate*");
    let mut and1 = LogicCircuit::new(LogicGate::AND);
    println!("{}", and1.to_string());

    and1.change_input_config(0, true);
    println!("{}", and1.to_string());

    and1.change_input_config(0, false);
    and1.change_input_config(1, true);
    println!("{}", and1.to_string());

    and1.change_input_config(0, true);
    println!("{}", and1.to_string());

    //create NOT gate
    println!("*NOT Gate*");
    let mut not1 = LogicCircuit::new(LogicGate::NOT);
    println!("{}", not1.to_string());

    and1.connect_head_to(&mut not1, 0);

    println!("{}  -connect-> {}", and1.to_string(), not1.to_string());

    // create OR Gate
    let mut or1 = LogicCircuit::new(LogicGate::OR);
    println!("{}", or1.to_string());
    or1.change_input_config(0, true);
    println!("{}", or1.to_string());
    and1.change_input_config(0, false);
    or1.connect_head_to(&mut and1, 0);

    println!(
        "{} -connect-> {}  -connect-> {}",
        or1.to_string(),
        and1.to_string(),
        not1.to_string()
    );
    or1.change_input_config(0, false);
    and1.update();
    not1.update();
    println!(
        "{} -connect-> {}  -connect-> {}",
        or1.to_string(),
        and1.to_string(),
        not1.to_string()
    );
}
