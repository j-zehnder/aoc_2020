use std::error::Error;

fn two_sum(entries: &[usize], sum: usize) -> Result<usize, Box<dyn Error>> {
    for i in 0..entries.len() {
        for j in i..entries.len() {
            if i != j {
                let a = entries.get(i).ok_or("invalid index")?;
                let b = entries.get(j).ok_or("invalid index")?;
                if a + b == sum {
                    return Ok(a * b);
                }
            }
        }
    }
    Err("matching sum not found".into())
}

#[aoc_generator(day9)]
pub fn parse_input(input: &str) -> Vec<usize> {
    input.lines().map(|l| l.parse::<usize>().unwrap()).collect()
}

const PREAMBLE_SIZE: usize = 25;

#[aoc(day9, part1)]
pub fn part1(entries: &[usize]) -> usize {
    let mut preamble: Vec<usize> = Vec::new();

    for i in 0..PREAMBLE_SIZE {
        preamble.push(*entries.get(i).unwrap());
    }

    for i in PREAMBLE_SIZE..entries.len() {
        let cur = entries.get(i).unwrap();

        if let Ok(_n) = two_sum(&preamble, *cur) {
        } else {
            return *cur;
        }

        preamble.remove(0);
        preamble.push(*cur);
    }

    panic!("not found");
}

const P1: usize = 466456641;

#[aoc(day9, part2)]
pub fn part2(entries: &[usize]) -> usize {
    for start_idx in 0..entries.len() {
        let start: usize = *entries.get(start_idx).unwrap();
        let mut min = start;
        let mut max = start;
        let mut sum = start;

        for end_idx in start_idx + 1..entries.len() {
            let cur = *entries.get(end_idx).unwrap();
            sum += cur;

            if cur > max {
                max = cur;
            }

            if cur < min {
                min = cur;
            }

            if sum == P1 {
                return min + max;
            }

            if sum > P1 {
                break;
            }
        }
    }
    panic!("set not found");
}
