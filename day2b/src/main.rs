use std::{error::Error, io, io::prelude::*};

#[derive(Default, Debug)]
struct MinimumColors {
    red: i32,
    green: i32,
    blue: i32,
}

impl MinimumColors {
    fn set_red_min(&mut self, min: i32) {
        self.red = min;
    }

    fn set_green_min(&mut self, min: i32) {
        self.green = min;
    }

    fn set_blue_min(&mut self, min: i32) {
        self.blue = min;
    }
}

fn update_mins(name: &str, count: i32, mins: &mut MinimumColors) -> Result<(), String> {
    match name {
        "red" => {
            if count > mins.red {
                mins.set_red_min(count);
            }
            Ok(())
        }
        "green" => {
            if count > mins.green {
                mins.set_green_min(count);
            }
            Ok(())
        }
        "blue" => {
            if count > mins.blue {
                mins.set_blue_min(count);
            }
            Ok(())
        }
        _ => Err(format!("Invalid color {name}")),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut sum: i32 = 0;
    for (line_number, line) in io::stdin().lock().lines().enumerate() {
        let line = line?;
        // Will default to 0 for integer types
        let mut mins: MinimumColors = Default::default();
        let mut split_game = line.splitn(2, ':');
        let game = split_game
            .next()
            .ok_or(format!("invalid line: {line_number}"))?
            .chars()
            .filter(char::is_ascii_digit)
            .collect::<String>()
            .parse::<i32>()?;
        let samples = split_game
            .next()
            .ok_or(format!("invalid line: {line_number}"))?
            .split(';');

        for sample in samples {
            for color in sample.split(',') {
                let mut split_number_color = color.split_ascii_whitespace();
                let color_sample_number = split_number_color
                    .next()
                    .ok_or(format!("invalid line: {line_number}"))?
                    .parse::<i32>()?;
                let color_sample_name = split_number_color
                    .next()
                    .ok_or(format!("invalid line: {line_number}"))?;

                // Update the minimums
                update_mins(color_sample_name, color_sample_number, &mut mins)?;
            }
        }
        let power = mins.red * mins.green * mins.blue;
        println!("Line {line_number}, game {game} power {power} mins {mins:?}");
        sum += power;
    }
    println!("{sum}");
    Ok(())
}
