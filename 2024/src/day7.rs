use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process::exit;

#[derive(Debug, Clone)]
struct Equation {
    result: u64,
    elements: Vec<u64>,
    operations: Vec<fn(u64, u64) -> u64>,
}

impl Equation {
    fn valid_result(&self, with_concat: bool) -> u64 {
        let mut copy: Equation = self.clone();
        let result = copy.calculate(self.elements[0], 1, with_concat);
        if result != 0 {
            copy.validate();
        }
        return result;
    }

    fn validate(&self) {
        let mut result = self.elements[0];

        let ops: Vec<fn(u64, u64) -> u64> = self.operations.iter().map(|n| *n).rev().collect();
        for i in 0..self.operations.len() {
            result = ops[i](result, self.elements[i + 1])
        }
        let ons: Vec<char> = ops
            .iter()
            .map(|f| {
                if f(1, 1) == 1 {
                    '*'
                } else if f(1, 1) == 2 {
                    '+'
                } else {
                    '|'
                }
            })
            .collect();
        println!(
            "Expected result {}, got result {}, elements {:?}, operations {:?}",
            self.result, result, self.elements, ons
        );
        assert!(result == self.result)
    }

    fn calculate(&mut self, current_result: u64, step: usize, with_concat: bool) -> u64 {
        if step == self.elements.len() {
            return current_result;
        }
        if current_result > self.result {
            return 0;
        }
        for op in Equation::ops(with_concat) {
            let inter_result = op(current_result, self.elements[step]);
            let new_result = self.calculate(inter_result, step + 1, with_concat);
            if new_result == self.result {
                self.operations.push(op);
                return new_result;
            }
        }
        return 0;
    }

    fn ops(with_concat: bool) -> Vec<fn(u64, u64) -> u64> {
        let mut ops = vec![u64::wrapping_add, u64::wrapping_mul];
        if with_concat {
            ops.push(concat)
        }
        return ops;
    }
}

fn concat(lhs: u64, rhs: u64) -> u64 {
    let mut power = 1;
    while rhs / (10 as u64).pow(power) != 0 {
        power += 1;
    }
    return lhs * (10 as u64).pow(power) + rhs;
}

fn parse_file(file_name: &str) -> std::io::Result<Vec<Equation>> {
    let mut equations: Vec<Equation> = Vec::new();

    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if !line.contains(":") {
            continue;
        }

        let mut elements: Vec<u64> = line
            .split(&[':', ' '])
            .filter_map(|n| n.parse::<u64>().ok())
            .collect();
        assert!(elements.len() > 1);
        let result: u64 = elements[0];
        elements = elements[1..].iter().map(|&n| n).collect();
        equations.push(Equation {
            result,
            elements,
            operations: Vec::new(),
        });
    }
    Ok(equations)
}

fn find_results(equations: &Vec<Equation>) -> u64 {
    let mut result = 0;
    for eq in equations {
        result += eq.valid_result(true) as u64;
    }
    return result;
}

pub fn run(args: &[String]) -> io::Result<()> {
    if args.len() < 1 {
        eprintln!("Usage day1 <input-file>");
        exit(1);
    }

    let file_name = args[0].as_str();

    let equations = &parse_file(file_name)?;

    // println!("{:?}", equations);
    println!("Sum of valid results {}", find_results(&equations));
    Ok(())
}
