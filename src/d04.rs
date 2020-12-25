use fancy_regex::Regex;

#[derive(Debug, Default)]
pub struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
}

impl Passport {
    fn from_str(s: &str) -> Self {
        let mut passport: Passport = Default::default();
        for kv in s.split(&[' ', '\n'][..]) {
            if let Ok((key, val)) = scan_fmt!(kv, "{}:{}", String, String) {
                let key = key.as_str();
                match key {
                    "byr" => passport.byr = Some(val),
                    "iyr" => passport.iyr = Some(val),
                    "eyr" => passport.eyr = Some(val),
                    "hgt" => passport.hgt = Some(val),
                    "hcl" => passport.hcl = Some(val),
                    "ecl" => passport.ecl = Some(val),
                    "pid" => passport.pid = Some(val),
                    "cid" => {}
                    _ => {
                        panic!("unknown key");
                    }
                }
            }
        }
        passport
    }

    fn valid_p1(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn valid_p2(&self) -> bool {
        if !self.valid_p1() {
            return false;
        }

        // byr (Birth Year) - four digits; at least 1920 and at most 2002.
        let byr = self
            .byr
            .as_ref()
            .expect("expected byr")
            .parse::<usize>()
            .expect("parse error byr");
        let byr_valid = byr >= 1920 && byr <= 2002;

        // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
        let iyr = self
            .iyr
            .as_ref()
            .expect("expected iyr")
            .parse::<usize>()
            .expect("parse error iyr");
        let iyr_valid = iyr >= 2010 && iyr <= 2020;

        // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
        let eyr = self
            .eyr
            .as_ref()
            .expect("expected eyr")
            .parse::<usize>()
            .expect("parse error eyr");
        let eyr_valid = eyr >= 2020 && eyr <= 2030;

        // hgt (Height) - a number followed by either cm or in:
        // If cm, the number must be at least 150 and at most 193.
        // If in, the number must be at least 59 and at most 76.
        let mut hgt_valid = false;
        let re_hgt = Regex::new(r"^(\d{2,3})(cm|in)$").unwrap();
        let hgt_valid_p1 = re_hgt
            .is_match(self.hgt.as_ref().expect("expected hgt"))
            .unwrap();
        if hgt_valid_p1 {
            let captures = re_hgt
                .captures(self.hgt.as_ref().expect("expected hgt"))
                .unwrap()
                .unwrap();
            if captures.get(2).expect("expected cm/in").as_str() == "cm" {
                let hgt = captures
                    .get(1)
                    .expect("expected a number")
                    .as_str()
                    .parse::<usize>()
                    .expect("expected a number");
                if hgt >= 150 && hgt <= 193 {
                    hgt_valid = true;
                }
            } else if captures.get(2).expect("expected cm/in").as_str() == "in" {
                let hgt = captures
                    .get(1)
                    .expect("expected a number")
                    .as_str()
                    .parse::<usize>()
                    .expect("expected a number");
                if hgt >= 59 && hgt <= 76 {
                    hgt_valid = true;
                }
            } else {
                panic!("invalid units");
            }
        }

        // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
        let re_hcl = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        let hcl_valid = re_hcl
            .is_match(self.hcl.as_ref().expect("expected hcl"))
            .unwrap();

        // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
        let re_ecl = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
        let ecl_valid = re_ecl
            .is_match(self.ecl.as_ref().expect("expected ecl"))
            .unwrap();

        // pid (Passport ID) - a nine-digit number, including leading zeroes.
        let re_pid = Regex::new(r"^\d{9}$").unwrap();
        let pid_valid = re_pid
            .is_match(self.pid.as_ref().expect("expected pid"))
            .unwrap();

        byr_valid && iyr_valid && eyr_valid && hgt_valid && hcl_valid && ecl_valid && pid_valid
    }
}

#[aoc_generator(day4)]
pub fn parse_input(input: &str) -> Vec<Passport> {
    input.split("\n\n").map(Passport::from_str).collect()
}

#[aoc(day4, part1)]
pub fn part1(passports: &[Passport]) -> usize {
    passports.iter().filter(|p| p.valid_p1()).count()
}

#[aoc(day4, part2)]
pub fn part2(passports: &[Passport]) -> usize {
    passports.iter().filter(|p| p.valid_p2()).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_t1() {
        let passports = parse_input(SAMPLE_1);
        assert_eq!(2, part1(&passports));
    }

    #[test]
    fn p2_t1() {
        let passports = parse_input(SAMPLE_2);
        assert_eq!(4, part2(&passports));
    }

    #[test]
    fn p2_t2() {
        let passports = parse_input(SAMPLE_3);
        assert_eq!(0, part2(&passports));
    }

    const SAMPLE_1: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    const SAMPLE_2: &str = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

    const SAMPLE_3: &str = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";
}
