use std::process::exit;

use nom::IResult;

pub fn must_parse<F, T>(parser: F, input: &str) -> T
where
    F: Fn(&str) -> IResult<&str, T>,
{
    match parser(input) {
        Ok(("", pairs)) => pairs,
        Ok((remaining, _)) => {
            println!(
                "Invalid puzzle input: could not parse input suffix: {}",
                remaining
            );
            exit(1)
        }
        Err(err) => {
            println!("Could not parse puzzle input: {}", err);
            exit(1)
        }
    }
}
