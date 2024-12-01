use crate::logic::{GateId, GateType};

mod logic {
    #[derive(Debug)]
    pub enum GateType{
        AND,
        OR
    }

    #[derive(Debug, Copy, Clone, PartialEq)]
    enum GateLevel{
        ZERO,
        ONE,
        TWO
    }

    #[derive(Debug, Copy, Clone)]
    pub struct GateId(u8);

    impl GateId {
        fn new(id: u8) -> Self {
            GateId(id)
        }

        pub fn power() -> Self {
            GateId(1)
        }

        pub fn ground() -> Self {
            GateId(0)
        }

        pub fn value(&self) -> u8{
            self.0
        }
    }

    #[derive(Debug)]
    pub struct Gate {
        id: GateId,
        level: GateLevel,
        gate_type: GateType,
        input: (GateId, GateId),
        output: Option<bool>
    }

    impl Gate{
        fn get_level(&self) -> GateLevel {
            self.level
        }
        fn calculate_level(i1: &GateId, i2: &GateId) -> GateLevel {
            match(i1.value() <= 1, i2.value() <= 1){
                (true, true) => GateLevel::ZERO,
                (true, false) => GateLevel::ONE,
                (false, true) => GateLevel::ONE,
                (false, false) => GateLevel::TWO
            }
        }
        fn new(id: GateId, gate_type: GateType, input1: GateId, input2: GateId) -> Self {
            Gate{
                id,
                level: Self::calculate_level(&input1, &input2),
                gate_type,
                input: (input1, input2),
                output: None
            }
        }
    }

    #[derive(Debug)]
    pub struct Circuit{
        connections: Vec<Gate>,
        next_id: u8
    }

    impl Circuit{
        pub fn new() -> Self {
            Circuit {
                connections: Vec::new(),
                next_id: 2
            }
        }
        pub fn add_gate(&mut self, gate_type: GateType, input1: GateId, input2: GateId) -> GateId{
            let gate_id = GateId(self.next_id);
            self.next_id += 1;
            let gate = Gate::new(gate_id, gate_type, input1, input2);
            self.connections.push(gate);
            gate_id
        }

        //TODO: Refactor to hashmap for faster lookups?
        fn find_gate_by_id(&self, gate_id: &GateId) -> Option<&Gate>{
            for gate in &self.connections{
                if gate.id.value() == gate_id.value(){
                    return Some(gate);
                }
            }
            None
        }

        fn extract(&self, gate_id: &GateId) -> bool{
            if gate_id.value() == 0{
                false
            }
            else if gate_id.value() == 1 {
                return true;
            }
            else {
                if let Some(gate) = Self::find_gate_by_id(self, gate_id) {
                    if let Some(output) = gate.output {
                        // Safely use `output` here
                        return output
                    } else {
                        // Handle the case where `output` is `None`
                        panic!("Gate with ID {:?} has no output", gate_id);
                    }
                } else {
                    panic!("Gate with ID {:?} not found", gate_id);
                }
            }
        }
        pub fn evaluate(&mut self) {
            // First collect the level zero inputs
            let mut inputs = Vec::new();
            for gate in &self.connections {
                if gate.level == GateLevel::ZERO{
                    let input1 = self.extract(&gate.input.0);
                    let input2 = self.extract(&gate.input.1);
                    inputs.push((gate.id, input1, input2));
                }
            }

            // Then apply them for level Zero
            for gate in &mut self.connections {
                if gate.level == GateLevel::ZERO{
                    for (eval_id, in1, in2) in &inputs {
                        if eval_id.value() == gate.id.value() {
                            match gate.gate_type {
                                GateType::AND => gate.output = Some(*in1 && *in2),
                                GateType::OR => gate.output = Some(*in1 || *in2)
                            }
                            break;  // Found the match, can stop looking
                        }
                    }
                }
            }

            // Then collect the level One inputs
            for gate in &self.connections {
                if gate.level == GateLevel::ONE{
                    let input1 = self.extract(&gate.input.0);
                    let input2 = self.extract(&gate.input.1);
                    inputs.push((gate.id, input1, input2));
                }
            }

            // Apply for level one
            for gate in &mut self.connections {
                if gate.level == GateLevel::ONE{
                    for (eval_id, in1, in2) in &inputs   {
                        if eval_id.value() == gate.id.value() {
                            match gate.gate_type {
                                GateType::AND => gate.output = Some(*in1 && *in2),
                                GateType::OR => gate.output = Some(*in1 || *in2)
                            }
                            break;
                        }
                    }
                }
            }

            // Then collect the level Two inputs
            for gate in &self.connections {
                if gate.level == GateLevel::TWO{
                    let input1 = self.extract(&gate.input.0);
                    let input2 = self.extract(&gate.input.1);
                    inputs.push((gate.id, input1, input2));
                }
            }

            // Apply for level two
            for gate in &mut self.connections {
                if gate.level == GateLevel::TWO{
                    for (eval_id, in1, in2) in &inputs   {
                        if eval_id.value() == gate.id.value() {
                            match gate.gate_type {
                                GateType::AND => gate.output = Some(*in1 && *in2),
                                GateType::OR => gate.output = Some(*in1 || *in2)
                            }
                            break;
                        }
                    }
                }
            }
        }
    }
}


fn main() {
    use logic::{Circuit, Gate};

    let mut circuit = Circuit::new();
    //Level ZERO Gate
    let gate1= circuit.add_gate(GateType::AND, GateId::power(), GateId::ground());
    //Level ONE Gate

    let gate2 = circuit.add_gate(GateType::OR, gate1, GateId::power());
    //Level TWO Gate
    circuit.add_gate(GateType::AND, gate2, gate1);

    circuit.evaluate();


    println!("Circuit: {circuit:#?}")
}