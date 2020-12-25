#[aoc_generator(day1)]
pub fn parse_input(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.parse::<i32>().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part1(entries: &[i32]) -> i32 {
    for i in 0..entries.len() {
        for j in 0..entries.len() {
            if i != j {
                let a = entries.get(i).unwrap();
                let b = entries.get(j).unwrap();
                if a + b == 2020 {
                    return a * b;
                }
            }
        }
    }
    panic!("not found");
}

#[aoc(day1, part2)]
pub fn part2(entries: &[i32]) -> i32 {
    for i in 0..entries.len() {
        for j in 0..entries.len() {
            for k in 0..entries.len() {
                if i != j {
                    let a = entries.get(i).unwrap();
                    let b = entries.get(j).unwrap();
                    let c = entries.get(k).unwrap();
                    if a + b + c == 2020 {
                        return a * b * c;
                    }
                }
            }
        }
    }
    panic!("not found")
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn p1_t1() {
        let entries = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(514579, part1(&entries));
    }
}
