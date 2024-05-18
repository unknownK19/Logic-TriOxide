use logic_o3::logic::{Circuit, LogicCircuit};

fn main() {
    let or1 = LogicCircuit::new(logic_o3::logic::LogicGate::OR);
    let and1 = LogicCircuit::new(logic_o3::logic::LogicGate::AND);
    let not1 = LogicCircuit::new(logic_o3::logic::LogicGate::NOT);

    let mut circuit01 = Circuit::new();

    circuit01.add_logic_gate(and1);
    circuit01.add_logic_gate(or1);
    circuit01.add_logic_gate(not1);

    circuit01.add_input_connection((false, vec![(0, 0)]));
    circuit01.add_input_connection((true, vec![(0, 1)]));

    circuit01.add_input_connection((true, vec![(1, 0)]));
    circuit01.add_input_connection((false, vec![(1, 1)]));

    circuit01.connection_scheme((0, 2, 0));

    circuit01.add_output_connection(2);
    circuit01.add_output_connection(0);
    circuit01.add_output_connection(1);

    circuit01.update();
    println!("{:?}", circuit01.know_output());
}
