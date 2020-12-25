pub struct PolicyPass {
    letter: char,
    min: i32,
    max: i32,
    password: String,
}

impl PolicyPass {
    fn from_str(s: &str) -> Self {
        if let Ok((min, max, letter, password)) =
            scan_fmt!(s, "{d}-{d} {}: {}", i32, i32, char, String)
        {
            return PolicyPass {
                letter,
                min,
                max,
                password,
            };
        }
        panic!("unable to parse input line")
    }

    fn valid_p1(&self) -> bool {
        let count = self.password.matches(self.letter).count() as i32;
        count >= self.min && count <= self.max
    }

    fn valid_p2(&self) -> bool {
        let letter_at_min =
            self.password.chars().nth((self.min - 1) as usize).unwrap() == self.letter;

        let letter_at_max =
            self.password.chars().nth((self.max - 1) as usize).unwrap() == self.letter;

        letter_at_min ^ letter_at_max
    }
}

#[aoc_generator(day2)]
pub fn parse_input(input: &str) -> Vec<PolicyPass> {
    input.lines().map(PolicyPass::from_str).collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[PolicyPass]) -> usize {
    input.iter().filter(|p| p.valid_p1()).count()
}

#[aoc(day2, part2)]
pub fn part2(input: &[PolicyPass]) -> usize {
    input.iter().filter(|p| p.valid_p2()).count()
}
