/*
project: advent_of_code_2020_rust
created by: aleks
on: 04 November 2021
*/

use std::collections::{HashMap, HashSet};
use std::any::Any;
use regex::Regex;
use lazy_static::lazy_static;

/*
algorithm summary
- convert the string relationships into structured entities
- consider bag and bags? harms the lookup

discard but probably consider the number prefixes for part 2
*/

// todo - assumption that can only contain single digit number of bags? e.g. <10
lazy_static! {
    static ref outer_bag_regex: Regex = Regex::new(r"(?P<adjective>\w[a-z]+) (?P<colour>\w[a-z]+) bags contain").unwrap();
    static ref inner_bag_regex: Regex = Regex::new(r"(?P<qty>\d) (?P<adjective>\w[a-z]+) (?P<colour>\w[a-z]+) bag").unwrap();
}


#[derive(Debug)]
#[derive(PartialEq)]
struct Bag {
    adjective: String,
    colour: String,
}

#[derive(Debug)]
#[derive(PartialEq)]
struct BagRelation {
    outer: Bag,
    quantity: i8,
    inner: Bag,
}

fn parse_input(inp: &str) -> Vec<BagRelation> {
    let mut out = vec![];
    for row in inp.lines() {
        let iter_out = build_bag_relation(row);
        out.extend(iter_out);
    }
    out
}


fn build_bag_relation(bag_str: &str) -> Vec<BagRelation> {
    // regex matches bag_str and creates BagRelations out of it
    let mut out = vec![];

    let outer_bag_capture = outer_bag_regex.captures(bag_str).unwrap();

    for bag in inner_bag_regex.captures_iter(bag_str) {
        let outer_bag = Bag {
            adjective: outer_bag_capture["adjective"].to_owned(),
            colour: outer_bag_capture["colour"].to_owned(),
        };
        let inner_bag = Bag {
            adjective: bag["adjective"].to_owned(),
            colour: bag["colour"].to_owned(),
        };
        out.push(BagRelation {
            outer: outer_bag,
            quantity: bag["qty"].parse::<i8>().unwrap(),
            inner: inner_bag,
        })
    }
    out
}


fn find_outer_bags(inner_bag: &Bag, search_space: &Vec<BagRelation>,
                   found_outer: &mut HashSet<String>) -> usize {
    // for a given inner bag, find the outer bags which can contain them
    for bag_relation in search_space {
        if &bag_relation.inner == inner_bag {
            println!("found! {:?}", &bag_relation.outer);
            found_outer.insert(format!("{} {}", &bag_relation.outer.adjective, &bag_relation.outer.colour));
            find_outer_bags(&bag_relation.outer, search_space, found_outer);
        }
    }
    found_outer.len()
}


fn count_inner_bags(outer_bag: &Bag, search_space: &Vec<BagRelation>,
                    counter: &u32) -> u32 {
    // for a given outer bag, count the number of inner bags it holds and recurse
    let mut running_total = 0;
    for bag_relation in search_space {
        if &bag_relation.outer == outer_bag {
            println!("a {:?} bag contains {:?} inner {:?} bags", &bag_relation.outer, &bag_relation.quantity, &bag_relation.inner);
            let additional = bag_relation.quantity as u32 + count_inner_bags(&bag_relation.inner, search_space, &(bag_relation.quantity as u32));
            running_total = running_total + counter.to_owned() * additional;
        }
    }
    running_total
}

pub fn part1(inp: &str) -> usize {
    /* How many bag colors can eventually contain at least one shiny gold bag? */
    let mut unique_outer_bags: HashSet<String> = HashSet::new();
    let bag_relations = parse_input(inp);
    let needle = Bag { adjective: "shiny".to_owned(), colour: "gold".to_owned() };
    // do the loop!
    let count = find_outer_bags(&needle,
                                &bag_relations,
                                &mut unique_outer_bags);

    println!("{} set of possible containing bags are {:?}", &count, &unique_outer_bags);
    count
}

