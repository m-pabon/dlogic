#[derive(Debug)]
struct Line{
    value: bool
}
impl Line{
    fn new(value: bool) -> Line{
        Line{
            value
        }
    }
}
#[derive(Debug)]
struct Gate {
    gate_type: String,
    input1: Line,
    input2: Line,
    output: Line
}
impl Gate {
    fn new(gate_type: String, input1: Line, input2: Line) -> Self {
        let output = match gate_type.as_str(){
            "AND" => Line::new(input1.value && input2.value),
            "OR" => Line::new(input1.value || input2.value),
            _ => Line::new(false)
        };
        Self{
            gate_type,
            input1,
            input2,
            output
        }
    }
}



fn main() {
    let input_line = Line::new(true);
    let input_line2 = Line::new(false);
    let input_line3 = Line::new(true);
    let input_line4 = Line::new(false);

    let gate = Gate::new(String::from("AND"), input_line, input_line2);
    let gate2 = Gate::new(String::from("OR"), input_line3, input_line4);
    // dbg!(gate);
    // dbg!(gate2);

    let gate3 = Gate::new(String::from("AND"), gate.output, gate2.output);
    dbg!(gate3);
}
