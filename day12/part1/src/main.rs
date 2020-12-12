use std::fs;
use std::env;

struct Command {
    action: char,
    value: i64,
}

struct Position {
    x: i64,
    y: i64,
    direction: i64,
}

fn make_command(line: &str) -> Command {
    let action = line.chars().nth(0).unwrap();
    let value = line[1..].parse::<i64>().unwrap();

    return Command{ action: action, value: value };
}

fn load_program(path: &String) -> Vec<Command> {
    let contents = fs::read_to_string(path).expect("Error reading file");
    return contents
        .lines()
        .map(|line| make_command(line))
        .collect();
}

fn turn(position: &mut Position, value: i64) {
    position.direction = (position.direction + value).abs() % 360;
}

fn move_ship(position: &mut Position, value: i64) {
    match position.direction {
        0 => { position.y += value; },
        180 => { position.y -= value; },
        90 => { position.x += value; },
        270 => { position.x -= value; },
        _ => unreachable!(),
    }
}

fn solve(commands: Vec<Command>) -> i64 {
    let mut position = Position{ x: 0, y: 0, direction: 90 };

    for command in commands {
        match command.action {
            'N' => { position.y += command.value; },
            'S' => { position.y -= command.value; },
            'E' => { position.x += command.value; },
            'W' => { position.x -= command.value; },
            'L' => { turn(&mut position, 360 - command.value); },
            'R' => { turn(&mut position, command.value); },
            'F' => { move_ship(&mut position, command.value) },
            _ => unreachable!(),
        }
    }
    
    return position.x.abs() + position.y.abs();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No file provided");
        return;
    }

    let input = load_program(&args[1]);
    let result = solve(input);

    println!("{}", result);
}