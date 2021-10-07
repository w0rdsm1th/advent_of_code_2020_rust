/*
project: advent_of_code_2020_rust
created by: aleks
on: 03 April 2021
*/

use std::env;
use std::fs;
use std::path::Path;


fn current_dir() -> std::io::Result<()> {
    // https://doc.rust-lang.org/std/env/fn.current_dir.html
    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());
    Ok(())
}

pub fn input_reader(day_number: char) -> String {
    /*
    input: day number,
     */
    // println!("current arch {}", env::ARCH);
    let fpath = Path::new("data").join(format!("day{}.txt", day_number));
    let contents = fs::read_to_string(fpath)
        .expect("Something went wrong reading the file");
    println!("read input file, returning");
    contents
}


#[cfg(test)]
mod tests {
    // demonstrating 2 different paths
    use crate::utils::current_dir;
    use super::input_reader;

    #[test]
    fn test_current_dir() {
        current_dir();
    }

    #[test]
    fn test_input_reader() {
        input_reader('1');
    }

}
