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
#[derive(Copy, Clone)]
enum Shape
{
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

impl Shape
{
    fn from_char(ch: char, base: char) -> Option<Shape>
    {
        num::FromPrimitive::from_u32(ch as u32 - base as u32)
    }
    fn from_stream(chars: &mut Chars, base: char) -> Result<Shape, String>
    {
        Shape::from_char(chars.next().ok_or("missing char")?, base).ok_or("invalid shape".to_string())
    }

    fn outcome(&self, opponent: &Shape) -> Outcome
    {
        if *self  == *opponent
        {
            return Outcome::Draw;
        }
        if *self as i32 == (*opponent as i32 + 1) % 3 
        {
            return Outcome::Win;
        }
        return Outcome::Loss;
    }
}

#[derive(FromPrimitive)]
#[derive(Debug)]
enum Outcome
{
    Loss = 0,
    Draw = 1,
    Win = 2
}

impl Outcome
{
    fn from_char(ch: char, base: char) -> Option<Outcome>
    {
        num::FromPrimitive::from_u32(ch as u32 - base as u32)
    }
    fn from_stream(chars: &mut Chars, base: char) -> Result<Outcome, String>
    {
        Outcome::from_char(chars.next().ok_or("missing char")?, base).ok_or("invalid outcome".to_string())
    }

    fn choice(&self, opponent: &Shape) -> Shape
    {
        match *self
        {
            Outcome::Draw => *opponent,
            Outcome::Win => num::FromPrimitive::from_i32((*opponent as i32 + 1) % 3).unwrap(),
            Outcome::Loss => num::FromPrimitive::from_i32((*opponent as i32 - 1).rem_euclid(3)).unwrap()
        }
    }

    fn score(&self) -> i32
    {
        match *self
        {
            Outcome::Loss => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

fn part1() -> Result<(), Box<dyn Error>>
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
        let score = my_choice as i32 + 1 + outcome.score();
        total_score += score;
        //println!("{line} aka {my_choice:?} vs {opponent_choice:?} = {outcome:?} => {score}")
    }
    println!("If interpreted as shape+shape, score is {total_score}");
    Ok(())
} 

fn part2() -> Result<(), Box<dyn Error>>
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
        let outcome = Outcome::from_stream(&mut chars, 'X')?;
        let my_choice = outcome.choice(&opponent_choice);
        let score = my_choice as i32 + 1 + outcome.score();
        total_score += score;
        //println!("{line} aka {opponent_choice:?} for {outcome:?} means {my_choice:?}  => {score}")
    }
    println!("If interpreted as shape+outcome, score is {total_score}");
    Ok(())
} 


fn main() -> Result<(), Box<dyn Error>>
{
    part1()?;
    part2()?;

    Ok(())
}
