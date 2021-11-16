/*
project: advent_of_code_2020_rust
created by: aleks
on: 07 October 2021
*/
use std::collections::HashSet;

use regex::Regex;


fn share_char(a: &str, b: &str) -> String {
    // https://stackoverflow.com/a/52882539/3596968
    // get which one is shorter
    let (shorter, longer) = if a.len() > b.len() {
        (b, a)
    }  else {
        (a, b)
    };

    // fill the set with the characters from the shorter string
    let shorter_set: HashSet<char> = shorter.chars().collect();
    let longer_set: HashSet<char> = longer.chars().collect();

    shorter_set.intersection(&longer_set).collect()
}


fn split_blank_lines_regex(inp: &str) -> Vec<&str> {
    //
    let re = Regex::new(r"\n\n").unwrap();
    // re.split(inp).map(|x| x.trim().replace("\n", "")).collect()
    vec![]
}

fn pt1_split_blank_lines_iter(inp: &str) -> Vec<String> {
    // not
    let mut out: Vec<String> = vec![];
    let mut tmp_iter = vec![];
    let mut iter = inp.lines().peekable();
    while let Some(line) = iter.next() {
        let trimmed_line = line.trim();
        // dbg!(trimmed_line);
        // dbg!(iter.peek());
        let next_line = iter.peek();
        // todo couldn't get match expression to work nicely
        // match next_line {
        //     None => println!("reached end of string"),
        //     Some(answer) => println!("next line: {}", answer.trim())
        // };
        if !trimmed_line.is_empty() {
            tmp_iter.push(trimmed_line);
        }
        // end of the string
        if next_line.is_none() {
            let joined_iter = tmp_iter.join(",");
            if !joined_iter.is_empty() {
                out.push(joined_iter);
            }
        }

        // next line is a break, add current batch of answers to out
        else if next_line.unwrap().trim().is_empty() {
            let joined_iter = tmp_iter.join(",");
            if !joined_iter.is_empty() {
                out.push(joined_iter);
            }
            tmp_iter = vec![];
        }

        // dbg!(&out);
    }
    out
}


pub fn part1(inp: &str) -> Vec<String> {
    /*For each group, count the number of questions to which anyone answered "yes". What is the sum of those counts?*/
    // split groups on totally blank lines
    // let groups = split_blank_lines_regex(inp);
    let groups = pt1_split_blank_lines_iter(inp);

    // for each group, get unique count of question responses
    let mut sum_unique = 0;
    let mut sets_unique_responses = vec![];
    for group in groups {
        let mut unique_responses = "".to_string();
        for response in group.chars() {
            if response != ',' &&
                !unique_responses.contains(response) {
                unique_responses.push(response);
            }
        }
        sum_unique += unique_responses.len() as i32;
        // dbg!(&counter);
        // dbg!(&unique_responses);
        sets_unique_responses.push(unique_responses);
    }
    dbg!(&sum_unique);
    dbg!(&sets_unique_responses.len());
    sets_unique_responses
}

pub fn part2(inp: &str) -> Vec<String>{
    /*
    got to take the intersection of each group's responses
    if group only == 1 person, then ALL those responses count
    */

    let groups = pt1_split_blank_lines_iter(inp);
    // for each group, taking intersection of responses.
    // intersection of uncertain number of sets
    let mut sum_intersection = 0;
    let mut sets_intersection_responses = vec![];


    for group in groups {
        // consider the embedded ',' which separates out each person in the
        let mut peoples_responses = group.split(',');

        let mut intersection_responses = match peoples_responses.nth(0) {
            None => "",
            Some(i) => i,
        }.to_string();

        for response in peoples_responses {
            // update logic: if intersection is empty
            intersection_responses = share_char(&intersection_responses, response);
            // dbg!(&intersection_responses);
        }
        sum_intersection += intersection_responses.len() as i32;
        sets_intersection_responses.push(intersection_responses);
    }
    dbg!(&sum_intersection);
    dbg!(&sets_intersection_responses.len());
    sets_intersection_responses
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, pt1_split_blank_lines_iter,
                split_blank_lines_regex, share_char};

    #[test]
    fn test_split() {
        //
        dbg!("34,a4".split(','));
        dbg!("".split(','));  // checking how behaves with empty string
    }

    #[test]
    fn test_share_char() {
        assert_eq!(share_char("ab", "c"), "");
        assert_eq!(share_char("ab", "ba"), "ba");
        assert_eq!(share_char("b", "c"), "");
    }

    #[test]
    fn test_split_blank() {
        for (inp, expected) in vec![
            (r#"a

            a
            b
            c

            ab
            ac

            bc

            a
            a"#, ["a", "a,b,c", "ab,ac", "bc", "a,a"]),
        ] {
            assert_eq!(pt1_split_blank_lines_iter(inp), expected)
        }
    }

    #[test]
    fn test_joining() {
        // struggled with Vecs of differently sized Vecs but was actually struggling with Vecs of arrays...
        let mut out = vec![];
        let vec1 = vec![vec![1, 2, 3], vec![1, 4]];
        let vec2 = vec![vec![4, 5, 6]];
        let vec3 = vec![vec![7, 8]];
        out.push(vec1);
        out.push(vec2);
        dbg!(&out);
        out.push(vec3);
        dbg!(&out);
    }

    #[test]
    fn test_sample() {
        let sample = r#"abc

a
b
c

ab
ac

a
a
a
a

b"#;
        // assert_eq!(part1(&sample), 11);
        assert_eq!(part1(&sample), ["abc", "abc", "abc", "a", "b", ]);
    }

    #[test]
    fn test_sample_pt2() {
        let sample = r#"abc

a
b
c

ab
ac

a
a
a
a

b"#;
        assert_eq!(part2(&sample), ["abc", "", "a", "a", "b", ]);
    }
    #[test]
    fn test_sample_pt2_cutback() {
        let sample = r#"a
b
c

ab
ac

a
a
a
a

b"#;
        assert_eq!(part2(&sample), ["", "a", "a", "b", ]);
    }
}


/* LEARNINGS:
-
*/