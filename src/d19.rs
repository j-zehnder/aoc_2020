pub struct D19Input {
    rules: Vec<Rule>,
    messages: Vec<String>,
}

pub enum Rule {
    Literal,
    Or,
}

#[aoc_generator(day19)]
pub fn parse_input(input: &str) -> D19Input {
    D19Input {
        rules: Vec::new(),
        messages: Vec::new(),
    }
}

#[aoc(day19, part1)]
pub fn part1(input: &D19Input) -> usize {
    0
}

#[aoc(day19, part2)]
pub fn part2(input: &D19Input) -> usize {
    0
}
