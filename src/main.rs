use std::env;
use std::fs;
use std::process;

use year2019::day1::{calc_additional_fuel, calc_fuel, parse_input_day1};
use year2019::day2::{computer_run, computer_run_find, parse_input_day2, restore_state1202};
use year2019::day3::{get_closest_dist, parse_input_day3};

pub mod year2019;

#[derive(Debug)]
struct Config {
    day: String,
    input: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Self, &'static str> {
        args.next();

        let day = match args.next() {
            Some(day) => day,
            None => return Err("Didn't get day: command <day> <path_to_file>"),
        };

        let input = match args.next() {
            Some(input) => match fs::read_to_string(input) {
                Ok(input) => input,
                Err(e) => {
                    eprintln!("{:?}", e);
                    return Err("Input file not found: command <day> <path_to_file>");
                }
            },
            None => return Err("Expect path to input file"),
        };
        Ok(Config { day, input })
    }
}

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problems parsing params: {}", err);
        process::exit(1);
    });

    match config.day.as_str() {
        "day1" => {
            println!("Result is: {}", calc_fuel(parse_input_day1(config.input)));
            process::exit(0);
        }
        "day1_1" => {
            println!(
                "Result is: {}",
                calc_additional_fuel(parse_input_day1(config.input))
            );
            process::exit(0);
        }
        "day2" => {
            println!(
                "Result is: {:?}",
                computer_run(restore_state1202(parse_input_day2(config.input)))
            );
            process::exit(0);
        }
        "day2_1" => {
            println!(
                "Result is: {}",
                computer_run_find(restore_state1202(parse_input_day2(config.input)), 19690720)
            );
            process::exit(0);
        }
        "day3" => {
            println!(
                "Result is: {}",
                get_closest_dist(parse_input_day3(config.input))
            );
            process::exit(0);
        }
        _ => {
            eprintln!("Not implemented or not found for {:?}", config);
            process::exit(1);
        }
    }
}
