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
    let steps = [1, 3, 5, 7, 1];
    let mut pos = [0, 0, 0, 0, 0];
    let mut slopes = [0, 0, 0, 0, 0];

    for i in 1..input.len() {
        for slope in 0..4 {
            let index = (pos[slope] + steps[slope]) % len;
            slopes[slope] += if input[i][index] == Point::Tree { 1 } else { 0 };

            pos[slope] += steps[slope];
        }
        if i % 2 == 0 {
            let index = (pos[4] + steps[4]) % len;
            slopes[4] += if input[i][index] == Point::Tree { 1 } else { 0 };

            pos[4] += steps[4];
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
