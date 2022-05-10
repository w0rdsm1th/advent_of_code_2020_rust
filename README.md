## Advent of Code 2020 
Aleks's solutions to [Advent of Code 2020](https://adventofcode.com/2020), written in rust


objectives
- [x] learning Rust syntax
- [x] learning a compiled language
- [x] familiar with rust docs
- [x] familiar with cargo and rustc's error messages
- [x] modules


Notes:

Day 1:
- a

Day 2:
- typing optional char function output
- getting the first char from a String
`.chars().next().unwrap()`

Day 3:
- multi-line string syntax: `r#"..."#`
- quite happy about the conditional `exceed_width` 0/1 multiplier flag for optionally resetting the x-coord width
- otherwise just quite 'loopy'


Day 4:
- `use std::collections::HashMap;`
- matching String against str using .to_string() 
- Regex and lazy_static avoids re-compilation every time
- debug statements `dbg!(&height);`: equivalent to more verbose `println!("{:?}")`


Day 5:
https://stackoverflow.com/a/40310140/3596968
- filtering `vec![]`: `xs.retain(|&x| x != some_x);`

Day 6:
- ran into issues with vectors of arrays of different sizes


Day 7:
- regex:
  - pair of great patterns:
  
    `static ref outer_bag_regex: Regex = Regex::new(r"(?P<adjective>\w[a-z]+) (?P<colour>\w[a-z]+) bags contain").unwrap();`
  
    `static ref inner_bag_regex: Regex = Regex::new(r"(?P<qty>\d) (?P<adjective>\w[a-z]+) (?P<colour>\w[a-z]+) bag").unwrap();`
  - lazy static eval to prevent recompilation every loop
  - CaptureMatches (contains meta-data abt location of match) and just Matches
- super concise sums over ranges
  `(1..64).fold(0u64, |r, s| r + square(s + 1))`


Day 8:
- 