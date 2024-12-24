use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;
use std::rc::Rc;

fn main() {
    let (wires, gates) = read_file(env::args().collect::<Vec<String>>()[1].clone());
    part1(&wires, &gates);
    part2(&wires, &gates);
}

fn part1(wires: &HashMap<String, Rc<RefCell<Wire>>>, _gates: &Vec<Gate>) {
    let mut z_wires = find_wires_named("z", wires);
    let result = value_of(&mut z_wires);
    println!("Part 1: {:?}", result);
}

fn part2(wires: &HashMap<String, Rc<RefCell<Wire>>>, _gates: &Vec<Gate>) {
    let mut x_wires = find_wires_named("x", wires);
    let mut y_wires = find_wires_named("y", wires);
    let mut z_wires = find_wires_named("z", wires);
    for i in 22..45 {
        let y = 1 << i;
        let x = 0;
        reset(wires);
        set_value(&mut x_wires, x); // 0b1010_1010_1010_1010
        set_value(&mut y_wires, y); // 0b0101_0101_0101_0101
        println!("x: {:?}", value_of(&mut x_wires));
        println!("y: {:?}", value_of(&mut y_wires));
        let z = value_of(&mut z_wires);
        println!("z: {:?} {:b}       {}", z, z, i);
        if z != x + y {
            break;
        }
        // swap shj z07
        // swap tpk wkb
        // swap z23 pfn
        // swap kcd z27
        // kcd,pfn,shj,tpk,wkb,z07,z23,z27
    }
}

fn reset(wires: &HashMap<String, Rc<RefCell<Wire>>>) {
    for wire in wires.values() {
        wire.borrow_mut().value = None;
    }
}

fn set_value(wires: &mut Vec<Rc<RefCell<Wire>>>, value: i64) {
    let mut  x = value;
    for wire in wires.iter().rev() {
        wire.borrow_mut().value = Some((x & 1) == 1);
        x >>= 1;
    }
}

fn value_of(wires: &mut Vec<Rc<RefCell<Wire>>>) -> i64 {
    let mut result = 0_i64;
    for wire in wires {
        let val = calculate_wire(wire.clone());
        result <<= 1;
        if val {
            result += 1;
        }
        //println!("Wire: {:?} {:?}, {}", wire.borrow().name, val, result);
    }
    result
}

fn find_wires_named(name: &str, wires: &HashMap<String, Rc<RefCell<Wire>>>) -> Vec<Rc<RefCell< crate::Wire >>> {

    let mut result: Vec<Rc<RefCell< crate::Wire >>> = Vec::new();

    for wire in wires.values() {
        if wire.borrow().name.starts_with(name) {
            result.push(wire.clone());
        }
    }
    result.sort_by(|a, b| b.borrow().name.cmp(&a.borrow().name));
    result
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
