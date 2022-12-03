use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::Chars;
extern crate num;
#[macro_use]
extern crate num_derive;

#[derive(FromPrimitive)]
#[derive(PartialEq)]
#[derive(Debug)]
enum Shape
{
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

#[derive(Debug)]
enum Outcome
{
    Loss = 0,
    Draw = 3,
    Win = 6
}

impl Shape {
    fn from_char(ch: char, base: char) -> Option<Shape>
    {
        num::FromPrimitive::from_u32(ch as u32 - base as u32)
    }
    fn from_stream(chars: &mut Chars, base: char) -> Result<Shape, String>
    {
        Shape::from_char(chars.next().ok_or("missing char")?, base).ok_or("invalid choice".to_string())
    }

    fn outcome(&self, opponent: &Shape) -> Outcome
    {
        if *self  == *opponent {
            return Outcome::Draw;
        }
        if *self as i32 == (*opponent as i32 + 1) % 3  {
            return Outcome::Win;
        }
        return Outcome::Loss;
    }
}

fn main() -> Result<(), Box<dyn Error>>
{
    let mut total_score = 0;

    let file = File::open("aoc02/input.txt")?;
    let lines = io::BufReader::new(file).lines();
    for line in lines
    {
        let line = line?;
        let mut chars = line.chars();
        let opponent_choice = Shape::from_stream(&mut chars, 'A')?;
        chars.next(); // skip space
        let my_choice = Shape::from_stream(&mut chars, 'X')?;
        let outcome = my_choice.outcome(&opponent_choice);
        let score = my_choice as u32 + 1 + outcome as u32;
        total_score += score;
        //println!("{line} aka {my_choice:?} vs {opponent_choice:?} = {outcome:?} => {score}")
    }
    println!("Total score is {total_score}");
    

    Ok(())
}
