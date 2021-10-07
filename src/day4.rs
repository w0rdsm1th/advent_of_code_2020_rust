/*
project: advent_of_code_2020_rust
created by: aleks
on: 30 August 2021
*/
use std::collections::{HashMap, HashSet};
use lazy_static::lazy_static;
use regex::Regex;

fn split_input(inp: &str) -> Vec<String> {
    // inp.split("\r\n").into_iter().map(|pet| pet.to_owned()).collect()
    let mut split_passports: Vec<String> = vec![];
    let passport_lines: Vec<String> = inp.lines().into_iter().map(|passport| passport.to_owned()).collect();

    // iterate over and combine entries that not separated by the empty string
    let mut assignment_index = 0 as usize;
    for line in &passport_lines {
        let trimmed_line: String = line.trim().to_owned();
        if trimmed_line == "" {
            assignment_index += 1;
        }
        // push and trim leading/trailing whitespace
        if assignment_index >= split_passports.len() {
            split_passports.push(trimmed_line)
        } else {
            split_passports[assignment_index].push_str(" ");
            split_passports[assignment_index].push_str(&trimmed_line);
        }
    };
    split_passports
}

fn convert_map(inp: &str) -> HashMap<String, String> {
    let mut out = HashMap::new();
    for pair in inp.split_whitespace() {
        let split: Vec<&str> = pair.split(':').into_iter().collect();
        out.insert(split[0].to_owned(), split[1].to_owned());
        // println!("split: {:?}", split)
    };
    // dbg!(&out);
    out
}

fn is_valid_passport_part1(inp: &HashMap<String, String>, mandatory: &HashSet<String>) -> bool {
    let passport_keys: HashSet<String> = inp.keys().cloned().collect();
    passport_keys.is_superset(mandatory)
}

fn is_valid_passport_part2(inp: &HashMap<String, String>) -> bool {
    fn parse_numeric(numeric_string: &String) -> u32 {
        numeric_string.parse().unwrap()
    }
    let birth_year = parse_numeric(&inp["byr"]);
    let issue_year = parse_numeric(&inp["iyr"]);
    let expiration_year = parse_numeric(&inp["eyr"]);

    let height = &inp["hgt"];
    // dbg!(&height);

    let (height_bounds, height_val) =
        if height.ends_with("cm") {
            // If cm, the number must be at least 150 and at most 193
            ((150, 193), parse_numeric(&height[..height.len() - "cm".len()].to_owned()))
        } else if height.ends_with("in") {
            // If in, the number must be at least 59 and at most 76
            ((59, 76), parse_numeric(&height[..height.len() - "in".len()].to_owned()))
        } else {
            ((0, 0), 42)  // will never be within bounds
        };

    // a # followed by exactly six characters 0-9 or a-f.
    lazy_static! {
        static ref HAIR_RE: Regex = Regex::new("^#[a-f0-9]{6}$").unwrap();
        static ref PASSPORT_RE: Regex = Regex::new("^[0-9]{9}$").unwrap();
    }
    let hair_condition = HAIR_RE.is_match(&inp["hcl"]);

    // exactly one of: amb blu brn gry grn hzl oth.
    let valid_eyecolors: Vec<String> = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].iter().map(|&key| key.to_owned()).collect();

    // a nine-digit number, including leading zeroes
    let passport_condition = PASSPORT_RE.is_match(&inp["pid"]);

    (inp["byr"].len() == 4) && (1920 <= birth_year) && (2002 >= birth_year) &&
        (inp["iyr"].len() == 4) && (2010 <= issue_year) && (2020 >= issue_year) &&
        (inp["eyr"].len() == 4) && (2020 <= expiration_year) && (2030 >= expiration_year) &&
        birth_year <= issue_year &&
        issue_year <= expiration_year &&
        (height_bounds.0 <= height_val) && (height_bounds.1 >= height_val) &&
        hair_condition &&
        (inp["ecl"].len() == 3) && (valid_eyecolors.contains(&inp["ecl"])) &&
        passport_condition
}


