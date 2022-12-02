use std::{process::exit, num::ParseIntError};

pub fn part1(input: String) {
  let lines = input.lines();

  let mut current_max_elf_calories: u32 = 0;
  let mut current_elf_calories: u32 = 0;

  for line in lines {
    if line == "" {
      if current_elf_calories > current_max_elf_calories {
        current_max_elf_calories = current_elf_calories;
      }
      current_elf_calories = 0;
    } else {
      let calories = line.parse::<u32>().unwrap_or_else(|err: ParseIntError| -> u32 {
        println!("Could not parse line as u32: {}", err);
        exit(1)
      });
      current_elf_calories += calories;
    }
  }

  println!("{}", current_max_elf_calories)
}

pub fn part2(input: String) {}
