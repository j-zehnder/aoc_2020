use std::collections::{HashMap, HashSet};
use std::hash::Hash;

fn parse_input_3d(input: &str, width: usize, height: usize) -> Conway<Coord3D> {
    let lines = input
        .lines()
        .map(|l| l.to_string())
        .collect::<Vec<String>>();

    let mut coords = Vec::new();
    for y in 0..height {
        let mut line = lines.get(y).unwrap().chars();
        for x in 0..width {
            let c = line.next().unwrap();
            let coord = Coord3D::new(x as i32, y as i32, 0);
            if c == '#' {
                coords.push((coord, State::Active));
            } else {
                coords.push((coord, State::Inactive));
            }
        }
    }

    let mut cube: Conway<Coord3D> = Conway::new();
    for (coord, state) in coords {
        cube.coords.insert(coord, state);
    }
    cube
}

fn parse_input_4d(input: &str, width: usize, height: usize) -> Conway<Coord4D> {
    let lines = input
        .lines()
        .map(|l| l.to_string())
        .collect::<Vec<String>>();

    let mut coords = Vec::new();
    for y in 0..height {
        let mut line = lines.get(y).unwrap().chars();
        for x in 0..width {
            let c = line.next().unwrap();
            let coord = Coord4D::new(x as i32, y as i32, 0, 0);
            if c == '#' {
                coords.push((coord, State::Active));
            } else {
                coords.push((coord, State::Inactive));
            }
        }
    }

    let mut hypercube = Conway::new();
    for (coord, state) in coords {
        hypercube.coords.insert(coord, state);
    }
    hypercube
}

//-------------------------------------------------------------------------------------------------

trait Coord {
    fn get_adjacent(&self) -> Vec<Self>
    where
        Self: Sized;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum State {
    Active,
    Inactive,
}

struct Conway<T>
where
    T: Hash,
    T: Eq,
    T: Coord,
    T: Clone,
{
    coords: HashMap<T, State>,
}

impl<T> Conway<T>
where
    T: Hash,
    T: Eq,
    T: Coord,
    T: Clone,
{
    fn new() -> Self {
        Self {
            coords: HashMap::new(),
        }
    }

    fn total_active_coords(&self) -> usize {
        self.coords
            .values()
            .filter(|v| **v == State::Active)
            .count()
    }

    fn get_coords_to_check(&self) -> HashSet<T> {
        let mut to_check: HashSet<T> = HashSet::new();

        for coord in self.coords.keys() {
            to_check.insert(coord.clone());
            for c in coord.get_adjacent() {
                to_check.insert(c.clone());
            }
        }
        to_check
    }

    fn get_coord_state(&self, coord: &T) -> State {
        if let Some(state) = self.coords.get(&coord) {
            if *state == State::Active {
                return State::Active;
            }
        }
        State::Inactive
    }

    fn count_active_neighbors(&self, origin: &T) -> usize {
        let mut count: usize = 0;
        for coord in origin.get_adjacent() {
            if self.get_coord_state(&coord) == State::Active {
                count += 1;
            }
        }
        count
    }

    fn step(&mut self) {
        let mut new_coords: HashMap<T, State> = HashMap::new();

        for coord in self.get_coords_to_check() {
            let state = self.get_coord_state(&coord);
            let active_neighbor_count = self.count_active_neighbors(&coord);
            match state {
                State::Active => {
                    if active_neighbor_count == 2 || active_neighbor_count == 3 {
                        new_coords.insert(coord, State::Active);
                    } else {
                        new_coords.insert(coord, State::Inactive);
                    }
                }
                State::Inactive => {
                    if active_neighbor_count == 3 {
                        new_coords.insert(coord, State::Active);
                    }
                }
            }
        }
        self.coords = new_coords;
    }

    fn solve(&mut self) -> usize {
        for _i in 0..6 {
            self.step();
        }
        self.total_active_coords()
    }
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
struct Coord3D {
    x: i32,
    y: i32,
    z: i32,
}

impl Coord3D {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

impl Coord for Coord3D {
    fn get_adjacent(&self) -> Vec<Self>
    where
        Self: Sized,
    {
        let mut coords: Vec<Self> = Vec::new();
        for x in -1..2 {
            for y in -1..2 {
                for z in -1..2 {
                    if x == 0 && y == 0 && z == 0 {
                        continue;
                    }
                    coords.push(Self::new(self.x + x, self.y + y, self.z + z));
                }
            }
        }
        coords
    }
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
struct Coord4D {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

impl Coord4D {
    fn new(x: i32, y: i32, z: i32, w: i32) -> Self {
        Self { x, y, z, w }
    }
}

impl Coord for Coord4D {
    fn get_adjacent(&self) -> Vec<Self> {
        let mut coords: Vec<Self> = Vec::new();
        for x in -1..2 {
            for y in -1..2 {
                for z in -1..2 {
                    for w in -1..2 {
                        if x == 0 && y == 0 && z == 0 && w == 0 {
                            continue;
                        }
                        coords.push(Self::new(self.x + x, self.y + y, self.z + z, self.w + w));
                    }
                }
            }
        }
        coords
    }
}

#[aoc(day17, part1)]
pub fn part1(input: &str) -> usize {
    let mut conway3d = parse_input_3d(input, 8, 8);
    conway3d.solve()
}

#[aoc(day17, part2)]
pub fn part2(input: &str) -> usize {
    let mut conway4d = parse_input_4d(input, 8, 8);
    conway4d.solve()
}
