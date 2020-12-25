use std::collections::{HashMap, HashSet};

use fancy_regex::Regex;

#[derive(Debug)]
pub struct D16Input {
    rules: HashMap<String, ValidRanges>,
    my_ticket: Vec<usize>,
    nearby_tickets: Vec<Vec<usize>>,
}

#[derive(Debug, Clone)]
pub struct ValidRanges {
    min1: usize,
    max1: usize,
    min2: usize,
    max2: usize,
}

#[aoc_generator(day16)]
pub fn parse_input(input: &str) -> D16Input {
    let mut sections = input.split("\n\n");

    let re_rules = Regex::new(r"^([\w\s]*): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
    let mut rules: HashMap<String, ValidRanges> = HashMap::new();
    sections.next().unwrap().lines().for_each(|line| {
        let captures = re_rules.captures(line).unwrap().unwrap();
        let field = captures.get(1).unwrap().as_str().to_string();
        let min1 = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let max1 = captures.get(3).unwrap().as_str().parse::<usize>().unwrap();
        let min2 = captures.get(4).unwrap().as_str().parse::<usize>().unwrap();
        let max2 = captures.get(5).unwrap().as_str().parse::<usize>().unwrap();

        let vr = ValidRanges {
            min1,
            max1,
            min2,
            max2,
        };
        rules.insert(field, vr);
    });

    let my_ticket: Vec<usize> = sections
        .next()
        .unwrap()
        .split('\n')
        .nth(1) // skip the header
        .unwrap()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let mut nearby_split = sections.next().unwrap().split('\n');
    nearby_split.next(); //skip the header

    let nearby_tickets: Vec<Vec<usize>> = nearby_split
        .map(|s| s.split(',').map(|t| t.parse::<usize>().unwrap()).collect())
        .collect();

    D16Input {
        rules,
        my_ticket,
        nearby_tickets,
    }
}

fn valid_column(val: &usize, rule: &ValidRanges) -> bool {
    (val >= &rule.min1 && val <= &rule.max1) || (val >= &rule.min2 && val <= &rule.max2)
}

fn p2_valid(nearby: &[usize], notes: &HashMap<String, ValidRanges>) -> bool {
    for item in nearby {
        let mut valid = false;
        for valid_ranges in notes.values() {
            if valid_column(item, valid_ranges) {
                valid = true;
            }
        }
        if !valid {
            return false;
        }
    }
    true
}

#[aoc(day16, part1)]
pub fn part1(input: &D16Input) -> usize {
    let mut checksum: usize = 0;

    for nearby in &input.nearby_tickets {
        for item in nearby {
            let mut valid = false;
            for rule in input.rules.values() {
                if valid_column(item, rule) {
                    valid = true;
                }
            }
            if !valid {
                checksum += item;
            }
        }
    }

    checksum
}

#[aoc(day16, part2)]
pub fn part2(input: &D16Input) -> usize {
    // filter out nearby that have values invalid for all rules
    let valid_nearby = input
        .nearby_tickets
        .iter()
        .filter(|i| p2_valid(i, &input.rules))
        .cloned()
        .collect::<Vec<Vec<usize>>>();

    // prime variables
    // initially must solve all of the rules
    let mut rules_to_solve: HashSet<String> = input.rules.keys().cloned().collect();

    // a solved rule will be of the form name -> column_index
    let mut solved_rules: HashMap<String, usize> = HashMap::new();

    // aliases for loop boundaries
    let num_rules = rules_to_solve.len();
    let valid_nearby_count = valid_nearby.len();

    while solved_rules.len() < num_rules {
        // while we have not solved all of the rules
        for index in 0..num_rules {
            // check each index if it can be solved
            // skip solved indices
            for i in solved_rules.values() {
                if index == *i {
                    continue;
                }
            }

            // store possible rules valid for the column with a count of rows that match it
            // prime with keys, and set counts to zero
            let mut possible_rules: HashMap<String, usize> = rules_to_solve
                .iter()
                .map(|r| (r.clone(), 0 as usize))
                .collect();

            for rule_name in &rules_to_solve {
                let rule = input.rules.get(rule_name).unwrap();

                for nearby in &valid_nearby {
                    let val = nearby.get(index).unwrap();

                    if valid_column(val, rule) {
                        possible_rules.insert(
                            rule_name.clone(),
                            possible_rules.get(rule_name).unwrap() + 1,
                        );
                    }
                }
            }

            // of the possible rules, consider only those who are valid for the entire column
            let valid_possible: Vec<String> = possible_rules
                .iter()
                .filter(|(_k, v)| **v == valid_nearby_count)
                .map(|(k, _v)| k.clone())
                .collect();

            // if there is only one rule possible for this column, it is solved
            if valid_possible.len() == 1 {
                let rule_name = valid_possible.get(0).unwrap();
                solved_rules.insert(rule_name.clone(), index); // save the solved rule
                rules_to_solve.remove(rule_name); // remove it from possible
            }
        }

        // in theory, bad input could cause an infinite loop here...
    }

    // calculate the checksum we care about
    // multiply MY ticket values for all column names beginning with 'departure'
    solved_rules
        .iter()
        .filter(|(k, _v)| k.starts_with("departure"))
        .map(|(_k, v)| input.my_ticket.get(*v).unwrap())
        .product::<usize>()
}
