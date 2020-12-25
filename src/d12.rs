#[derive(Debug, Copy, Clone)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn manhattan_distance(&self) -> i32 {
        self.y.abs() + self.x.abs()
    }

    fn rotate(&mut self, degrees: i32) {
        match (degrees + 360) % 360 {
            0 => {}
            90 => {
                let temp = self.x;
                self.x = -self.y;
                self.y = temp;
            }
            180 => {
                self.x = -self.x;
                self.y = -self.y;
            }
            270 => {
                let temp = self.x;
                self.x = self.y;
                self.y = -temp;
            }
            _ => panic!("unknown rotation: {:?}", degrees),
        }
    }
}

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::Mul<i32> for Point {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

pub enum Instruction {
    Translation(Point),
    Rotation(i32),
    Move(i32),
}

impl Instruction {
    fn from_str(s: &str) -> Self {
        if let Ok((key, amount)) = scan_fmt!(s, "{[NSWEFLR]}{d}", char, i32) {
            return match key {
                'N' => Instruction::Translation(Point::new(0, amount)),
                'S' => Instruction::Translation(Point::new(0, -amount)),
                'E' => Instruction::Translation(Point::new(amount, 0)),
                'W' => Instruction::Translation(Point::new(-amount, 0)),
                'F' => Instruction::Move(amount),
                'L' => Instruction::Rotation(amount),
                'R' => Instruction::Rotation(-amount),
                _ => panic!("invalid instruction"),
            };
        }
        panic!("unable to parse");
    }
}

struct Ship {
    pos: Point,
    waypoint: Point,
    facing: Point,
}

impl Ship {
    fn new() -> Self {
        Ship {
            pos: Point::new(0, 0),
            waypoint: Point::new(10, 1),
            facing: Point::new(1, 0),
        }
    }

    fn execute_p1(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Translation(point) => self.pos += *point,
            Instruction::Move(amount) => self.pos += self.facing * *amount,
            Instruction::Rotation(amount) => self.facing.rotate(*amount),
        }
    }

    fn execute_p2(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Translation(point) => self.waypoint += *point,
            Instruction::Rotation(amount) => self.waypoint.rotate(*amount),
            Instruction::Move(amount) => self.pos += self.waypoint * *amount,
        }
    }
}

#[aoc_generator(day12)]
pub fn parse_input(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::from_str).collect()
}

#[aoc(day12, part1)]
pub fn part1(instructions: &[Instruction]) -> i32 {
    let mut ship = Ship::new();

    instructions.iter().for_each(|i| ship.execute_p1(i));

    ship.pos.manhattan_distance()
}

#[aoc(day12, part2)]
pub fn part2(instructions: &[Instruction]) -> i32 {
    let mut ship = Ship::new();

    instructions.iter().for_each(|i| ship.execute_p2(i));

    ship.pos.manhattan_distance()
}
