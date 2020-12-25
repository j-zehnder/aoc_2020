pub struct D13Input {
    earliest_depart: i64,
    busses: Vec<Bus>,
}

pub struct Bus {
    id: i64,
    remainder: i64,
}

#[aoc_generator(day13)]
pub fn parse_input(input: &str) -> D13Input {
    let mut split = input.split('\n');

    let earliest_depart = split.next().unwrap().parse::<i64>().unwrap();

    let mut busses: Vec<Bus> = Vec::new();

    for (i, s) in split.next().unwrap().split(',').enumerate() {
        if s != "x" {
            let id = s.parse::<i64>().unwrap();

            let mut remainder = (id - i as i64) % id;
            while remainder < 0 {
                remainder += id;
            }
            busses.push(Bus { id, remainder });
        }
    }

    D13Input {
        earliest_depart,
        busses,
    }
}

#[aoc(day13, part1)]
pub fn part1(input: &D13Input) -> i64 {
    let mut min_wait = i64::max_value();
    let mut min_id = -1;

    for bus in &input.busses {
        let x = input.earliest_depart / bus.id;
        let x = x + 1;

        let depart = bus.id * x;
        let wait = depart - input.earliest_depart;

        if wait < min_wait {
            min_wait = wait;
            min_id = bus.id;
        }
    }
    min_wait * min_id
}

#[aoc(day13, part2)]
pub fn part2(input: &D13Input) -> i64 {
    let mut step: i64 = 1;
    let max = input.busses.iter().map(|b| b.id).product::<i64>();

    let mut t: i64 = 1;
    let mut i: usize = 0;

    while t < max {
        if t % input.busses[i].id == input.busses[i].remainder {
            step *= input.busses[i].id;
            i += 1;
        }
        if i == input.busses.len() {
            break;
        }
        t += step;
    }
    t
}
