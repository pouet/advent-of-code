use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct PassportString {
    //(Birth Year)
    byr: Option<String>,
    //(Issue Year)
    iyr: Option<String>,
    //(Expiration Year)
    eyr: Option<String>,
    //(Height)
    hgt: Option<String>,
    //(Hair Color)
    hcl: Option<String>,
    //(Eye Color)
    ecl: Option<String>,
    //(Passport ID)
    pid: Option<String>,
    //(Country ID)
    cid: Option<String>,
}

#[derive(Debug, PartialEq)]
pub enum HeightUnit {
    CM,
    IN,
}

#[derive(Debug, PartialEq)]
pub struct Height {
    unit: HeightUnit,
    size: usize,
}

#[derive(Debug, PartialEq)]
pub struct Passport {
    //(Birth Year)
    byr: Option<usize>,
    //(Issue Year)
    iyr: Option<usize>,
    //(Expiration Year)
    eyr: Option<usize>,
    //(Height)
    hgt: Option<Height>,
    //(Hair Color)
    hcl: Option<String>,
    //(Eye Color)
    ecl: Option<String>,
    //(Passport ID)
    pid: Option<usize>,
    //(Country ID)
    cid: Option<usize>,
}

pub struct PassportError;

impl PassportString {
    fn new() -> PassportString {
        PassportString {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }

    fn set_field_from(self, key: &str, value: &str) -> PassportString {
        match &key[..] {
            "byr" => PassportString { byr: Some(value.to_string()), ..self },
            "iyr" => PassportString { iyr: Some(value.to_string()), ..self },
            "eyr" => PassportString { eyr: Some(value.to_string()), ..self },
            "hgt" => PassportString { hgt: Some(value.to_string()), ..self },
            "hcl" => PassportString { hcl: Some(value.to_string()), ..self },
            "ecl" => PassportString { ecl: Some(value.to_string()), ..self },
            "pid" => PassportString { pid: Some(value.to_string()), ..self },
            "cid" => PassportString { cid: Some(value.to_string()), ..self },
            _ => panic!("Invalid key {}", key)
        }
    }

    fn is_valid(&self) -> bool {
        match self {
            PassportString {
                byr: Some(_),
                iyr: Some(_),
                eyr: Some(_),
                hgt: Some(_),
                hcl: Some(_),
                ecl: Some(_),
                pid: Some(_),
                ..
            } => true,
            _ => false
        }
    }
}

impl Passport {
    fn new() -> Passport {
        Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }

    fn get_byr_from(value: &str) -> Option<usize> {
        value
            .parse()
            .ok()
            .filter(|&n| n >= 1920 && n <= 2002)
    }

    fn get_iyr_from(value: &str) -> Option<usize> {
        value
            .parse()
            .ok()
            .filter(|&n| n >= 2010 && n <= 2020)
    }

    fn get_eyr_from(value: &str) -> Option<usize> {
        value
            .parse()
            .ok()
            .filter(|&n| n >= 2020 && n <= 2030)
    }

    fn get_hgt_from(value: &str) -> Option<Height> {
        let len = value.len();
        if len <= 2 {
            None
        } else {
            let size: &Option<usize> = &value[..len - 2]
                .parse()
                .ok();
            match &value[len - 2..len] {
                "cm" => size
                    .filter(|&n| n >= 150 && n <= 193)
                    .map(|n| Height {
                        unit: HeightUnit::CM,
                        size: n,
                    }),
                "in" => size
                    .filter(|&n| n >= 59 && n <= 76)
                    .map(|n| Height {
                        unit: HeightUnit::IN,
                        size: n,
                    }),
                _ => None
            }
        }
    }

    fn get_hcl_from(value: &str) -> Option<String> {
        Some(value)
            .filter(|&s| s.len() == 7)
            .filter(|&s| &s[0..1] == "#")
            .filter(|&s| {
                s[1..]
                    .chars()
                    .into_iter()
                    .all(|c| (c.is_numeric() || c.is_lowercase()) && c.is_ascii_hexdigit())
            })
            .map(|s| s.to_string())
    }

    fn get_ecl_from(value: &str) -> Option<String> {
        match value {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => Some(value.to_string()),
            _ => None
        }
    }

    fn get_pid_from(value: &str) -> Option<usize> {
        if value.len() != 9 {
            None
        } else {
            value
                .parse()
                .ok()
        }
    }

    fn get_cid_from(value: &str) -> Option<usize> {
        value
            .parse()
            .ok()
    }

    fn set_field_from(self, key: &str, value: &str) -> Passport {
        match &key[..] {
            "byr" => Passport { byr: Passport::get_byr_from(value), ..self },
            "iyr" => Passport { iyr: Passport::get_iyr_from(value), ..self },
            "eyr" => Passport { eyr: Passport::get_eyr_from(value), ..self },
            "hgt" => Passport { hgt: Passport::get_hgt_from(value), ..self },
            "hcl" => Passport { hcl: Passport::get_hcl_from(value), ..self },
            "ecl" => Passport { ecl: Passport::get_ecl_from(value), ..self },
            "pid" => Passport { pid: Passport::get_pid_from(value), ..self },
            "cid" => Passport { cid: Passport::get_cid_from(value), ..self },
            _ => panic!("Invalid key {}", key)
        }
    }

