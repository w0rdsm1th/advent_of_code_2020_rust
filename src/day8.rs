/*
project: advent_of_code_2020_rust
created by: aleks
on: 26 April 2022
*/


use std::collections::HashMap;


pub fn part1(inp: &str) -> i32 {
    /*
    Immediately before the program would run an instruction a second time, the value in the accumulator is 5.

    sudo
        executing boot code instructions
        tracking line numbers executed, raising and returning the
    */

    let mut accumulator = 0;
    let mut instructions = HashMap::new();
    for (idx, instruction) in inp.lines().enumerate() {
        instructions.insert(idx as i32, instruction.trim());
    }
    // dbg!(&instructions);

    let mut next_position: i32 = 0;
    loop  {
        let instruction = instructions.remove(&next_position);

        if instruction.is_none() {
            break  // already visited this Op Code instruction before
        }

        let split_instruction: Vec<&str> = match instruction {
            Some(y) => y.split(" ").collect(),
            None => panic!("unexpectedly None Op Code in position {:?}", next_position)
        };

        let argument: i32 = split_instruction[1].parse().unwrap();

        next_position = match split_instruction[0] {
            "acc" => next_position + 1,
            "jmp" => next_position + argument,
            "nop" => next_position + 1,
            _ => panic!("unexpected Op Code={:?}", instruction)
        };
        if split_instruction[0] == "acc" {
            accumulator = accumulator + argument;
        };
    }
    dbg!(&accumulator);
    accumulator

}

fn part2_inner(max_key: i32, instructions: &HashMap<i32, &str>) -> Result<i32, &'static str> {
    let mut accumulator = 0;
    let mut next_position: i32 = 0;
    let mut seen_positions: Vec<i32> = vec![];
    loop {
        if next_position == max_key + 1 {
            println!("SUCCESSFUL TERMINATION");
            break  // successfully
        }
        if seen_positions.contains(&next_position) {
            return Err("INFINITE LOOP DETECTED")
        }

        seen_positions.push(next_position);

        let instruction = instructions.get(&next_position).to_owned();

        let split_instruction: Vec<&str> = match instruction {
            Some(y) => y.split(" ").collect(),
            None => panic!("unexpectedly None Op Code in position {:?}", next_position)
        };

        let argument: i32 = split_instruction[1].parse().unwrap();

        next_position = match split_instruction[0] {
            "acc" => next_position + 1,
            "jmp" => next_position + argument,
            "nop" => next_position + 1,
            _ => panic!("unexpected Op Code={:?}", instruction)
        };
        if split_instruction[0] == "acc" {
            accumulator = accumulator + argument;
        };
    }
    dbg!(&accumulator);
    Ok(accumulator)
}


fn switch_instruction(instruction: &str) -> String {
    // if is instruction is jmp, switch it to nop and vice versa
    let mut new_instruction: String = instruction.to_owned();
    if instruction.starts_with("jmp") {
        new_instruction = instruction.replace("jmp", "nop");
    }
    else if instruction.starts_with("nop") {
        new_instruction = instruction.replace("nop", "jmp");
    }
    new_instruction
}


pub fn part2(inp: &str) -> i32 {
    /*
    normal termination = attempts to execute an instruction immediately after the last instruction in the file.
    so when next_position = max+1

    TODO - ideas
        - can we just set last instruction = nop? >> NO! didn't work
            let max_key = instructions.keys().max().unwrap().to_owned();
            // just flipping the last
            instructions.insert(max_key, "nop +1");
        - else brute-force combinations and try switching all jmp -> nop and nop -> jmp and testing if halts correctly
    */

    let mut instructions = HashMap::new();
    for (idx, instruction) in inp.lines().enumerate() {
        instructions.insert(idx as i32, instruction.trim());
    }
    let max_key = instructions.keys().max().unwrap().to_owned();

    for (key, instruction) in &instructions {
        let mut iter_instructions = instructions.clone();

        // set the new_instruction on some input
        let new_instruction = switch_instruction(instruction);

        iter_instructions.insert(key.to_owned(), &new_instruction);

        // run to see if inf loops
        let check = part2_inner(max_key, &iter_instructions);
        if let Ok(accumulator_out) = check {
            return accumulator_out;
        }
    };
    42  // else catch-all case... should really be an Err()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        let example_instructions = r#"nop +0
                                acc +1
                                jmp +4
                                acc +3
                                jmp -3
                                acc -99
                                acc +1
                                jmp -4
                                acc +6"#;
        assert_eq!(part1(&example_instructions), 5);

    }

    #[test]
    fn test_part2() {
        let example_instructions = r#"nop +0
                                acc +1
                                jmp +4
                                acc +3
                                jmp -3
                                acc -99
                                acc +1
                                jmp -4
                                acc +6"#;
        assert_eq!(part2(&example_instructions), 8);

    }
}
/* LEARNINGS: 
-
*/