use std::{env, fs, process};

mod days;

fn main() {
    let mut args = env::args().skip(1);
    let day_arg = match args.next() {
        Some(s) => s,
        None => {
            eprintln!("Usage: cargo run -- <day_index>");
            process::exit(1);
        }
    };

    let day: usize = match day_arg.parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Invalid day '{}'. Please provide a non-negative integer.", day_arg);
            process::exit(1);
        }
    };

    // Optional second argument selects part (1 or 2). If omitted, run both.
    let part: Option<u8> = match args.next() {
        Some(p_str) => match p_str.parse::<u8>() {
            Ok(1) => Some(1),
            Ok(2) => Some(2),
            Ok(_) | Err(_) => {
                eprintln!("Invalid part '{}'. Provide 1 or 2.", p_str);
                process::exit(1);
            }
        },
        None => None,
    };

    // Read the corresponding input file (relative to the solutions directory)
    let input_path = format!("../inputs/day{}.txt", day);
    let input = match fs::read_to_string(&input_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to read {}: {}", input_path, e);
            process::exit(1);
        }
    };

    // Dispatch to the appropriate day's solution and part(s).
    match (day, part) {
        (0, Some(1)) => days::day0::part1(&input),
        (0, Some(2)) => days::day0::part2(&input),
        (0, None) => days::day0::solve(&input),
        (1, Some(1)) => days::day1::part1(&input),
        (1, Some(2)) => days::day1::part2(&input),
        (1, None) => days::day1::solve(&input),
        (2, Some(1)) => days::day2::part1(&input),
        (2, Some(2)) => days::day2::part2(&input),
        (2, None) => days::day2::solve(&input),
        (3, Some(1)) => days::day3::part1(&input),
        (3, Some(2)) => days::day3::part2(&input),
        (3, None) => days::day3::solve(&input),
        (4, Some(1)) => days::day4::part1(&input),
        (4, Some(2)) => days::day4::part2(&input),
        (4, None) => days::day4::solve(&input),
        (5, Some(1)) => days::day5::part1(&input),
        (5, Some(2)) => days::day5::part2(&input),
        (5, None) => days::day5::solve(&input),
        (6, Some(1)) => days::day6::part1(&input),
        (6, Some(2)) => days::day6::part2(&input),
        (6, None) => days::day6::solve(&input),
        _ => {
            eprintln!("No solution implemented for day {}", day);
            process::exit(1);
        }
    }
}