pub fn part1(puzzle_input: &str) -> u32 {
    /*
    Count the number of valid passports - those that have all required fields.
    Treat cid as optional. In your batch file, how many passports are valid?
    */
    let mandatory: HashSet<String> = ["iyr", "byr", "eyr", "hgt", "hcl", "ecl", "pid"].iter().map(|&key| key.to_owned()).collect();

    let parsed_input = split_input(puzzle_input);
    // splitting on the newlines to
    let mut counter = 0_u32;
    for passport in parsed_input {
        if is_valid_passport_part1(&convert_map(&passport), &mandatory) {
            counter += 1;
        }
    }
    println!("part 1 solution: {:?}", &counter);
    counter
}

pub fn part2(puzzle_input: &str) -> u32 {
    /* */
    let mandatory: HashSet<String> = ["iyr", "byr", "eyr", "hgt", "hcl", "ecl", "pid"].iter().map(|&key| key.to_owned()).collect();

    let parsed_input = split_input(puzzle_input);
    let mut counter = 0_u32;
    for passport in parsed_input {
        let passport_map = convert_map(&passport);
        if is_valid_passport_part1(&passport_map, &mandatory) && is_valid_passport_part2(&passport_map) {
            counter += 1;
        }
    }
    println!("part 2 solution: {:?}", &counter);
    counter
}

#[cfg(test)]
mod tests {
    use super::{HashSet, convert_map, is_valid_passport_part1, is_valid_passport_part2, split_input, part1, part2};

    #[test]
    fn test_is_valid_passport_part1() {
        let mandatory: HashSet<String> = ["iyr", "byr", "eyr", "hgt", "hcl", "ecl", "pid"].iter().map(|&pet| pet.to_owned()).collect();

        let valid_passport = r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
    byr:1937 iyr:2017 cid:147 hgt:183cm"#;
        assert_eq!(is_valid_passport_part1(&convert_map(valid_passport), &mandatory), true);

        let invalid_passport = r#"iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929"#;
        assert_eq!(is_valid_passport_part1(&convert_map(invalid_passport), &mandatory), false)
    }

    #[test]
    fn test_split_input() {
        let puzzle_input = r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
        byr:1937 iyr:2017 cid:147 hgt:183cm

        iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
        hcl:#cfa07d byr:1929

        hcl:#ae17e1 iyr:2013
        eyr:2024
        ecl:brn pid:760753108 byr:1931
        hgt:179cm

        hcl:#cfa07d eyr:2025 pid:166559648
        iyr:2011 ecl:brn hgt:59in"#;
        let res = split_input(puzzle_input);
        println!("{:?}", res);
        assert_eq!(res.len(), 4)
    }

    #[test]
    fn test_part1() {
        let puzzle_input = r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
        byr:1937 iyr:2017 cid:147 hgt:183cm

        iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
        hcl:#cfa07d byr:1929

        hcl:#ae17e1 iyr:2013
        eyr:2024
        ecl:brn pid:760753108 byr:1931
        hgt:179cm

        hcl:#cfa07d eyr:2025 pid:166559648
        iyr:2011 ecl:brn hgt:59in"#;
        let res = part1(puzzle_input);
        println!("{:?}", res);
        assert_eq!(res, 2)
    }

    #[test]
    fn test_is_valid_passport_part2() {
//         let valid_passport = r#"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
// hcl:#623a2f"#;
//         assert_eq!(is_valid_passport_part2(&convert_map(valid_passport)), true);

        let newly_valid_passport = r#"eyr:2030 cid:100
hcl:#18171f ecl:oth hgt:170cm pid:012345679 iyr:2018 byr:1926"#;
        assert_eq!(is_valid_passport_part2(&convert_map(newly_valid_passport)), true);

//         let invalid_passport = r#"eyr:1972 cid:100
// hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926"#;
//         assert_eq!(is_valid_passport_part2(&convert_map(invalid_passport)), false)
    }

    #[test]
    fn test_part2() {
        let passports = r#"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007

pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"#;
        assert_eq!(part2(&passports), 4)
    }
}


/* LEARNINGS:
- HashMap and HashSet
- Regex and lazy_static avoids re-compilation every time
- debug statements `dbg!(&height);`
*/