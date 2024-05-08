use logicO3::logic::{LogicCircuit, LogicGate};

fn main() {
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
