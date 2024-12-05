use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process::exit;

fn parse_file(file_name: &str) -> std::io::Result<Vec<Vec<i32>>> {
    let mut reports: Vec<Vec<i32>> = Vec::new();

    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|n| {
                n.parse::<i32>()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
            })
            .collect::<Result<_, _>>()?;
        if nums.len() == 0 {
            eprintln!("Warning: skipping '{}' line", line);
            continue;
        }
        reports.push(nums);
    }
    Ok(reports)
}

fn validate_reports(reports: &Vec<Vec<i32>>) -> u32 {
    let mut valid_reports: u32 = 0;

    for report in reports {
        if find_error(report) < 0 {
            valid_reports += 1
        };
    }
    valid_reports
}

fn find_error(report: &Vec<i32>) -> isize {
    let mut invalid: isize = -1;
    let mut valid: bool = true;
    for n in 0..report.len() - 1 {
        let abs_diff = report[n].abs_diff(report[n + 1]);
        if n < report.len() - 2 {
            valid = (report[n] - report[n + 1]) * (report[n + 1] - report[n + 2]) > 0;
        }
        // > 0 is checked by same direction
        valid = valid && abs_diff < 4;
        if !valid {
            invalid = n as isize;
            break;
        };
    }
    invalid
}

fn validate_reports_safe(reports: &Vec<Vec<i32>>) -> u32 {
    let mut valid_reports: u32 = 0;
    let reports = reports.to_vec();

    for report in reports {
        let error_index = find_error(&report);
        if error_index < 0 {
            valid_reports += 1;
        } else {
            for n in [error_index, error_index + 1, error_index + 2] {
                let n = n as usize;
                let part1 = &report[..n];
                let part2 = &report[n + 1..];
                if find_error(&part1.iter().chain(part2.iter()).copied().collect()) < 0 {
                    valid_reports += 1;
                    break;
                }
            }
        };
    }
    valid_reports
}

pub fn run(args: &[String]) -> io::Result<()> {
    if args.len() < 1 {
        eprintln!("Usage day2 <input-file>");
        exit(1);
    }

    let file_name = args[0].as_str();

    let reports = parse_file(file_name)?;

    //println!("Reports are {:?}", reports);

    println!("Valid reports: {}", validate_reports(&reports));
    println!(
        "Valid reports with tolerance: {}",
        validate_reports_safe(&reports)
    );

    Ok(())
}