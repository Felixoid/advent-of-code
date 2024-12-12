use crate::common;
use std::io;
use std::process::exit;

//#[cfg(debug_assertions)]
//use std::{thread, time};

use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Rules;

impl Rules {
    fn process(stone: u64, n: u8, cache: &mut HashMap<(u64, u8), usize>) -> usize {
        if n == 0 {
            return 1;
        }
        let key = (stone, n);
        if let Some(result) = cache.get(&key) {
            return *result;
        }
        let result = if stone == 0 {
            Self::process(1, n - 1, cache)
        } else {
            let exp = stone.ilog10() + 1;
            if exp % 2 == 0 {
                let pow = (10 as u64).pow(exp / 2);
                let first_stone: u64 = stone / pow;
                let second_stone = stone - first_stone * pow;
                return Self::process(first_stone, n - 1, cache)
                    + Self::process(second_stone, n - 1, cache);
            }
            Self::process(stone * 2024, n - 1, cache)
        };
        cache.insert(key, result);
        result
    }
}

// Function to calculate stones after n steps
fn calculate_stones(
    initial_stones: &Vec<u64>, // The initial array of stones
    n: u8,                     // The number of steps left
) -> usize {
    let mut cache: HashMap<(u64, u8), usize> = HashMap::new(); // Cache for storing evolutions
    let mut stone_count: usize = 0;

    // For each initial stone, calculate its evolution and update the stone count
    for stone in initial_stones {
        stone_count += Rules::process(*stone, n, &mut cache);
    }

    stone_count
}

pub fn run(args: &[String]) -> io::Result<()> {
    if args.len() < 2 {
        eprintln!("Usage day2 <input-file> <number-of-blinks>");
        exit(1);
    }

    let file_name = args[0].as_str();
    let n = args[1].parse::<u8>().expect("Second argument must be int");

    let lines: Vec<Vec<u64>> = common::parse_file(file_name)?;
    println!(
        "Here are stones after {} blinks: {}",
        calculate_stones(&lines[0], n),
        n
    );

    Ok(())
}
