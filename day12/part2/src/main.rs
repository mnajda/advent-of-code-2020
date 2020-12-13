use std::fs;
use std::env;

struct Command {
    action: char,
    value: i64,
}

#[derive(Clone)]
struct Position {
    x: i64,
    y: i64,
}

fn make_command(line: &str) -> Command {
    let action = line.chars().nth(0).unwrap();
    let value = line[1..].parse::<i64>().unwrap();

    return Command{ action: action, value: value };
}

fn load_file(path: &String) -> Vec<Command> {
    let contents = fs::read_to_string(path).expect("Error reading file");
    return contents
        .lines()
        .map(|line| make_command(line))
        .collect();
}

fn turn(position: &mut Position, value: i64) {
    let old = position.clone();
    match value {
        0 => {},
        90 => {
            position.x = old.y;
            position.y = old.x * -1;
        },
        180 => {
            position.x = old.x * -1;
            position.y = old.y * -1;
        },
        270 => {
            position.x = old.y * -1;
            position.y = old.x;
        },
        _ => unreachable!(),
    }
}

fn move_ship(position: &mut Position, target: &Position, value: i64) {
    position.x += value * target.x;
    position.y += value * target.y;
}

fn solve(commands: Vec<Command>) -> i64 {
    let mut ship = Position{ x: 0, y: 0};
    let mut target = Position{ x: 10, y: 1};

    for command in commands {
        match command.action {
            'N' => { target.y += command.value; },
            'S' => { target.y -= command.value; },
            'E' => { target.x += command.value; },
            'W' => { target.x -= command.value; },
            'L' => { turn(&mut target, 360 - command.value); },
            'R' => { turn(&mut target, command.value); },
            'F' => { move_ship(&mut ship, &target, command.value) },
            _ => unreachable!(),
        }
    }
    
    return ship.x.abs() + ship.y.abs();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No file provided");
        return;
    }

    let input = load_file(&args[1]);
    let result = solve(input);

    println!("{}", result);
}