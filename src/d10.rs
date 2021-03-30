use itertools::sorted;

#[aoc_generator(day10)]
pub fn parse_input(input: &str) -> Vec<usize> {
    let mut entries = input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    // add in the first and last values (not in input file...)
    entries.push(0);
    entries.push(entries.iter().max().unwrap() + 3);

    sorted(entries).as_slice().to_vec()
}

#[aoc(day10, part1)]
pub fn part1(entries: &[usize]) -> usize {
    let mut one_ct = 0;
    let mut three_ct = 0;

    // use windows to avoid prev/cur moving pointers
    entries.windows(2).for_each(|w| {
        if w[1] == w[0] + 1 {
            one_ct += 1
        } else if w[1] == w[0] + 3 {
            three_ct += 1;
        } else {
            panic!("unknown gap");
        }
    });

    one_ct * three_ct
}

#[aoc(day10, part2)]
pub fn part2(entries: &[usize]) -> usize {
    // split into chains of sequential numbers
    let mut chains: Vec<Vec<usize>> = Vec::new();
    let mut cur_chain: Vec<usize> = vec![0];

    entries.windows(2).for_each(|w| {
        if w[1] == w[0] + 3 {
            chains.push(cur_chain.to_owned());
            cur_chain = Vec::new();
        }
        cur_chain.push(w[1]);
    });
    chains.push(cur_chain);

    // each chain of sequential numbers creates a set of combinations
    // each 3-gap has only 1 possibility
    chains
        .iter()
        .map(|chain| match chain.len() {
            1 => 1,
            2 => 1,
            3 => 2,
            4 => 4,
            5 => 7,
            _ => panic!("not found"),
        })
        .product()
}
