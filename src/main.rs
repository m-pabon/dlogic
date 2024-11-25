#[derive(Debug)]
enum GateType{
    AND,
    OR
}

#[derive(Debug)]
struct Gate {
    gate_type: GateType,
    pin1: bool,
    pin2: bool
}


impl Gate{
    fn new(gate_type: GateType, pin1: bool, pin2: bool) -> Self{
        Gate{
            gate_type,
            pin1,
            pin2
        }
    }
    fn evaluate(&self) -> bool{
        match self.gate_type {
            GateType::AND => self.pin1 && self.pin2,
            GateType::OR => self.pin1 || self.pin2
        }
    }
}

fn main() {
    let gate1 = Gate::new(GateType::AND, true, false);
    let gate1_out = gate1.evaluate();

    let gate2 = Gate::new(GateType::OR, gate1_out, false);
    let gate2_out = gate2.evaluate();


    println!("Gate 1: {gate1:#?}");
    println!("Gate 2: {gate2:#?}");
    println!("Circuit Result: {gate2_out}");
}