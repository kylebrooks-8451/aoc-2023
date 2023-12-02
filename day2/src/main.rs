use std::{error::Error, io, io::prelude::*};

fn color_string_to_limit(s: &str) -> Result<i32, String> {
    match s {
        "red" => Ok(12),
        "green" => Ok(13),
        "blue" => Ok(14),
        _ => Err(format!("Invalid color {s}"))
    }
}

fn main() -> Result<(), Box<dyn Error>>  {
    let mut sum: i32 = 0;
    for (line_number, line) in io::stdin().lock().lines().enumerate() {
        let line = line?;
        let mut split_game = line.splitn(2, ':');
        let game = split_game.next().ok_or(format!("invalid line: {line_number}"))?.chars().filter(char::is_ascii_digit).collect::<String>().parse::<i32>()?;
        let samples = split_game.next().ok_or(format!("invalid line: {line_number}"))?.split(';');

        let mut valid_game: bool = true;
        for sample in samples {
            for color in sample.split(',') {
                let mut split_number_color = color.split_ascii_whitespace();
                let color_sample_number = split_number_color.next().ok_or(format!("invalid line: {line_number}"))?.parse::<i32>()?;
                let color_sample_name = split_number_color.next().ok_or(format!("invalid line: {line_number}"))?;
                // Check for an impossible sample and game
                if color_sample_number > color_string_to_limit(color_sample_name)? {
                    valid_game = false;
                }
            }
        }
        // Only sum valid games
        if valid_game {
            sum += game;
        // For debug only
        } else {
            println!("Game number {game} impossible!");
        }        
    }
    println!("{sum}");
    Ok(())
}
