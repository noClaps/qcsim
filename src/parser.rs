use std::{
    collections::HashMap,
    f64::consts::{E, PI},
    process::exit,
    str::FromStr,
};

use num_complex::Complex64;
use tree_sitter::{Node, Parser, Range};

use crate::{computer::Computer, qubit::Qubit};

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Measure,       // measure(uint?)
    PauliX,        // x(qubit)
    PauliY,        // y(qubit)
    PauliZ,        // z(qubit)
    Hadamard,      // hadamard(qubit)
    Phase,         // phase(qubit)
    PiBy8,         // pi_8(qubit)
    ControlledNot, // cnot(qubit, qubit)
    ControlledZ,   // cz(qubit, qubit)
    Swap,          // swap(qubit, qubit)
    Toffoli,       // toffoli(qubit, qubit, qubit)
}

pub struct QCLang {
    pub input: String,
    pub variables: Vec<(String, Qubit)>,
    pub instructions: Vec<(Instruction, Vec<String>)>,
}

impl QCLang {
    pub fn new(input: String) -> Self {
        Self {
            input,
            variables: vec![],
            instructions: vec![],
        }
    }

    fn add_instr(&mut self, instr: Instruction, children: Vec<Node>, grammar_name: &str) {
        let args = children
            .iter()
            .filter(|c| c.grammar_name() == grammar_name)
            .map(|c| {
                let name = self.get_slice(c.range());
                match grammar_name {
                    "var_name" => name,
                    "uint" => match u64::from_str_radix(&name, 10) {
                        Ok(_) => name,
                        Err(err) => {
                            eprintln!("[ERROR] Invalid argument: {err}");
                            exit(1);
                        }
                    },
                    _ => unreachable!("The only possible grammar names are `var_name` and `uint`"),
                }
            })
            .collect::<Vec<String>>();

        self.instructions.push((instr, args));
    }

    fn parse_expr(&mut self, node: Node) -> Complex64 {
        match node.kind() {
            "number" => match Complex64::from_str(&self.get_slice(node.range())) {
                Ok(num) => num,
                Err(err) => {
                    eprintln!("[ERROR] Error parsing value: {err}");
                    exit(1);
                }
            },

            // constants
            "pi" => Complex64::from(PI),
            "imag" => Complex64::I,
            "euler" => Complex64::from(E),

            // binary expressions
            "add" => {
                self.parse_expr(node.child_by_field_name("arg1").unwrap())
                    + self.parse_expr(node.child_by_field_name("arg2").unwrap())
            }
            "sub" => {
                self.parse_expr(node.child_by_field_name("arg1").unwrap())
                    - self.parse_expr(node.child_by_field_name("arg2").unwrap())
            }
            "mul" => {
                self.parse_expr(node.child_by_field_name("arg1").unwrap())
                    * self.parse_expr(node.child_by_field_name("arg2").unwrap())
            }
            "div" => {
                self.parse_expr(node.child_by_field_name("arg1").unwrap())
                    / self.parse_expr(node.child_by_field_name("arg2").unwrap())
            }
            "exp" => self
                .parse_expr(node.child_by_field_name("arg1").unwrap())
                .powc(self.parse_expr(node.child_by_field_name("arg2").unwrap())),

            // other functions
            "sin" => self
                .parse_expr(node.child_by_field_name("arg").unwrap())
                .sin(),
            "cos" => self
                .parse_expr(node.child_by_field_name("arg").unwrap())
                .cos(),
            "tan" => self
                .parse_expr(node.child_by_field_name("arg").unwrap())
                .tan(),
            "root" => self
                .parse_expr(node.child_by_field_name("arg1").unwrap())
                .powc(
                    self.parse_expr(node.child_by_field_name("arg2").unwrap())
                        .inv(),
                ),

            "(" => self.parse_expr(node.next_sibling().unwrap()),
            _ => {
                unreachable!()
            }
        }
    }

    fn get_num_value(&mut self, children: Vec<Node>) -> Complex64 {
        match children[0] {
            qubit if self.get_slice(qubit.range()) == "|0>" => Complex64::ONE,
            qubit if self.get_slice(qubit.range()) == "|1>" => Complex64::ONE,
            _ => {
                let mut cursor = 0;
                while children[cursor].grammar_name() == "(" {
                    cursor += 1;
                }
                self.parse_expr(children[cursor])
            }
        }
    }

