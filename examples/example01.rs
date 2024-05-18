use logic_o3::logic::{Circuit, LogicCircuit};

fn main() {
    let and1 = LogicCircuit::new(logic_o3::logic::LogicGate::AND);
    let not1 = LogicCircuit::new(logic_o3::logic::LogicGate::NOT);

    let mut nand = Circuit::new();

    nand.add_logic_gate(and1);
    nand.add_logic_gate(not1);

    nand.connection_scheme((0, 1, 0));

    nand.add_input_connection((true, vec![(0, 0)]));
    nand.add_input_connection((false, vec![(0, 1)]));

    nand.add_output_connection(1);

    nand.update();
    println!("{:?}", nand.know_output());
}
