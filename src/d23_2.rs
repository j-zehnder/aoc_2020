use std::fmt::{Debug, Formatter};

const LEN: usize = 1_000_000;
const STEPS: usize = 10_000_000;
fn parse_input(seed_str: &str) -> Vec<usize> {
    seed_str
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect()
}

struct Cups {
    current: usize,
    cups: [usize; LEN + 1],
}

impl Debug for Cups {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "Cups: {} - {:?}", self.current, self.cups)
    }
}

impl Cups {
    fn new(seed: &[usize]) -> Self {
        let mut cups = Cups {
            current: seed[0],
            cups: [0; LEN + 1],
        };

        for w in seed.windows(2) {
            cups.cups[w[0]] = w[1];
        }

        let mut last = seed[seed.len() - 1];

        for x in 10..=LEN {
            cups.cups[last] = x;
            last = x;
        }

        cups.cups[last] = seed[0];
        cups
    }

    fn step(&mut self, steps: usize) {
        for _ in 0..steps {
            //println!("cur: {}", self.current);

            let c1 = self.cups[self.current];
            let c2 = self.cups[c1];
            let c3 = self.cups[c2];
            self.cups[self.current] = self.cups[c3];

            //println!("picking up {}, {}, {}", c1, c2, c3);

            let mut destination = self.current - 1;
            if destination == 0 {
                destination = LEN
            };
            while [c1, c2, c3].contains(&destination) {
                destination -= 1;
                if destination == 0 {
                    destination = LEN
                };
            }
            //println!("destination: {}", destination);

            let after = self.cups[destination];
            self.cups[destination] = c1;
            self.cups[c3] = after;

            self.current = self.cups[self.current];
        }
    }
}

#[aoc(day23, part2)]
pub fn part2(input: &str) -> usize {
    let seed = parse_input(input);
    let mut cups = Cups::new(&seed);
    cups.step(STEPS);
    let a = cups.cups[1];
    let b = cups.cups[a];
    a * b
}

