use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process::exit;

fn parse_file(file_name: &str) -> std::io::Result<(Vec<[u32; 2]>, Vec<Vec<u32>>)> {
    let mut orders: Vec<[u32; 2]> = Vec::new();
    let mut reports: Vec<Vec<u32>> = Vec::new();

    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if line.contains("|") {
            // orders
            let order: Vec<u32> = line
                .split("|")
                .map(|n| {
                    n.parse::<u32>()
                        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
                })
                .collect::<Result<_, _>>()?;
            orders.push([order[0], order[1]]);
        }
        if line.contains(",") {
            // reports
            let report: Vec<u32> = line
                .split(",")
                .map(|n| {
                    n.parse::<u32>()
                        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
                })
                .collect::<Result<_, _>>()?;
            reports.push(report);
        }
    }
    Ok((orders, reports))
}

fn correct_middles(orders: &Vec<[u32; 2]>, reports: &Vec<Vec<u32>>) -> (u32, u32) {
    let mut mid_sum: u32 = 0;
    let mut fixed_sum: u32 = 0;
    for report in reports {
        let mut _report = report.to_vec();
        // find order
        let valid_report = report.is_sorted_by(|a, b| _sorted(a, b, orders));
        let mid: usize = report.len() / 2;
        if valid_report {
            mid_sum += report[mid];
        } else {
            _report.sort_by(|a, b| _order(a, b, orders));
            fixed_sum += _report[mid];
        }
    }
    return (mid_sum, fixed_sum);
}

fn _order(a: &u32, b: &u32, orders: &Vec<[u32; 2]>) -> Ordering {
    let order: &[u32; 2] = orders
        .iter()
        .filter(|o| o.contains(a) && o.contains(b))
        .next()
        .expect("Order must be u32|u32");
    if a == &order[0] {
        return Ordering::Greater;
    }
    return Ordering::Less;
}

fn _sorted(a: &u32, b: &u32, orders: &Vec<[u32; 2]>) -> bool {
    return _order(a, b, orders) == Ordering::Greater;
}

pub fn run(args: &[String]) -> io::Result<()> {
    if args.len() < 1 {
        eprintln!("Usage day2 <input-file>");
        exit(1);
    }

    let file_name = args[0].as_str();

    let (orders, reports) = parse_file(file_name)?;

    //println!("Reports are {:?}, orders are {:?} ", reports, orders);
    println!(
        "Middle sums of valid reports: {:?}",
        correct_middles(&orders, &reports)
    );

    Ok(())
}
