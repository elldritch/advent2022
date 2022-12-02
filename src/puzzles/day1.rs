use std::{num::ParseIntError, process::exit};

// This is pretty simple. Keep a running tally of the largest number.
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
            let calories = line
                .parse::<u32>()
                .unwrap_or_else(|err: ParseIntError| -> u32 {
                    println!("Could not parse line as u32: {}", err);
                    exit(1)
                });
            current_elf_calories += calories;
        }
    }

    println!("{}", current_max_elf_calories)
}

// This is also pretty simple. The annoying part is mostly the book-keeping of
// keeping track of three numbers.
//
// Here, we keep a sorted vector of 4 numbers. Whenever we have a new elf, we
// set the first element to the new elf's calories and re-sort. At the end, we
// take the last 3 elements.
//
// I wish this were Haskell, where it would be more idiomatic to just map, sort,
// take, and call it a day.
pub fn part2(input: String) {
    let lines = input.lines();

    let mut top_elf_calories: Vec<u32> = vec![0, 0, 0, 0];
    let mut current_elf_calories: u32 = 0;

    for line in lines {
        if line == "" {
            top_elf_calories[0] = current_elf_calories;
            current_elf_calories = 0;
            top_elf_calories.sort();
        } else {
            let calories = line
                .parse::<u32>()
                .unwrap_or_else(|err: ParseIntError| -> u32 {
                    println!("Could not parse line as u32: {}", err);
                    exit(1)
                });
            current_elf_calories += calories;
        }
    }

    println!(
        "{}",
        top_elf_calories[1] + top_elf_calories[2] + top_elf_calories[3]
    )
}
