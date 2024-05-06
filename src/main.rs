use logic::logic::LogicCircut;

mod logic;
fn main() {
    let and1 = LogicCircut::new_with_pins(logic::logic::LogicGate::AND, 3);
    println!("{}", and1.to_string())
}
