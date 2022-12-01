use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Elf {
    cals: u32,
}

fn main() -> Result<(), Box<dyn Error>>
{
    let mut elves: Vec<Elf> = Vec::new();
    let mut current_elf = Elf { cals: 0 };

    let file = File::open("aoc01/input.txt")?;
    let lines = io::BufReader::new(file).lines();
    for line in lines
    {
        let line = line?;
        match line.parse::<u32>() {
            Ok(cals) => current_elf.cals += cals,
            Err(_) => {
                elves.push(current_elf);
                current_elf = Elf { cals: 0 };
            },
        }
    }
    elves.push(current_elf);

    elves.sort_by_key(|k| k.cals );
    let mut top_three = elves.into_iter().rev().take(3);

    let top = top_three.next().ok_or("too few elves")?;
    println!("Winning elf: {}", top.cals);

    let three_sum :u32 = top_three.map(|elf| elf.cals ).sum();
    println!("Together: {}", three_sum);

    Ok(())
}
