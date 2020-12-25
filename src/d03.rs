use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

pub struct Forest {
    width: usize,
    height: usize,
    trees: HashSet<Coord>,
}

impl Forest {
    fn new(input: &str) -> Self {
        let width = input.lines().last().unwrap().len();
        let mut height = 0;
        let mut trees = HashSet::new();
        for line in input.lines() {
            height += 1;

            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => {
                        trees.insert(Coord { x, y: height - 1 });
                    }
                    '.' => {}
                    _ => panic!("unknown character"),
                }
            }
        }

        Forest {
            width,
            height,
            trees,
        }
    }

    fn is_tree(&self, x: usize, y: usize) -> bool {
        let coord = Coord {
            x: x % self.width,
            y,
        };
        self.trees.contains(&coord)
    }

    fn count_hit_trees(&self, vx: usize, vy: usize) -> usize {
        let mut count: usize = 0;

        let mut x: usize = 0;
        let mut y: usize = 0;

        while y < self.height {
            if self.is_tree(x, y) {
                count += 1;
            }
            x += vx;
            y += vy;
        }
        count
    }
}

#[aoc_generator(day3)]
pub fn parse_input(input: &str) -> Forest {
    Forest::new(input)
}

#[aoc(day3, part1)]
pub fn part1(forest: &Forest) -> usize {
    forest.count_hit_trees(3, 1)
}

#[aoc(day3, part2)]
pub fn part2(forest: &Forest) -> usize {
    forest.count_hit_trees(1, 1)
        * forest.count_hit_trees(3, 1)
        * forest.count_hit_trees(5, 1)
        * forest.count_hit_trees(7, 1)
        * forest.count_hit_trees(1, 2)
}
