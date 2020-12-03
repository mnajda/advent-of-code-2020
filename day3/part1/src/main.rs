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
    let mut pos = 0;
    let mut trees = 0;

    for row in &input[1..] {
        let index = (pos + 3) % row.len();
        if row[index] == Point::Tree {
            trees += 1;
        }

        pos += 3;
    }

    return trees;
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