    fn is_valid(&self) -> bool {
        match self {
            Passport {
                byr: Some(_),
                iyr: Some(_),
                eyr: Some(_),
                hgt: Some(_),
                hcl: Some(_),
                ecl: Some(_),
                pid: Some(_),
                ..
            } => true,
            _ => false
        }
    }
}

impl FromStr for PassportString {
    type Err = PassportError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s
            .replace("\n", " ")
            .split(" ")
            .fold(PassportString::new(), |acc, val| {
                let sp: Vec<_> = val.split(":").collect();
                acc.set_field_from(&sp[0], &sp[1])
            },
            )
        )
    }
}

impl FromStr for Passport {
    type Err = PassportError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s
            .replace("\n", " ")
            .split(" ")
            .fold(Passport::new(), |acc, val| {
                let sp: Vec<_> = val.split(":").collect();
                acc.set_field_from(&sp[0], &sp[1])
            },
            )
        )
    }
}

#[aoc_generator(day4, part1)]
pub fn gen_part1(input: &str) -> Vec<PassportString> {
    input
        .split("\n\n")
        .flat_map(|s| s.parse())
        .collect()
}

#[aoc(day4, part1)]
pub fn solve_part1(passports: &Vec<PassportString>) -> usize {
    passports
        .iter()
        .filter(|p| p.is_valid())
        .count()
}

#[aoc_generator(day4, part2)]
pub fn gen_part2(input: &str) -> Vec<Passport> {
    input
        .split("\n\n")
        .flat_map(|s| s.parse())
        .collect()
}

