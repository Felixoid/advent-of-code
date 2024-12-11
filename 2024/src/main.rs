use std::env;
use std::io;
use std::process::exit;

mod common;
mod day1;
mod day10;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage {} dayX [params...]", args[0]);
        exit(1);
    }

    match args[1].as_str() {
        "day1" => day1::run(&args[2..])?,
        "day2" => day2::run(&args[2..])?,
        "day3" => day3::run(&args[2..])?,
        "day4" => day4::run(&args[2..])?,
        "day5" => day5::run(&args[2..])?,
        "day6" => day6::run(&args[2..])?,
        "day7" => day7::run(&args[2..])?,
        "day8" => day8::run(&args[2..])?,
        "day9" => day9::run(&args[2..])?,
        "day10" => day10::run(&args[2..])?,
        _ => {
            eprintln!("Unknown day {}", args[1]);
            exit(1);
        }
    }
    Ok(())
}
