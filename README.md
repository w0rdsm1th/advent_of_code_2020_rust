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
- 