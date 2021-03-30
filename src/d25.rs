type PubKey = usize;

fn transform_subject(num: usize, loop_size: usize) -> usize {
    let mut result = 1;

    for _ in 0..loop_size {
        result *= num;
        result %= 20201227;
    }

    result
}

fn determine_loop_size(key: PubKey) -> usize {
    let mut value = 1;
    let mut loop_size = 0;
    while key != value {
        loop_size += 1;
        value *= 7;
        value %= 20201227;
    }
    loop_size
}

#[aoc_generator(day25)]
pub fn parse_input(input: &str) -> Vec<PubKey> {
    input
        .lines()
        .map(|l| l.parse::<PubKey>().unwrap())
        .collect()
}

#[aoc(day25, part1)]
pub fn part1(keys: &[PubKey]) -> usize {
    let card_pub_key = keys[0];
    let door_pub_key = keys[1];

    let card_loop_size = determine_loop_size(card_pub_key);
    let door_loop_size = determine_loop_size(door_pub_key);

    let ek1 = transform_subject(door_pub_key, card_loop_size);
    let ek2 = transform_subject(card_pub_key, door_loop_size);
    assert_eq!(ek1, ek2);

    ek1
}

// #[aoc(day25, part2)]
// pub fn part2() -> usize {
//
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_t1() {
        assert_eq!(8, determine_loop_size(5764801));
        assert_eq!(11, determine_loop_size(17807724));
        assert_eq!(14897079, part1(&vec![5764801, 17807724]));
    }
}
