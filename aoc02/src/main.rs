use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

fn main() -> Result<(), Box<dyn Error>>
{
    let mut score = 0;

    let file = File::open("aoc02/input.txt")?;
    let lines = io::BufReader::new(file).lines();
    for line in lines
    {
        let line = line?;
        let opponent_choice = Shape(line.chars().next().ok_or("missing opponent choice")? as u8 - 65);
        line.chars.next();
        let my_choice = Shape(line.chars().next().ok_or("missing opponent choice")? as u8 - 65);

    }
    

    Ok(())
}
