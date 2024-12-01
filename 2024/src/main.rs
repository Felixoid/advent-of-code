use std::env;
use std::io;
use std::process::exit;

mod day1;
mod day2;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage {} dayX [params...]", args[0]);
        exit(1);
    }

    match args[1].as_str() {
        "day1" => day1::run(&args[2..])?,
        "day2" => day2::run(&args[2..])?,
        _ => {
            eprintln!("Unknown day {}", args[1]);
            exit(1);
        }
    }
    Ok(())
}