#[aoc(day4, part2)]
pub fn solve_part2(passports: &Vec<Passport>) -> usize {
    passports
        .iter()
        .filter(|p| p.is_valid())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> &'static str {
        return "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";
    }

    #[test]
    fn test_gen1() {
        let p1 = PassportString {
            byr: Some(String::from("1937")),
            iyr: Some(String::from("2017")),
            eyr: Some(String::from("2020")),
            hgt: Some(String::from("183cm")),
            hcl: Some(String::from("#fffffd")),
            ecl: Some(String::from("gry")),
            pid: Some(String::from("860033327")),
            cid: Some(String::from("147")),
        };

        let p2 = PassportString {
            byr: Some(String::from("1929")),
            iyr: Some(String::from("2013")),
            eyr: Some(String::from("2023")),
            hgt: None,
            hcl: Some(String::from("#cfa07d")),
            ecl: Some(String::from("amb")),
            pid: Some(String::from("028048884")),
            cid: Some(String::from("350")),
        };

        let p3 = PassportString {
            hcl: Some(String::from("#ae17e1")),
            iyr: Some(String::from("2013")),
            eyr: Some(String::from("2024")),
            ecl: Some(String::from("brn")),
            pid: Some(String::from("760753108")),
            byr: Some(String::from("1931")),
            hgt: Some(String::from("179cm")),
            cid: None,
        };

        let p4 = PassportString {
            byr: None,
            iyr: Some(String::from("2011")),
            eyr: Some(String::from("2025")),
            hgt: Some(String::from("59in")),
            hcl: Some(String::from("#cfa07d")),
            ecl: Some(String::from("brn")),
            pid: Some(String::from("166559648")),
            cid: None,
        };

        let expected = vec![p1, p2, p3, p4];
        assert_eq!(gen_part1(&get_input()), expected);
    }

    #[test]
    fn test_gen2() {
        let p1 = Passport {
            byr: Some(1937),
            iyr: Some(2017),
            eyr: Some(2020),
            hgt: Some(Height { unit: HeightUnit::CM, size: 183 }),
            hcl: Some(String::from("#fffffd")),
            ecl: Some(String::from("gry")),
            // pid: Some(String::from("860033327")),
            pid: Some(860033327),
            cid: Some(147),
        };

        let p2 = Passport {
            byr: Some(1929),
            iyr: Some(2013),
            eyr: Some(2023),
            hgt: None,
            hcl: Some(String::from("#cfa07d")),
            ecl: Some(String::from("amb")),
            pid: Some(028048884),
            cid: Some(350),
        };

        let p3 = Passport {
            hcl: Some(String::from("#ae17e1")),
            iyr: Some(2013),
            eyr: Some(2024),
            ecl: Some(String::from("brn")),
            pid: Some(760753108),
            byr: Some(1931),
            hgt: Some(Height { unit: HeightUnit::CM, size: 179 }),
            cid: None,
        };

        let p4 = Passport {
            byr: None,
            iyr: Some(2011),
            eyr: Some(2025),
            hgt: Some(Height { unit: HeightUnit::IN, size: 59 }),
            hcl: Some(String::from("#cfa07d")),
            ecl: Some(String::from("brn")),
            pid: Some(166559648),
            cid: None,
        };

        let expected = vec![p1, p2, p3, p4];
        assert_eq!(gen_part2(&get_input()), expected);
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&gen_part1(get_input())), 2);
    }

    #[test]
    fn test_byr() {
        assert_eq!(Passport::get_byr_from("2002"), Some(2002));
        assert_eq!(Passport::get_byr_from("2003"), None);
        assert_eq!(Passport::get_byr_from("1920"), Some(1920));
        assert_eq!(Passport::get_byr_from("1919"), None);
        assert_eq!(Passport::get_byr_from("1"), None);
        assert_eq!(Passport::get_byr_from("12345"), None);
    }

    #[test]
    fn test_iyr() {
        assert_eq!(Passport::get_iyr_from("2010"), Some(2010));
        assert_eq!(Passport::get_iyr_from("2009"), None);
        assert_eq!(Passport::get_iyr_from("2020"), Some(2020));
        assert_eq!(Passport::get_iyr_from("2021"), None);
        assert_eq!(Passport::get_iyr_from("1"), None);
        assert_eq!(Passport::get_iyr_from("12345"), None);
    }

    #[test]
    fn test_eyr() {
        assert_eq!(Passport::get_eyr_from("2020"), Some(2020));
        assert_eq!(Passport::get_eyr_from("2019"), None);
        assert_eq!(Passport::get_eyr_from("2030"), Some(2030));
        assert_eq!(Passport::get_eyr_from("2031"), None);
        assert_eq!(Passport::get_eyr_from("1"), None);
        assert_eq!(Passport::get_eyr_from("12345"), None);
    }

    #[test]
    fn test_hgt() {
        assert_eq!(Passport::get_hgt_from("60in"), Some(Height { unit: HeightUnit::IN, size: 60 }));
        assert_eq!(Passport::get_hgt_from("190cm"), Some(Height { unit: HeightUnit::CM, size: 190 }));
        assert_eq!(Passport::get_hgt_from("190in"), None);
        assert_eq!(Passport::get_hgt_from("190"), None);

        assert_eq!(Passport::get_hgt_from("150cm"), Some(Height { unit: HeightUnit::CM, size: 150 }));
        assert_eq!(Passport::get_hgt_from("193cm"), Some(Height { unit: HeightUnit::CM, size: 193 }));
        assert_eq!(Passport::get_hgt_from("59in"), Some(Height { unit: HeightUnit::IN, size: 59 }));
        assert_eq!(Passport::get_hgt_from("76in"), Some(Height { unit: HeightUnit::IN, size: 76 }));

        assert_eq!(Passport::get_hgt_from("149cm"), None);
        assert_eq!(Passport::get_hgt_from("194cm"), None);
        assert_eq!(Passport::get_hgt_from("58in"), None);
        assert_eq!(Passport::get_hgt_from("77in"), None);

        assert_eq!(Passport::get_hgt_from("cm"), None);
        assert_eq!(Passport::get_hgt_from("in"), None);
        assert_eq!(Passport::get_hgt_from("150cmi"), None);
        assert_eq!(Passport::get_hgt_from("59ini"), None);
        assert_eq!(Passport::get_hgt_from("123123cm"), None);
        assert_eq!(Passport::get_hgt_from("123123in"), None);
    }

    #[test]
    fn test_hcl() {
        assert_eq!(Passport::get_hcl_from("#123abc"), Some("#123abc".to_string()));
        assert_eq!(Passport::get_hcl_from("#123abz"), None);
        assert_eq!(Passport::get_hcl_from("123abc"), None);
        assert_eq!(Passport::get_hcl_from("#123abcd"), None);
    }

    #[test]
    fn test_ecl() {
        assert_eq!(Passport::get_ecl_from("amb"), Some("amb".to_string()));
        assert_eq!(Passport::get_ecl_from("blu"), Some("blu".to_string()));
        assert_eq!(Passport::get_ecl_from("brn"), Some("brn".to_string()));
        assert_eq!(Passport::get_ecl_from("gry"), Some("gry".to_string()));
        assert_eq!(Passport::get_ecl_from("grn"), Some("grn".to_string()));
        assert_eq!(Passport::get_ecl_from("hzl"), Some("hzl".to_string()));
        assert_eq!(Passport::get_ecl_from("oth"), Some("oth".to_string()));

        assert_eq!(Passport::get_ecl_from("yolo"), None);
        assert_eq!(Passport::get_ecl_from("test"), None);
        assert_eq!(Passport::get_ecl_from(""), None);
    }

    #[test]
    fn test_pid() {
        assert_eq!(Passport::get_pid_from("000000000"), Some(0));
        assert_eq!(Passport::get_pid_from("000000001"), Some(1));
        assert_eq!(Passport::get_pid_from("000000010"), Some(10));
        assert_eq!(Passport::get_pid_from("000000100"), Some(100));
        assert_eq!(Passport::get_pid_from("100000000"), Some(100000000));

        assert_eq!(Passport::get_pid_from("1000000000"), None);
        assert_eq!(Passport::get_pid_from("10000000"), None);
        assert_eq!(Passport::get_pid_from(""), None);
        assert_eq!(Passport::get_pid_from("abcdefghi"), None);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(&gen_part2(get_input())), 2);
    }
}