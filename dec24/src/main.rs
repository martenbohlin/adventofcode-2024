use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;
use std::rc::Rc;

fn main() {
    let (wires, gates) = read_file(env::args().collect::<Vec<String>>()[1].clone());
    part1(&wires, &gates);
}

fn part1(wires: &HashMap<String, Rc<RefCell<Wire>>>, gates: &Vec<Gate>) {
    let mut z_wires: Vec<Rc<RefCell<Wire>>> = Vec::new();
    for wire in wires.values() {
        if wire.borrow().name.starts_with("z") {
            z_wires.push(wire.clone());
        }
    }
    z_wires.sort_by(|a, b| b.borrow().name.cmp(&a.borrow().name));
    let mut result = 0_i64;
    for wire in &z_wires {
        let val = calculate_wire(wire.clone());
        result <<= 1;
        if val {
            result += 1;
        }
        println!("Wire: {:?} {:?}, {}", wire.borrow().name, val, result);
    }
    println!("Part 1: {:?}", result);
}

fn calculate_wire(wire: Rc<RefCell<Wire>>) -> bool {
    if wire.borrow().value.is_some() {
        return wire.borrow().value.unwrap();
    }
    let value = calculate_gate(&wire.borrow().input.as_ref().unwrap());
    wire.borrow_mut().value = Some(value);
    value
}

fn calculate_gate(gate: &Gate) -> bool {
    let a = calculate_wire(gate.input_a.clone());
    let b = calculate_wire(gate.input_b.clone());
    let value = match gate.typex {
        GateType::AND => a && b,
        GateType::OR => a || b,
        GateType::XOR => a ^ b,
    };
    value
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Wire {
    name: String,
    value: Option<bool>,
    input: Option<Gate>,
    output: Vec<Gate>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Gate {
    typex: GateType,
    input_a: Rc<RefCell<Wire>>,
    input_b: Rc<RefCell<Wire>>,
    output: Rc<RefCell<Wire>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum GateType {
    AND,
    OR,
    XOR,
}

fn read_file(filename: String) -> (HashMap<String, Rc<RefCell<Wire>>>, Vec<Gate> ) {
    let lines: Vec<String> = read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)
        .collect(); // gather them together into a vector

    let mut wires: HashMap<String, Rc<RefCell<Wire>>> = HashMap::new();
    let mut gates: Vec<Gate> = Vec::new();

    let mut parsing_inputs = true;
    for line in lines {
        if line == "" {
            parsing_inputs = false;
            continue;
        }
        if parsing_inputs {
            let parts: Vec<&str> = line.split(": ").collect();
            let wire = Wire {
                name: String::from(parts[0].trim()),
                value: Some(parts[1] == "1"),
                input: None,
                output: Vec::new(),
            };
            wires.insert(wire.name.clone(), Rc::new(RefCell::new(wire)));
        } else {
            let parts: Vec<&str> = line.split(" ").collect();
            let input_a = find_wire(&mut wires, parts[0].trim());
            let input_b = find_wire(&mut wires, parts[2].trim());
            let output = find_wire(&mut wires, parts[4].trim());
            let gate = Gate {
                typex: match parts[1].trim() {
                    "AND" => GateType::AND,
                    "OR" => GateType::OR,
                    "XOR" => GateType::XOR,
                    _ => panic!("Unknown gate type"),
                },
                input_a: input_a.clone(),
                input_b: input_b.clone(),
                output: output.clone(),
            };
            input_a.borrow_mut().output.push(gate.clone());
            input_b.borrow_mut().output.push(gate.clone());
            output.borrow_mut().input = Some(gate.clone());
            gates.push(gate)
        }
    }
    (wires, gates)
}

fn find_wire(wires: &mut HashMap<String, Rc<RefCell<Wire>>>, iname: &str) -> Rc<RefCell<Wire>> {
    let s_name = String::from(iname);
    wires.entry(s_name.clone()).or_insert_with(|| Rc::new(RefCell::new(Wire {
        name: s_name.clone(),
        value: None,
        input: None,
        output: Vec::new(),
    })));

    wires.get(&s_name).unwrap().clone()
}
