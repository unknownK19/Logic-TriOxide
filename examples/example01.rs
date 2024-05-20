use logic_o3::logic::{Circuit, LogicCircuit};

fn main() {
    // Add Two Logic Gate
    let and1 = LogicCircuit::new(logic_o3::logic::LogicGate::AND);
    let not1 = LogicCircuit::new(logic_o3::logic::LogicGate::NOT);
    // Create New Circuit
    let mut nand = Circuit::new();
    // Insert Two Logic Gate
    nand.add_logic_gate(and1);
    nand.add_logic_gate(not1);
    // Connect wire between Two Gate
    nand.connection_scheme((0, 1, 0));
    // Add Input Signal on Circuit
    nand.add_input_connection((true, vec![(0, 0)]));
    nand.add_input_connection((false, vec![(0, 1)]));
    // Add Output Signal to know Output of this Circuit
    nand.add_output_connection(1);
    // Update Cicuit in According to Input
    nand.update();
    // TADA !!
    println!("{:?}", nand.know_output());
}
