use fancy_regex::Regex;
use std::collections::HashMap;

#[derive(Copy, Debug, Clone)]
enum Operation<'a> {
    Assign(u32, &'a str),
    Transfer(&'a str, &'a str),
    And(&'a str, &'a str, &'a str),
    AndSingle(u32, &'a str, &'a str),
    Or(&'a str, &'a str, &'a str),
    LShift(&'a str, u32, &'a str),
    RShift(&'a str, u32, &'a str),
    Not(&'a str, &'a str),
    Undefined,
}

impl<'a> From<&'a str> for Operation<'a> {
    fn from(text: &'a str) -> Self {
        if let Ok(Some(captures)) = Regex::new(r"^(\d+)\s+->\s+(\w+)$").unwrap().captures(text) {
            let signal: u32 = captures.get(1).unwrap().as_str().parse().unwrap();
            let wire = captures.get(2).unwrap().as_str();

            Operation::Assign(signal, wire)
        } else if let Ok(Some(captures)) =
            Regex::new(r"^(\w+)\s+->\s+(\w+)$").unwrap().captures(text)
        {
            let input = captures.get(1).unwrap().as_str();
            let output = captures.get(2).unwrap().as_str();

            Operation::Transfer(input, output)
        } else if let Ok(Some(captures)) = Regex::new(r"^(\d+)\s+AND\s+(\w+)\s+->\s+(\w+)$")
            .unwrap()
            .captures(text)
        {
            let val: u32 = captures.get(1).unwrap().as_str().parse().unwrap();
            let var = captures.get(2).unwrap().as_str();
            let out = captures.get(3).unwrap().as_str();

            Operation::AndSingle(val, var, out)
        } else if let Ok(Some(captures)) = Regex::new(r"^(\w+)\s+AND\s+(\w+)\s+->\s+(\w+)$")
            .unwrap()
            .captures(text)
        {
            let a = captures.get(1).unwrap().as_str();
            let b = captures.get(2).unwrap().as_str();
            let c = captures.get(3).unwrap().as_str();

            Operation::And(a, b, c)
        } else if let Ok(Some(captures)) = Regex::new(r"^(\w+)\s+OR\s+(\w+)\s+->\s+(\w+)$")
            .unwrap()
            .captures(text)
        {
            let a = captures.get(1).unwrap().as_str();
            let b = captures.get(2).unwrap().as_str();
            let c = captures.get(3).unwrap().as_str();

            Operation::Or(a, b, c)
        } else if let Ok(Some(captures)) = Regex::new(r"^(\w+)\s+LSHIFT\s+(\d+)\s+->\s+(\w+)$")
            .unwrap()
            .captures(text)
        {
            let input = captures.get(1).unwrap().as_str();
            let shift: u32 = captures.get(2).unwrap().as_str().parse().unwrap();
            let output = captures.get(3).unwrap().as_str();

            Operation::LShift(input, shift, output)
        } else if let Ok(Some(captures)) = Regex::new(r"^(\w+)\s+RSHIFT\s+(\d+)\s+->\s+(\w+)$")
            .unwrap()
            .captures(text)
        {
            let input = captures.get(1).unwrap().as_str();
            let shift: u32 = captures.get(2).unwrap().as_str().parse().unwrap();
            let output = captures.get(3).unwrap().as_str();

            Operation::RShift(input, shift, output)
        } else if let Ok(Some(captures)) = Regex::new(r"^NOT\s+(\w+)\s+->\s+(\w+)$")
            .unwrap()
            .captures(text)
        {
            let input = captures.get(1).unwrap().as_str();
            let output = captures.get(2).unwrap().as_str();

            Operation::Not(input, output)
        } else {
            Operation::Undefined
        }
    }
}

struct Circuit<'a> {
    // Actual values
    wires: HashMap<&'a str, u32>,
    // Mappings
    operations: HashMap<&'a str, Operation<'a>>,
}

impl<'a> Circuit<'a> {
    fn new() -> Self {
        Circuit {
            wires: HashMap::new(),
            operations: HashMap::new(),
        }
    }

    fn clear_cache(&mut self) {
        self.wires.clear();
    }

    fn add(&mut self, operation: Operation<'a>) {
        match operation {
            // A macro could work here
            Operation::Assign(_, wire) => {
                self.operations.insert(wire, operation);
            }
            Operation::Transfer(_, to) => {
                self.operations.insert(to, operation);
            }
            Operation::And(_, _, out) => {
                self.operations.insert(out, operation);
            }
            Operation::AndSingle(_, _, out) => {
                self.operations.insert(out, operation);
            }
            Operation::Or(_, _, out) => {
                self.operations.insert(out, operation);
            }
            Operation::LShift(_, _, output) => {
                self.operations.insert(output, operation);
            }
            Operation::RShift(_, _, output) => {
                self.operations.insert(output, operation);
            }
            Operation::Not(_, output) => {
                self.operations.insert(output, operation);
            }
            Operation::Undefined => {}
        }
    }

    fn signal(&mut self, wire: &'a str) -> u32 {
        if let Some(signal) = self.wires.get(wire) {
            return *signal;
        }

        //Not found, start searching
        let op = self.operations[wire];

        match op {
            Operation::Assign(signal, _) => {
                self.wires.insert(wire, signal);

                signal
            }
            Operation::Transfer(from, _) => {
                let res = self.signal(from);
                self.wires.insert(wire, res);

                res
            }
            Operation::And(a, b, _) => {
                let a = self.signal(a);
                let b = self.signal(b);

                self.wires.insert(wire, a & b);

                a & b
            }
            Operation::AndSingle(signal, input, _) => {
                let input = self.signal(input);
                self.wires.insert(wire, signal & input);

                signal & input
            }
            Operation::Or(a, b, _) => {
                let a = self.signal(a);
                let b = self.signal(b);

                self.wires.insert(wire, a | b);

                a | b
            }
            Operation::LShift(input, shift, _) => {
                let res = self.signal(input) << shift;
                self.wires.insert(wire, res);

                res
            }
            Operation::RShift(input, shift, _) => {
                let res = self.signal(input) >> shift;
                self.wires.insert(wire, res);

                res
            }
            Operation::Not(input, _) => {
                let res = !self.signal(input);
                self.wires.insert(wire, res);

                res
            }
            Operation::Undefined => 0,
        }
    }
}

//Multiline input
pub fn puzzle1(input: &str) -> u32 {
    let mut circuit = Circuit::new();

    input.lines().for_each(|line| {
        let op: Operation = line.into();
        circuit.add(op);
    });

    circuit.signal("a")
}

pub fn puzzle2(input: &str) -> u32 {
    let mut circuit = Circuit::new();

    input.lines().for_each(|line| {
        let op: Operation = line.into();
        circuit.add(op);
    });

    let val = circuit.signal("a");
    circuit.clear_cache();
    circuit.wires.insert("b", val);

    circuit.signal("a")
}
