use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process::exit;

fn parse_file(file_name: &str) -> std::io::Result<(Vec<[u32; 2]>, Vec<[u32; 2]>)> {
    let mut arguments: Vec<[u32; 2]> = Vec::new();
    let mut arguments_checked: Vec<[u32; 2]> = Vec::new();

    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    let mut ignore = false;

    for line in reader.lines() {
        let line = line?;

        let mut i = 0;
        while i < line.len() {
            if line[i..].starts_with("do()") {
                ignore = false
            };
            if line[i..].starts_with("don't()") {
                ignore = true
            };
            if line[i..].starts_with("mul(") {
                if let Some(end) = line[i..].find(")") {
                    let content = &line[i + 4..i + end];
                    if let Some(args) = validate_mul(content) {
                        arguments.push(args);
                        if !ignore {
                            arguments_checked.push(args)
                        }
                        i += end
                    }
                } else {
                    break;
                }
            }
            i += 1;
        }
    }
    Ok((arguments, arguments_checked))
}

fn validate_mul(input: &str) -> Option<[u32; 2]> {
    let parts: Vec<&str> = input.split(",").collect();
    if parts.len() != 2 {
        return None;
    }
    if parts
        .iter()
        .all(|&part| part.chars().all(|c| c.is_ascii_digit()))
    {
        if let (Ok(arg1), Ok(arg2)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
            if arg1 < 1000 && arg2 < 1000 {
                return Some([arg1, arg2]);
            }
        }
    }
    None
}

fn multiply_sum(arguments: &Vec<[u32; 2]>) -> u32 {
    return arguments.iter().map(|a| a[0] * a[1]).sum();
}

pub fn run(args: &[String]) -> io::Result<()> {
    if args.len() < 1 {
        eprintln!("Usage day2 <input-file>");
        exit(1);
    }

    let file_name = args[0].as_str();

    let (arguments, arguments_checked) = parse_file(file_name)?;

    println!("Reports are {:?}, len {}", arguments, arguments.len());

    println!("Sum of multiplications: {}", multiply_sum(&arguments));
    println!(
        "Reports with instructions are {:?}, len {}",
        arguments_checked,
        arguments_checked.len()
    );
    println!(
        "Sum of multiplications: {}",
        multiply_sum(&arguments_checked)
    );
    //println!(
    //    "Valid reports with tolerance: {}",
    //    validate_reports_safe(&reports)
    //);

    Ok(())
}