pub fn part2(inp: &str) -> u32 {
    /* How many individual bags are required inside your single shiny gold bag? */
    let bag_relations = parse_input(inp);
    let needle = Bag { adjective: "shiny".to_owned(), colour: "gold".to_owned() };

    let count = count_inner_bags(&needle,
                                 &bag_relations,
                                 &1);
    println!("a {:?} bag must contain {:?} other bags.", &needle, &count);
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex_pattern() {
        let fixture = r#"light red bags contain 1 bright white bag, 2 muted yellow bags."#;
        // https://stackoverflow.com/a/58010784/3596968
        let outer_bag = outer_bag_regex.captures(fixture).unwrap();
        // find_iter returns string Matches
        let inner_bags: Vec<String> = inner_bag_regex.find_iter(fixture)
            // try to parse the string matches as i64 (inferred from fn type signature)
            // and filter out the matches that can't be parsed (e.g. if there are too many digits to store in an i64).
            .filter_map(|_match| _match.as_str().parse().ok())
            // collect the results in to a Vec<String> (inferred from fn type signature)
            .collect();

        assert_eq!(format!("{} {}", &outer_bag["adjective"], &outer_bag["colour"]), "light red");
        assert_eq!(inner_bags, ["1 bright white bag", "2 muted yellow bag"])
        // capture_iter returns CaptureMatches - yields information about the capture groups
        // let out: CaptureMatches = inner_bag_regex.captures_iter(fixture);
    }

    #[test]
    fn test_build_bag_relation() {
        let fixture = r#"light red bags contain 1 bright white bag, 2 muted yellow bags."#;
        let data_struct = build_bag_relation(&fixture);
        assert_eq!(data_struct, vec![BagRelation {
            outer: Bag {
                adjective: "light".to_owned(),
                colour: "red".to_owned(),
            },
            quantity: 1,
            inner: Bag {
                adjective: "bright".to_owned(),
                colour: "white".to_owned(),
            },
        },
                                     BagRelation {
                                         outer: Bag {
                                             adjective: "light".to_owned(),
                                             colour: "red".to_owned(),
                                         },
                                         quantity: 2,
                                         inner: Bag {
                                             adjective: "muted".to_owned(),
                                             colour: "yellow".to_owned(),
                                         },
                                     },
        ])
    }

    #[test]
    fn test_parse_input() {
        let fixture = r#"light red bags contain 1 bright white bag, 2 shiny gold bags."#;
        let data_struct = parse_input(&fixture);
        dbg!(data_struct);
    }

    #[test]
    fn test_find_outer_bags() {
        let mut needle = Bag { adjective: "shiny".to_owned(), colour: "gold".to_owned() };
        let fixture = r#"light red bags contain 1 bright white bag, 2 shiny gold bags.
                               shiny crimson bags contain 2 light red bags.
                               mirrored red bags contain 3 shiny crimson bags."#;
        let bag_relations = parse_input(fixture);
        let mut unique_outer_bags: HashSet<String> = HashSet::new();
        let counter = find_outer_bags(&needle, &bag_relations, &mut unique_outer_bags);
        dbg!(counter);
        assert_eq!(counter, 3)
    }

    #[test]
    fn test_sample_pt1() {
        let fixture = r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
                                dark orange bags contain 3 bright white bags, 4 muted yellow bags.
                                bright white bags contain 1 shiny gold bag.
                                muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
                                shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
                                dark olive bags contain 3 faded blue bags, 4 dotted black bags.
                                vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
                                faded blue bags contain no other bags.
                                dotted black bags contain no other bags."#;
        assert_eq!(part1(fixture), 4);
    }

    #[test]
    fn test_sample_pt2_simple() {
        // cut-back, single level hierarchy
        let fixture = r#"shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
                              vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
                              faded blue bags contain no other bags.
                              dotted black bags contain no other bags.
                              dark olive bags contain no other bags."#;
        assert_eq!(part2(fixture), 25);
    }

    #[test]
    fn test_sample_pt2() {
        let fixture = r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
                                dark orange bags contain 3 bright white bags, 4 muted yellow bags.
                                bright white bags contain 1 shiny gold bag.
                                muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
                                shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
                                dark olive bags contain 3 faded blue bags, 4 dotted black bags.
                                vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
                                faded blue bags contain no other bags.
                                dotted black bags contain no other bags."#;
        assert_eq!(part2(fixture), 32);
    }


    #[test]
    fn test_other_sample_pt2() {
        let fixture = r#"shiny gold bags contain 2 dark red bags.
                                dark red bags contain 2 dark orange bags.
                                dark orange bags contain 2 dark yellow bags.
                                dark yellow bags contain 2 dark green bags.
                                dark green bags contain 2 dark blue bags.
                                dark blue bags contain 2 dark violet bags.
                                dark violet bags contain no other bags."#;
        assert_eq!(part2(fixture), 126);
    }

}
/* LEARNINGS:
- calling .fold on ranges
let out = (1..=6).fold(0i32, |r, s| r + i32::pow(2, s));

*/