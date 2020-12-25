use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Group {
    forms: Vec<Form>,
}

impl Group {
    fn from_str(s: &str) -> Self {
        Group {
            forms: s.split('\n').map(Form::from_str).collect(),
        }
    }
}

#[derive(Debug)]
pub struct Form {
    yes_questions: Vec<char>,
}

impl Form {
    fn from_str(s: &str) -> Self {
        Form {
            yes_questions: s.chars().collect(),
        }
    }
}

#[aoc_generator(day6)]
pub fn parse_input(input: &str) -> Vec<Group> {
    input.split("\n\n").map(Group::from_str).collect()
}

#[aoc(day6, part1)]
pub fn part1(groups: &[Group]) -> usize {
    let mut count = 0;

    for group in groups {
        let mut yes_set: HashSet<char> = HashSet::new();

        for form in &group.forms {
            for c in &form.yes_questions {
                yes_set.insert(*c);
            }
        }

        count += yes_set.len();
    }

    count
}

#[aoc(day6, part2)]
pub fn part2(groups: &[Group]) -> usize {
    let mut count = 0;

    for group in groups {
        let mut yes_map: HashMap<char, usize> = HashMap::new();
        for form in &group.forms {
            for c in &form.yes_questions {
                if yes_map.contains_key(c) {
                    *yes_map.get_mut(c).unwrap() += 1;
                } else {
                    yes_map.insert(*c, 1);
                }
            }
        }

        count += yes_map
            .values()
            .filter(|v| **v == group.forms.len())
            .count();
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_t1() {
        let groups = parse_input(SAMPLE_1);
        assert_eq!(part1(&groups), 11)
    }

    const SAMPLE_1: &str = "abc

a
b
c

ab
ac

a
a
a
a

b";
}
