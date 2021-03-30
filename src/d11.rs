const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Debug, PartialEq, Clone)]
pub enum Tile {
    Floor,
    Occupied,
    Empty,
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Tile::Floor,
            'L' => Tile::Empty,
            '#' => Tile::Occupied,
            _ => panic!("unknown input"),
        }
    }
}

fn step_adjacent(tiles: &[Vec<Tile>]) -> Option<Vec<Vec<Tile>>> {
    let mut changed = false;
    let mut new_tiles: Vec<Vec<Tile>> = tiles.to_owned();

    for y in 0..tiles.len() {
        for x in 0..tiles[0].len() {
            match tiles[y][x] {
                Tile::Floor => {}
                Tile::Empty => {
                    // If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
                    if count_occupied_adjacent(tiles, x, y) == 0 {
                        changed = true;
                        new_tiles[y][x] = Tile::Occupied;
                    }
                }
                Tile::Occupied => {
                    // If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
                    if count_occupied_adjacent(&tiles, x, y) >= 4 {
                        changed = true;
                        new_tiles[y][x] = Tile::Empty;
                    }
                }
            }
        }
    }

    if changed {
        Some(new_tiles)
    } else {
        None
    }
}

fn count_occupied_adjacent(tiles: &[Vec<Tile>], x: usize, y: usize) -> usize {
    let height = tiles.len();
    let width = tiles[0].len();

    let mut count = 0;

    for (dx, dy) in DIRECTIONS.iter() {
        let new_x = (x as isize) + dx;
        let new_y = (y as isize) + dy;

        if new_x >= 0
            && (new_x as usize) < width
            && new_y >= 0
            && (new_y as usize) < height
            && tiles[new_y as usize][new_x as usize] == Tile::Occupied
        {
            count += 1;
        }
    }

    count
}

fn step_seen(tiles: &[Vec<Tile>]) -> Option<Vec<Vec<Tile>>> {
    let mut changed = false;
    let mut new_tiles: Vec<Vec<Tile>> = tiles.to_owned();

    for y in 0..tiles.len() {
        for x in 0..tiles[0].len() {
            match tiles[y][x] {
                Tile::Floor => {}
                Tile::Empty => {
                    // If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
                    if count_occupied_seen(&tiles, x, y) == 0 {
                        changed = true;
                        new_tiles[y][x] = Tile::Occupied;
                    }
                }
                Tile::Occupied => {
                    // If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
                    if count_occupied_seen(&tiles, x, y) >= 5 {
                        changed = true;
                        new_tiles[y][x] = Tile::Empty;
                    }
                }
            }
        }
    }

    if changed {
        Some(new_tiles)
    } else {
        None
    }
}

fn count_occupied_seen(tiles: &[Vec<Tile>], x: usize, y: usize) -> usize {
    let height = tiles.len();
    let width = tiles[0].len();

    let mut count = 0;
    for (dx, dy) in DIRECTIONS.iter() {
        let mut new_x: isize = (x as isize) + dx;
        let mut new_y: isize = (y as isize) + dy;

        if new_x < 0 || new_y < 0 {
            continue;
        }

        while (new_x as usize) < width && (new_y as usize) < height {
            match tiles[new_y as usize][new_x as usize] {
                Tile::Floor => {
                    new_x += dx;
                    new_y += dy;
                }
                Tile::Empty => break,
                Tile::Occupied => {
                    count += 1;
                    break;
                }
            }
        }
    }
    count
}

fn total_occupied_tiles(tiles: &[Vec<Tile>]) -> usize {
    let mut count = 0;
    for row in tiles {
        for tile in row {
            if tile == &Tile::Occupied {
                count += 1;
            }
        }
    }
    count
}

#[aoc_generator(day11)]
pub fn parse_input(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|line| line.chars().map(Tile::from_char).collect::<Vec<Tile>>())
        .collect::<Vec<Vec<Tile>>>()
}

#[aoc(day11, part1)]
pub fn part1(tiles: &[Vec<Tile>]) -> usize {
    let map = tiles.to_vec();
    let mut result = Some(map);
    while result != None {
        let map = result.unwrap();
        result = step_adjacent(&map);
        if result == None {
            return total_occupied_tiles(&map);
        }
    }
    panic!("count not found");
}

#[aoc(day11, part2)]
pub fn part2(tiles: &[Vec<Tile>]) -> usize {
    let map = tiles.to_vec();
    let mut result = Some(map);
    while result != None {
        let map = result.unwrap();
        result = step_seen(&map);
        if result == None {
            return total_occupied_tiles(&map);
        }
    }
    panic!("count not found");
}
