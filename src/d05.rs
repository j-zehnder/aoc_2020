use fancy_regex::Regex;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct SeatID(usize);

const RE_BOARDING_PASS: &str = r"^([BF]{7})([LR]{3})$";

impl SeatID {
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

        SeatID((row * 8) + col)
    }
}

#[aoc_generator(day5)]
pub fn parse_input(input: &str) -> Vec<SeatID> {
    input.lines().map(SeatID::from_str).collect()
}

#[aoc(day5, part1)]
pub fn part1(seat_ids: &[SeatID]) -> usize {
    seat_ids.iter().max().unwrap().0
}

#[aoc(day5, part2)]
pub fn part2(seat_ids: &[SeatID]) -> usize {
    let min = seat_ids.iter().min().unwrap().0;
    let max = seat_ids.iter().max().unwrap().0;

    for i in min..=max {
        if !seat_ids.contains(&SeatID(i)) {
            return i;
        }
    }

    panic!("not found")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_SeatID() {
        assert_eq!(SeatID(567), SeatID::from_str("BFFFBBFRRR"));
        assert_eq!(SeatID(119), SeatID::from_str("FFFBBBFRRR"));
        assert_eq!(SeatID(820), SeatID::from_str("BBFFBBFRLL"));
    }
}
