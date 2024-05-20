use core::panic;
use std::{cell::RefCell, rc::Rc};

pub struct Circuit {
    component: Vec<LogicCircuit>,                // Vec[and1, or1, not1]
    connection_path: Vec<(usize, usize, usize)>, // Vev[(from_comp_id, to_comp_id, to_input_index)]
    input: Vec<Rc<RefCell<(bool, Vec<(usize, usize)>)>>>, // Vec[(Input_Binary , Vec[(to_id, to_input_index)])]
    output: Vec<Rc<RefCell<(Vec<usize>, bool)>>>,         // Vec[(from_id, Output_Binary)]
                                                          // update method require for change output
}

impl Circuit {
    /// Create Circuit
    pub fn new() -> Self {
        Self {
            component: vec![],
            connection_path: vec![],
            input: vec![],
            output: vec![],
        }
    }
    /// Add component AKA LogicCircuit
    pub fn add_logic_gate(&mut self, new_comp: LogicCircuit) {
        self.component.push(new_comp)
    }
    /// Connect Between Component
    pub fn connection_scheme(&mut self, connection: (usize, usize, usize)) {
        if self.component.len() < connection.0 {
            dbg!(println!(
                "WARNING: According your instructed FROM_ID is not Exist"
            ))
        }
        self.connection_path.push(connection)
    }
    /// Add Input on Circuit
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
    /// Add Input with connection
    pub fn add_input_connection(&mut self, connection: (bool, Vec<(usize, usize)>)) {
        self.input.push(Rc::new(RefCell::new(connection)))
    }
    /// Change Input Signal
    pub fn change_input_signal(&mut self, input_index: usize, change_to: bool) {
        match *(*self.input[input_index]).borrow_mut() {
            (ref mut signal, _) => *signal = change_to,
        }
    }
    /// Add Input config to attach component AKA wiring between LogicCircuit and Input Signal
    pub fn change_input_config(
        &mut self,
        connection: (bool, Vec<(usize, usize)>),
        input_index: usize,
    ) {
        if self.input.len() < input_index {
            dbg!(println!(
                "WARNING: according your instructed input index is out of Bound"
            ))
        } else {
            *(*self.input[input_index]).borrow_mut() = connection
        }
    }
    /// Add Output Signal to know Output.
    pub fn add_output_connection(&mut self, comp_id: usize) {
        self.output
            .push(Rc::new(RefCell::new((vec![comp_id], false))))
    }
    /// Adding more comp on specific Output
    pub fn add_comp_onoutput(&mut self, comp_id: usize, output_index: usize) {
        match *(*self.output[output_index]).borrow_mut() {
            (ref mut x, _) => x.push(comp_id),
        }
    }
    /// Know Number of output
    pub fn know_no_output(&self) -> usize {
        self.output.len()
    }
    /// Know Output on Specific component
    pub fn know_output_comp(&self, comp_id: usize) -> bool {
        *(*self.component[comp_id].output).borrow_mut()
    }
    /// Know Output of Circuit
    pub fn know_output(&mut self) -> Vec<bool> {
        let mut out = vec![];
        for out_each in self.output.clone() {
            match *(*out_each).borrow_mut() {
                (_, output) => out.push(output),
            }
        }
        out
    }
    /// Refresh The Cicuit to update Output
    pub fn update(&mut self) {
        let mut count = 0;
        {
            while count != self.input.len() {
                match &(*(*self.input[count]).borrow_mut()) {
                    (x, y) => {
                        // KNOW: x is input boolean and y is input connection
                        for i in y {
                            // KNOW: i has two value tuples (id_component, index_component)
                            self.component[i.0].change_input_config(i.1, *x);
                            self.component[i.0].update()
                        }
                    }
                }
                count += 1;
            }
            count = 0;
        }
        {
            while count != self.connection_path.len() {
                match self.connection_path[count] {
                    (from_comp, to_comp, to_pin_index) => {
                        self.component[to_comp].input[to_pin_index] =
                            Rc::clone(&self.component[from_comp].output);
                        self.component[to_comp].update();
                        // Due to borrowing Conflict
                        // self.component[from_comp]
                        //     .connect_head_to(&mut self.component[to_comp], to_pin_index);
                    }
                }
                count += 1
            }
            count = 0
        }
        {
            while count != self.output.len() {
                match *(*self.output[count]).borrow_mut() {
                    (ref mut x, ref mut out) => {
                        let mut output = false;
                        for by_comp in x {
                            self.component[*by_comp].update();
                            output = output || self.know_output_comp(*by_comp);
                        }
                        *out = output
                    }
                }
                count += 1
            }
        }
    }
    //TODO: Add removing specific comp also destroy connected connection
}

pub enum LogicGate {
    AND, // AND Gate
    NOT, // NOT Gate
    OR,  // OR Gate
}

pub struct LogicCircuit {
    input: Vec<Rc<RefCell<bool>>>, // Input pin indexing like [true, false, false, false]
    gate_type: LogicGate,          // Which Logic Gate
    output: Rc<RefCell<bool>>,     // Single output pin
}
/// To Know LogicCircuit Current supply.
impl ToString for LogicCircuit {
    fn to_string(&self) -> String {
        use LogicGate::{AND, NOT, OR};
        let mut input = String::new();
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
    /// Create LogicCircuit
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
    /// Add LogicCircuit with number of pins
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
    /// To update the LogicCircuit
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
                0 | 1 => dbg!(println!("Zero and Single input doesn't exist in AND Gate")),
                _ => {
                    let mut step = 0;
                    while self.input.len() != step + 1 {
                        *(*self.output).borrow_mut() = input(step) && input(step + 1);
                        step += 1
                    }
                }
            },
            OR => match input_len {
                0 | 1 => dbg!(println!("Zero and Single input doesn't exist in AND Gate")),
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
    // Connect self output pin to other input gate pin
    // KNOW: `self` is own LogicCircuit where `other` is other LogicCircuit where`index` represent
    // other gate input pin index
    #[deprecated]
    pub fn connect_head_to(&mut self, other: &mut Self, index: usize) {
        other.input[index] = Rc::clone(&self.output);
        self.update();
        other.update()
    }
}
