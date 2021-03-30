use fancy_regex::Regex;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct SeatId(usize);

const RE_BOARDING_PASS: &str = r"^([BF]{7})([LR]{3})$";

impl SeatId {
    fn from_str(s: &str) -> Self {
        let re = Regex::new(RE_BOARDING_PASS).unwrap();
        let captures = re.captures(s).unwrap().unwrap();
        let row_str = captures.get(1).unwrap().as_str();
        let col_str = captures.get(2).unwrap().as_str();

        let row = usize::from_str_radix(
            row_str
                .chars()
                .map(|c| match c {
                    'F' => '0',
                    'B' => '1',
                    _ => panic!("unknown char"),
                })
                .collect::<String>()
                .as_str(),
            2,
        )
        .unwrap();

        let col = usize::from_str_radix(
            col_str
                .chars()
                .map(|c| match c {
                    'L' => '0',
                    'R' => '1',
                    _ => panic!("unknown char"),
                })
                .collect::<String>()
                .as_str(),
            2,
        )
        .unwrap();

        SeatId((row * 8) + col)
    }
}

#[aoc_generator(day5)]
pub fn parse_input(input: &str) -> Vec<SeatId> {
    input.lines().map(SeatId::from_str).collect()
}

#[aoc(day5, part1)]
pub fn part1(seat_ids: &[SeatId]) -> usize {
    seat_ids.iter().max().unwrap().0
}

#[aoc(day5, part2)]
pub fn part2(seat_ids: &[SeatId]) -> usize {
    let min = seat_ids.iter().min().unwrap().0;
    let max = seat_ids.iter().max().unwrap().0;

    for i in min..=max {
        if !seat_ids.contains(&SeatId(i)) {
            return i;
        }
    }

    panic!("not found")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seat_id() {
        assert_eq!(SeatId(567), SeatId::from_str("BFFFBBFRRR"));
        assert_eq!(SeatId(119), SeatId::from_str("FFFBBBFRRR"));
        assert_eq!(SeatId(820), SeatId::from_str("BBFFBBFRLL"));
    }
}
