use std::env;
use std::fs;

#[derive(PartialEq)]
enum Point {
    Empty,
    Tree
}

fn to_point(val: char) -> Point {
    match val {
        '.' => Point::Empty,
        '#' => Point::Tree,
        _ => panic!("Unknown character"),
    }
}

fn convert(row: &str) -> Vec<Point> {
    return row.chars().map(|c| to_point(c)).collect();
}

fn load_file(path: &String) -> Vec<Vec<Point>> {
    let contents = fs::read_to_string(path).expect("Error reading file");
    return contents.split_whitespace().map(|row| convert(row)).collect();
}

fn solve(input: Vec<Vec<Point>>) -> i64 {
    let len = input.first().unwrap().len();
    let steps = [(1 ,1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut pos = [0, 0, 0, 0, 0];
    let mut slopes = [0, 0, 0, 0, 0];

    for (slope, (right, down)) in steps.iter().enumerate() {
        for i in (*down..input.len()).step_by(*down) {
            let index = (pos[slope] + right) % len;
            if input[i][index] == Point::Tree {
                slopes[slope] += 1;
            }
            pos[slope] += right;
        }
    }

    return slopes.iter().fold(1, |acc, val| acc * val);
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
