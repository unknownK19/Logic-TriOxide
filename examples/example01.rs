use logic_o3::logic::{Circuit, LogicCircuit};

fn main() {
    // Create new Logic Gate
    let and1 = LogicCircuit::new(logic_o3::logic::LogicGate::AND);
    let not1 = LogicCircuit::new(logic_o3::logic::LogicGate::NOT);

    // Create Empty Circuit
    let mut nand = Circuit::new();

    // Add Two Logic Gate AKA Component
    nand.add_logic_gate(and1);
    nand.add_logic_gate(not1);

    // Connect wire between two component (from_comp_id, to_comp_id, to_comp_input_index)
    nand.connection_scheme((0, 1, 0));

    // Add Two Input Signal
    nand.add_input_connection((true, vec![(0, 0)]));
    nand.add_input_connection((false, vec![(0, 1)]));

    // Add One Output Signal
    nand.add_output_connection(1);

    // Refreshing for change
    nand.update();

    // And Done
    println!("{:?}", nand.know_output());
}
