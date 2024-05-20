use logic_o3::logic::{Circuit, LogicCircuit};

fn main() {
    // Create new Logic Gate
    let or1 = LogicCircuit::new(logic_o3::logic::LogicGate::OR);
    let not1 = LogicCircuit::new(logic_o3::logic::LogicGate::NOT);

    // Create Empty Circuit
    let mut nor = Circuit::new();

    // Add Two Logic Gate AKA Component
    nor.add_logic_gate(or1);
    nor.add_logic_gate(not1);

    // Connect wire between two component (from_comp_id, to_comp_id, to_comp_input_index)
    nor.connection_scheme((0, 1, 0));

    // Add Two Input Signal
    nor.add_input_connection((false, vec![(0, 0)]));
    nor.add_input_connection((false, vec![(0, 1)]));

    // Add One Output Signal
    nor.add_output_connection(1);

    // Refreshing for change
    nor.update();

    // And Done
    println!("{:?}", nor.know_output());
}
