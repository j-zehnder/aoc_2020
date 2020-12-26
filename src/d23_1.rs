use itertools::Itertools;

const STEPS: usize = 100;

fn parse_input(data: &str) -> Vec<usize> {
    data.chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect()
}

fn step(cups: &mut Vec<usize>) {
    //println!("=====");
    //println!("cups: {:?}", cups);

    let current = *cups.get(0).unwrap();
    //println!("current: {}", current);

    let r1 = *cups.get(1).unwrap();
    cups.remove(1);
    let r2 = *cups.get(1).unwrap();
    cups.remove(1);
    let r3 = *cups.get(1).unwrap();
    cups.remove(1);
    //println!("picks up: {}, {}, {}", r1, r2, r3);

    let mut destination = current - 1;

    while !cups.contains(&destination) {
        if destination <= 1 {
            destination = *cups.iter().max().unwrap();
        } else {
            destination -= 1;
        }
    }
    let destination_index = cups.iter().position(|&x| x == destination).unwrap();
    //println!("destination: {} at index: {}", destination, destination_index);

    cups.insert(destination_index + 1, r3);
    cups.insert(destination_index + 1, r2);
    cups.insert(destination_index + 1, r1);

    cups.rotate_left(1);
    //println!("=====");
}

#[aoc(day23, part1)]
pub fn part1(input: &str) -> String {
    let mut cups = parse_input(input);
    for _i in 1..=STEPS {
        step(&mut cups);
    }
    println!("START: {:?}", cups);
    let one_pos = cups.iter().position(|&x| x == 1).unwrap();
    cups.rotate_left(one_pos);
    println!("END: {:?}", cups);

    let mut it = cups.iter();
    it.next();
    it.join("")
}
