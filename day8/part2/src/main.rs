use std::env;
use std::fs;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Operation {
    Nop,
    Acc,
    Jmp,
}

#[derive(Clone, Debug)]
struct Instruction {
    operation: Operation,
    operand: i64,
}

struct Device {
    ip: i64,
    acc: i64,
    program: Vec<Instruction>,
    executed: Vec<bool>,
}

impl Device {
    pub fn new(program: Vec<Instruction>) -> Self {
        let executed = vec![false; program.len()];
        Device {
            ip: 0,
            acc: 0,
            program,
            executed: executed,
        }
    }

    pub fn run(&mut self) -> Option<i64> {
        loop {
            if self.ip as usize >= self.program.len() {
                return Some(self.acc);
            }
            if self.executed[self.ip as usize] == true {
                return None
            }
            self.execute_instruction();
        }
    }

    fn execute_instruction(&mut self) {
        let instruction = &self.program[self.ip as usize];
        match instruction.operation {
            Operation::Nop => {
                self.executed[self.ip as usize] = true;
                self.ip += 1;
            }
            Operation::Acc => {
                self.executed[self.ip as usize] = true;
                self.acc += instruction.operand;
                self.ip += 1;
            }
            Operation::Jmp => {
                let offset = instruction.operand;
                self.executed[self.ip as usize] = true;
                self.ip += offset;
            }
        };
    }
}

fn make_operation(op: &str) -> Operation {
    match op {
        "nop" => Operation::Nop,
        "acc" => Operation::Acc,
        "jmp" => Operation::Jmp,
        _ => panic!("Unsupported operation"),
    }
}

fn make_instruction(line: &str) -> Instruction {
    let instruction: Vec<&str> = line.split_whitespace().collect();

    return Instruction {
        operation: make_operation(instruction[0]),
        operand: instruction[1].parse::<i64>().unwrap(),
    };
}

fn load_program(path: &String) -> Vec<Instruction> {
    let contents = fs::read_to_string(path).expect("Error reading file");
    return contents
        .lines()
        .map(|line| make_instruction(line))
        .collect();
}

fn change_operation(op: Operation) -> Operation {
    match op {
        Operation::Jmp => Operation::Nop,
        Operation::Nop => Operation::Jmp,
        _ => panic!("Only jmps and nops should be changed"),
    }
}

fn make_programs(input: &Vec<Instruction>, op: Operation) -> Vec<Vec<Instruction>> {
    let mut output = Vec::new();

    for _ in 0..input.len() {
        for i in 0..input.len() {
            let operation = &input[i].operation;
            if *operation == op {
                let mut copy = input.clone();
                let operand = input[i].operand;
                copy[i] = Instruction {
                    operation: change_operation(op),
                    operand: operand,
                };
                output.push(copy);
            }
        }
    }

    return output;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No file provided");
        return;
    }

    let input = load_program(&args[1]);

    let changed_nops = make_programs(&input, Operation::Nop);
    let changed_jmps = make_programs(&input, Operation::Jmp);

    for program in changed_nops.iter().zip(changed_jmps.iter()) {
        let (changed_nop, changed_jmp) = program;

        let mut changed_nop_device = Device::new(changed_nop.to_vec());
        let mut changed_jmp_device = Device::new(changed_jmp.to_vec());

        if let Some(result) = changed_nop_device.run() {
            println!("Changed nop and fixed program, acc: {}", result);
            return;
        }

        if let Some(result) = changed_jmp_device.run() {
            println!("Changed jmp and fixed program, acc: {}", result);
            return;
        }
    }
}
