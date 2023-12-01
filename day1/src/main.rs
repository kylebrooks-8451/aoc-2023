use std::{error::Error, io, io::prelude::*};

fn main() -> Result<(), Box<dyn Error>> {
    let mut sum = 0;
    for (line_number, line) in io::stdin().lock().lines().enumerate() {
        let s = line?;
        let bytes = s.as_bytes();
        let first_digit = s
            .find(char::is_numeric)
            .and_then(|index| std::str::from_utf8(&bytes[index..]).ok()?.chars().next());
        let last_digit = s
            .rfind(char::is_numeric)
            .and_then(|index| std::str::from_utf8(&bytes[index..]).ok()?.chars().next());
        match (first_digit, last_digit) {
            (Some(f), Some(l)) => {
                sum += [f, l].iter().collect::<String>().parse::<i32>()?;
            }
            _ => Err(format!("line number {} contains no digits", line_number))?,
        }
    }
    println!("{sum}");
    Ok(())
}
