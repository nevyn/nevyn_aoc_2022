use std::{collections::HashSet, error::Error};
use std::fs::File;
use std::io::{self, BufRead};
use std::str::Chars;

fn char_to_item_priority(c: char) -> Option<i32>
{
    if c >= 'a' && c <= 'z'
    {
        return Some(c as i32 - 'a' as i32);
    }
    else if c >= 'A' && c <= 'Z'
    {
        return Some(27 + c as i32 - 'A' as i32);
    }
    None
}

fn main() -> Result<(), Box<dyn Error>>
{
    let file = File::open("aoc03/input.txt")?;
    let lines = io::BufReader::new(file).lines();
    for line in lines
    {
        let line = line?;
        let chars = line.chars();
        let mut lefts = HashSet::new();
        let mid = line.len()/2;
        for char in chars.copied().take(mid)
        {
            lefts.insert(char_to_item_priority(char).ok_or("invalid char")?);
        }

        for char in chars.skip(mid)
        {

        }
    }
    Ok(())
}
