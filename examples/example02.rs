use logic_o3::logic::{Circuit, LogicCircuit};

fn main() {
    let or1 = LogicCircuit::new(logic_o3::logic::LogicGate::OR);
    let not1 = LogicCircuit::new(logic_o3::logic::LogicGate::NOT);

    let mut nor = Circuit::new();

    nor.add_logic_gate(or1);
    nor.add_logic_gate(not1);

    nor.connection_scheme((0, 1, 0));

    nor.add_input_connection((false, vec![(0, 0)]));
    nor.add_input_connection((false, vec![(0, 1)]));

    nor.add_output_connection(1);

    nor.update();
    println!("{:?}", nor.know_output());
}
