use regex::Regex;
use std::{error::Error, io, io::prelude::*};

fn digit_to_string(digit: &str) -> Result<char, String> {
    match digit {
        "one" => Ok('1'),
        "two" => Ok('2'),
        "three" => Ok('3'),
        "four" => Ok('4'),
        "five" => Ok('5'),
        "six" => Ok('6'),
        "seven" => Ok('7'),
        "eight" => Ok('8'),
        "nine" => Ok('9'),
        _ => Err(format!("Invalid digit {digit}")),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let digit_string_re = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine)")?;
    let mut sum = 0;
    for (line_number, line) in io::stdin().lock().lines().enumerate() {
        let s = line?;
        let first_word_digit = digit_string_re.find(&s);
        let last_word_digit = digit_string_re.find_iter(&s).last();
        let bytes = s.as_bytes();
        let first_digit = s
            .find(char::is_numeric)
            .and_then(|index| match first_word_digit {
                Some(m) => {
                    if m.start() < index {
                        digit_to_string(m.as_str()).ok()
                    } else {
                        std::str::from_utf8(&bytes[index..]).ok()?.chars().next()
                    }
                }
                _ => std::str::from_utf8(&bytes[index..]).ok()?.chars().next(),
            });
        let last_digit = s
            .rfind(char::is_numeric)
            .and_then(|index| match last_word_digit {
                Some(m) => {
                    if m.end() > index {
                        digit_to_string(m.as_str()).ok()
                    } else {
                        std::str::from_utf8(&bytes[index..]).ok()?.chars().next()
                    }
                }
                _ => std::str::from_utf8(&bytes[index..]).ok()?.chars().next(),
            });
        match (first_digit, last_digit) {
            (Some(f), Some(l)) => {
                sum += [f, l].iter().collect::<String>().parse::<i32>()?;
            }
            _ => Err(format!(
                "line number {} contains no valid digits",
                line_number
            ))?,
        }
    }
    println!("{sum}");
    Ok(())
}
