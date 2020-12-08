use std::env;
use std::fs;

enum Operation {
    Nop,
    Acc,
    Jmp,
}

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
        Device { ip: 0, acc: 0, program, executed: executed }
    }

    pub fn run(&mut self) -> i64 {
        while self.executed[self.ip as usize] != true {
            self.execute_instruction()
        }

        return self.acc;
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
            },
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

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No file provided");
        return;
    }

    let program = load_program(&args[1]);
    let mut device = Device::new(program);
    let result = device.run();

    println!("{}", result);
}