    fn add_variable(&mut self, children: Vec<Node>) {
        let var_name = self.get_slice(children[0].range());

        let var_expr = children[2];
        let mut cursor = var_expr.walk();
        let children = var_expr.children(&mut cursor).collect::<Vec<Node>>();

        let qubit_zero = match children
            .iter()
            .find(|c| c.grammar_name() == "qubit_zero")
            .cloned()
        {
            None => Complex64::ZERO,
            Some(node) => {
                let mut cursor = node.walk();
                let node_children = node.children(&mut cursor).collect::<Vec<Node>>();
                self.get_num_value(node_children)
            }
        };
        let qubit_one = match children
            .iter()
            .find(|c| c.grammar_name() == "qubit_one")
            .cloned()
        {
            None => Complex64::ZERO,
            Some(node) => {
                let mut cursor = node.walk();
                let node_children = node.children(&mut cursor).collect::<Vec<Node>>();
                self.get_num_value(node_children)
            }
        };

        self.variables
            .push((var_name, Qubit::new(qubit_zero, qubit_one)));
    }

    fn get_slice(&self, range: Range) -> String {
        let start = range.start_point;
        let end = range.end_point;
        let line = self.input.lines().nth(start.row).unwrap_or_default();

        let slice = line[start.column..end.column].to_string();
        slice
    }

    fn parse_tree(&mut self, nodes: Vec<Node>, depth: usize) {
        for node in nodes {
            let mut cursor = node.walk();
            let children = node.children(&mut cursor).collect::<Vec<Node>>();
            match node.grammar_name() {
                "measure" => self.add_instr(Instruction::Measure, children, "uint"),
                "pauli_x" => self.add_instr(Instruction::PauliX, children, "var_name"),
                "pauli_y" => self.add_instr(Instruction::PauliY, children, "var_name"),
                "pauli_z" => self.add_instr(Instruction::PauliZ, children, "var_name"),
                "hadamard" => self.add_instr(Instruction::Hadamard, children, "var_name"),
                "phase" => self.add_instr(Instruction::Phase, children, "var_name"),
                "pi_by_8" => self.add_instr(Instruction::PiBy8, children, "var_name"),
                "controlled_not" => {
                    self.add_instr(Instruction::ControlledNot, children, "var_name")
                }
                "controlled_z" => self.add_instr(Instruction::ControlledZ, children, "var_name"),
                "swap" => self.add_instr(Instruction::Swap, children, "var_name"),
                "toffoli" => self.add_instr(Instruction::Toffoli, children, "var_name"),
                "variable_declaration" => self.add_variable(children),
                _ => {
                    if node.child_count() > 0 {
                        self.parse_tree(children, depth + 1);
                    }
                }
            };
        }
    }

    pub fn parse(&mut self) {
        let mut parser = Parser::new();
        match parser.set_language(&tree_sitter_qc::LANGUAGE.into()) {
            Ok(_) => (),
            Err(err) => {
                eprintln!("[ERROR] Error setting language: {err}");
                exit(1);
            }
        };

        let tree = match parser.parse(self.input.clone(), None) {
            Some(tree) => tree,
            None => {
                eprintln!("[ERROR] Error parsing input");
                exit(1);
            }
        };
        let nodes = vec![tree.root_node()];
        self.parse_tree(nodes, 0);
    }

