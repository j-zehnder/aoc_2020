use std::collections::HashMap;

#[aoc_generator(day15)]
pub fn parse_input(input: &str) -> Vec<usize> {
    input.split(',').map(|s| s.parse::<usize>().unwrap()).collect()
}

#[aoc(day15, part1)]
pub fn part1(seed: &[usize]) -> usize {
    memory_game(seed, 2020)
}

#[aoc(day15, part2)]
pub fn part2(seed: &[usize]) -> usize {
    memory_game(seed, 30000000)
}

fn memory_game(seed: &[usize], nth: usize) -> usize {
    //prime the map
    let mut last_said_map: HashMap<usize, usize> = HashMap::new();
    seed.iter().enumerate().for_each(|i| {
        last_said_map.insert(*i.1, i.0);
    });

    let mut most_recent_said: usize = *seed.get(seed.len() - 1).unwrap();
    last_said_map.remove(&most_recent_said); // because of where we add to the map

    for i in seed.len() - 1..nth - 1 {
        let last_said = last_said_map.get(&most_recent_said);
        let mut to_say = 0;
        if let Some(last_said_index) = last_said {
            to_say = i - last_said_index
        }
        last_said_map.insert(most_recent_said, i);
        most_recent_said = to_say;
    }

    most_recent_said
}

