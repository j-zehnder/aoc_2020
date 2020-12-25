use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use Dir::{East, NorthEast, NorthWest, SouthEast, SouthWest, West};

#[derive(Debug, Copy, Clone)]
pub enum Dir {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

impl Dir {
    fn from_str(s: &str) -> Self {
        match s {
            "e" => East,
            "se" => SouthEast,
            "sw" => SouthWest,
            "w" => West,
            "nw" => NorthWest,
            "ne" => NorthEast,
            _ => panic!("unknown [{}]", s),
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct HexCoord {
    x: i32,
    y: i32,
    z: i32,
}

impl HexCoord {
    fn origin() -> Self {
        HexCoord { x: 0, y: 0, z: 0 }
    }

    fn get_neighbor(&self, dir: Dir) -> HexCoord {
        match dir {
            East => HexCoord {
                x: self.x + 1,
                y: self.y - 1,
                z: self.z,
            },
            SouthEast => HexCoord {
                x: self.x,
                y: self.y - 1,
                z: self.z + 1,
            },
            SouthWest => HexCoord {
                x: self.x - 1,
                y: self.y,
                z: self.z + 1,
            },
            West => HexCoord {
                x: self.x - 1,
                y: self.y + 1,
                z: self.z,
            },
            NorthWest => HexCoord {
                x: self.x,
                y: self.y + 1,
                z: self.z - 1,
            },
            NorthEast => HexCoord {
                x: self.x + 1,
                y: self.y,
                z: self.z - 1,
            },
        }
    }
}

impl Coord for HexCoord {
    fn get_adjacent(&self) -> Vec<Self>
    where
        Self: Sized,
    {
        let mut adjacent: Vec<Self> = Vec::new();

        adjacent.push(self.get_neighbor(East));
        adjacent.push(self.get_neighbor(SouthEast));
        adjacent.push(self.get_neighbor(SouthWest));
        adjacent.push(self.get_neighbor(West));
        adjacent.push(self.get_neighbor(NorthEast));
        adjacent.push(self.get_neighbor(NorthWest));

        adjacent
    }
}

fn seed_board_for_active(tile_directions: &[Vec<Dir>]) -> Vec<HexCoord> {
    let mut tile_state: HashMap<HexCoord, bool> = HashMap::new();

    for directions in tile_directions {
        let mut tile = HexCoord::origin();

        for dir in directions {
            tile = tile.get_neighbor(*dir);
        }

        if tile_state.contains_key(&tile) {
            let state = tile_state.get_mut(&tile).unwrap();
            *state ^= true;
        } else {
            tile_state.insert(tile, true);
        }
    }

    tile_state
        .iter()
        .filter(|kv| *kv.1)
        .map(|kv| kv.0)
        .cloned()
        .collect::<Vec<HexCoord>>()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum State {
    Active,
    Inactive,
}

trait Coord {
    fn get_adjacent(&self) -> Vec<Self>
    where
        Self: Sized;
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
                    if active_neighbor_count == 0 || active_neighbor_count > 2 {
                        new_coords.insert(coord, State::Inactive);
                    } else {
                        new_coords.insert(coord, State::Active);
                    }
                }
                State::Inactive => {
                    if active_neighbor_count == 2 {
                        new_coords.insert(coord, State::Active);
                    }
                }
            }
        }
        self.coords = new_coords;
    }
}

#[aoc_generator(day24)]
pub fn parse_input(input: &str) -> Vec<Vec<Dir>> {
    input
        .lines()
        .map(|line| {
            let modified = line.replace("e", "e,").replace("w", "w,");

            modified[0..modified.len() - 1]
                .split(',')
                .map(Dir::from_str)
                .collect::<Vec<Dir>>()
        })
        .collect()
}

#[aoc(day24, part1)]
pub fn part1(tile_directions: &[Vec<Dir>]) -> usize {
    seed_board_for_active(&tile_directions).iter().count()
}

#[aoc(day24, part2)]
pub fn part2(tile_directions: &[Vec<Dir>]) -> usize {
    let active_coords = seed_board_for_active(tile_directions);

    let mut conway: Conway<HexCoord> = Conway::new();
    for coord in active_coords {
        conway.coords.insert(coord, State::Active);
    }

    for _i in 0..100 {
        conway.step();
        //println!("day {}: {} active tiles", i + 1, conway.total_active_coords());
    }

    conway.total_active_coords()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_t1() {
        let directions = parse_input(SAMPLE_INPUT);
        let p1 = part1(&directions);
        assert_eq!(p1, 10);
    }

    #[test]
    fn p2_t1() {
        let directions = parse_input(SAMPLE_INPUT);
        let p2 = part2(&directions);
        assert_eq!(p2, 2208);
    }

    const SAMPLE_INPUT: &str = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";
}
