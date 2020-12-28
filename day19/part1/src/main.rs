use std::collections::HashMap;
use std::env;
use std::fs;

enum Rule {
    Character(char),
    Set(Vec<usize>),
}

type Rules = Vec<Rule>;
type RulesMap = HashMap<usize, Rules>;

fn make_rule_set(rule: &str) -> Rule {
    return Rule::Set(
        rule.split_whitespace()
            .map(|val| val.parse().unwrap())
            .collect::<Vec<usize>>(),
    );
}

fn parse_rule(rule: &str) -> Rules {
    let mut output = Vec::new();

    if rule.contains("\"") {
        output.push(Rule::Character(rule.chars().skip(1).next().unwrap()))
    } else if rule.contains(" | ") {
        for part in rule.split(" | ") {
            output.push(make_rule_set(part));
        }
    } else {
        output.push(make_rule_set(rule));
    }

    return output;
}

fn parse_rules(text: &str) -> RulesMap {
    let mut output = HashMap::new();

    for rule in text.lines() {
        let parts: Vec<&str> = rule.split(": ").collect();
        let index = parts[0].parse::<usize>().unwrap();
        let rules = parse_rule(parts[1]);

        output.insert(index, rules);
    }

    return output;
}

fn load_file(path: &String) -> (RulesMap, Vec<Vec<char>>) {
    let contents = fs::read_to_string(path).expect("Error reading file");
    let mut strings = contents.split("\n\n");

    let rules = strings.next().unwrap();
    let messages = strings.next().unwrap();

    return (
        parse_rules(rules),
        messages
            .lines()
            .map(|line| line.chars().collect())
            .collect(),
    );
}

fn is_valid(rules: &RulesMap, letters: &[char], set: Vec<usize>) -> bool {
    match (letters.len(), set.len()) {
        (0, 0) => return true,
        (_, 0) => return false,
        (0, _) => return false,
        (_, _) => {}
    }

    let rule = set.first().unwrap();
    let remaining = &set[1..];
    let to_be_checked = &rules[rule];

    for rule in to_be_checked {
        let valid = match rule {
            Rule::Character(c) => {
                if c == letters.first().unwrap() {
                    is_valid(&rules, &letters[1..], remaining.to_vec())
                } else {
                    false
                }
            }
            Rule::Set(rest) => {
                let set = rest.iter().chain(remaining.iter()).copied().collect();
                is_valid(&rules, &letters, set)
            }
        };

        if valid {
            return true;
        }
    }

    return false;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No file provided");
        return;
    }

    let (rules, messages) = load_file(&args[1]);
    let result = messages
        .iter()
        .map(|message| return is_valid(&rules, message, vec![0]))
        .filter(|&valid| return valid)
        .count();

    println!("{}", result);
}