    pub fn run(&self) {
        let qubits = self.variables.clone();

        let mut computer = Computer::new(qubits.iter().map(|q| q.1).collect());
        for (instr, args) in self.instructions.clone() {
            match instr {
                Instruction::Measure => {
                    let mut count = 1;
                    if args.len() == 1 {
                        count = match u64::from_str(&args[0]) {
                            Ok(num) => num,
                            Err(err) => {
                                eprintln!("[ERROR] Error parsing number: {err}");
                                exit(1);
                            }
                        };
                    }

                    let mut outputs = HashMap::new();
                    if count == 1 {
                        println!("{}", computer.measure());
                        continue;
                    }

                    for _ in 0..count {
                        let measured_state = computer.measure();
                        match outputs.get_mut(&measured_state) {
                            Some(state) => *state += 1,
                            None => {
                                outputs.insert(measured_state, 1);
                            }
                        };
                    }
                    println!("{:#?}", outputs);
                }
                Instruction::PauliX => {
                    let arg = args[0].clone();
                    let index = match qubits.iter().position(|q| q.0 == arg) {
                        Some(i) => i,
                        None => {
                            eprintln!("[ERROR] Variable used but not declared: {arg}");
                            exit(1);
                        }
                    };
                    computer.pauli_x(index);
                }
                Instruction::PauliY => {
                    let arg = args[0].clone();
                    let index = match qubits.iter().position(|q| q.0 == arg) {
                        Some(i) => i,
                        None => {
                            eprintln!("[ERROR] Variable used but not declared: {arg}");
                            exit(1);
                        }
                    };
                    computer.pauli_y(index);
                }
                Instruction::PauliZ => {
                    let arg = args[0].clone();
                    let index = match qubits.iter().position(|q| q.0 == arg) {
                        Some(i) => i,
                        None => {
                            eprintln!("[ERROR] Variable used but not declared: {arg}");
                            exit(1);
                        }
                    };
                    computer.pauli_z(index);
                }
                Instruction::Hadamard => {
                    let arg = args[0].clone();
                    let index = match qubits.iter().position(|q| q.0 == arg) {
                        Some(i) => i,
                        None => {
                            eprintln!("[ERROR] Variable used but not declared: {arg}");
                            exit(1);
                        }
                    };
                    computer.hadamard(index);
                }
                Instruction::Phase => {
                    let arg = args[0].clone();
                    let index = match qubits.iter().position(|q| q.0 == arg) {
                        Some(i) => i,
                        None => {
                            eprintln!("[ERROR] Variable used but not declared: {arg}");
                            exit(1);
                        }
                    };
                    computer.phase(index);
                }
                Instruction::PiBy8 => {
                    let arg = args[0].clone();
                    let index = match qubits.iter().position(|q| q.0 == arg) {
                        Some(i) => i,
                        None => {
                            eprintln!("[ERROR] Variable used but not declared: {arg}");
                            exit(1);
                        }
                    };
                    computer.pi_by_8(index);
                }
                Instruction::ControlledNot => {
                    let arg1 = args[0].clone();
                    let arg2 = args[1].clone();
                    let index1 = match qubits.iter().position(|q| q.0 == arg1) {
                        Some(i) => i,
                        None => {
                            eprintln!("[ERROR] Variable used but not declared: {arg1}");
                            exit(1);
                        }
                    };
                    let index2 = match qubits.iter().position(|q| q.0 == arg2) {
                        Some(i) => i,
                        None => {
                            eprintln!("[ERROR] Variable used but not declared: {arg2}");
                            exit(1);
                        }
                    };
                    computer.controlled_not(index1, index2);
                }
                Instruction::ControlledZ => {
                    let arg1 = args[0].clone();
                    let arg2 = args[1].clone();
                    let index1 = match qubits.iter().position(|q| q.0 == arg1) {
                        Some(i) => i,
                        None => {
                            eprintln!("[ERROR] Variable used but not declared: {arg1}");
                            exit(1);
                        }
                    };
                    let index2 = match qubits.iter().position(|q| q.0 == arg2) {
                        Some(i) => i,
                        None => {
                            eprintln!("[ERROR] Variable used but not declared: {arg2}");
                            exit(1);
                        }
                    };
                    computer.controlled_z(index1, index2);
                }
                Instruction::Swap => {
                    let arg1 = args[0].clone();
                    let arg2 = args[1].clone();
                    let index1 = match qubits.iter().position(|q| q.0 == arg1) {
                        Some(i) => i,
                        None => {
                            eprintln!("[ERROR] Variable used but not declared: {arg1}");
                            exit(1);
                        }
                    };
                    let index2 = match qubits.iter().position(|q| q.0 == arg2) {
                        Some(i) => i,
                        None => {
                            eprintln!("[ERROR] Variable used but not declared: {arg2}");
                            exit(1);
                        }
                    };
                    computer.swap(index1, index2);
                }
                Instruction::Toffoli => {
                    let arg1 = args[0].clone();
                    let arg2 = args[1].clone();
                    let arg3 = args[2].clone();
                    let index1 = match qubits.iter().position(|q| q.0 == arg1) {
                        Some(i) => i,
                        None => {
                            eprintln!("[ERROR] Variable used but not declared: {arg1}");
                            exit(1);
                        }
                    };
                    let index2 = match qubits.iter().position(|q| q.0 == arg2) {
                        Some(i) => i,
                        None => {
                            eprintln!("[ERROR] Variable used but not declared: {arg2}");
                            exit(1);
                        }
                    };
                    let index3 = match qubits.iter().position(|q| q.0 == arg3) {
                        Some(i) => i,
                        None => {
                            eprintln!("[ERROR] Variable used but not declared: {arg3}");
                            exit(1);
                        }
                    };
                    computer.toffoli(index1, index2, index3);
                }
            }
        }
    }
}
