enum LogicGate {
    AND(usize), // Number of Input
                /*More TODO*/
}

impl LogicGate {
    pub fn output(&self, input: Vec<bool>) -> Result<bool, String> {
        use self::LogicGate::AND;
        match self {
            AND(x) => {
                let no_input_gate = *x;
                if no_input_gate == 1 {
                    Err("That's Doesn't make sence if AND Gate have single input".to_string())
                } else if no_input_gate == input.len() {
                    let mut i = 0;
                    let mut two_result = true;
                    while i != no_input_gate {
                        two_result = two_result && input[i];
                        i += 1;
                    }
                    Ok(two_result)
                } else {
                    Err("Wrong Input length".to_string())
                }
            }
        }
    }
}

fn main() {
    let and1 = LogicGate::AND(3);
    println!("AND of 011 {:?}", and1.output(vec![false, true, true]));
    println!("AND of 111 {:?}", and1.output(vec![true, true, true]));
    println!(
        "AND of 0110 {:?}",
        and1.output(vec![false, true, true, false])
    );
}
