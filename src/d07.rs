use std::collections::{HashMap, HashSet};

use fancy_regex::Regex;

type BagName = String;

#[derive(Debug)]
pub struct RuleSet {
    rules: HashMap<BagName, Vec<BagContent>>,
}

impl RuleSet {
    fn from_str(input: &str) -> Self {
        let mut rules = HashMap::new();

        let re = Regex::new(r"^(.*) bags contain (.*)\.$").unwrap();
        let content_re = Regex::new(r"^(\d+) (.*) bag[s]?$").unwrap();

        input.lines().for_each(|line| {
            let captures = re.captures(line).unwrap().unwrap();
            let bag_name: BagName = captures.get(1).unwrap().as_str().to_string();

            let contents_string = captures.get(2).unwrap().as_str();
            let mut contents: Vec<BagContent> = Vec::new();

            if contents_string != "no other bags" {
                for content_string in contents_string.split(",") {
                    let content_string = content_string.trim();
                    let content_captures = content_re.captures(content_string).unwrap().unwrap();
                    let bag_content = BagContent {
                        name: String::from(content_captures.get(2).unwrap().as_str()),
                        count: content_captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                    };
                    contents.push(bag_content);
                }
            }
            rules.insert(bag_name, contents);
        });
        RuleSet { rules }
    }

    fn count_bags_containing(&self, search_key: &str) -> usize {
        self.list_bags_containing(search_key).len()
    }

    fn list_bags_containing(&self, search_key: &str) -> HashSet<BagName>{
        let mut bags_containing: HashSet<BagName> = HashSet::new();
        for (key, bag_contents) in &self.rules {
            for bag_content in bag_contents {
                if bag_content.name == search_key {
                    bags_containing.insert(key.clone());
                    for bag in self.list_bags_containing(key) {
                        bags_containing.insert(bag);
                    }
                }
            }
        }
        bags_containing
    }

    fn count_bags_contained_in(&self, search_key: &str) -> usize {
        let mut count: usize = 0;

        let bag_contents = self.rules.get(search_key).unwrap();

        for bag_content in bag_contents {
            count += bag_content.count
                + bag_content.count * self.count_bags_contained_in(&bag_content.name);
        }

        count
    }
}

#[derive(Debug)]
pub struct BagContent {
    name: BagName,
    count: usize,
}


#[aoc_generator(day7)]
pub fn parse_input(input: &str) -> RuleSet {
    RuleSet::from_str(input)
}

#[aoc(day7, part1)]
pub fn part1(ruleset: &RuleSet) -> usize {
    ruleset.count_bags_containing("shiny gold")
}

#[aoc(day7, part2)]
pub fn part2(ruleset: &RuleSet) -> usize {
    ruleset.count_bags_contained_in("shiny gold")
}
