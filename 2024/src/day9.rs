use crate::day4;
use std::io;
use std::process::exit;

fn read_blocks(line: &Vec<char>) -> Vec<i32> {
    let mut is_data: bool = true;
    let map: Vec<usize> = line
        .iter()
        .filter_map(|n| n.to_digit(10).map(|n| n as usize))
        .collect();
    let mut blocks: Vec<i32> = Vec::new();
    let mut block_id: i32 = 0;
    for i in map {
        if is_data {
            blocks.extend(vec![block_id; i]);
            block_id += 1;
        } else {
            blocks.extend(vec![-1; i]);
        }
        is_data = !is_data;
    }
    return blocks;
}

fn checksum_rearranged_blocks(blocks: &Vec<i32>) -> u64 {
    let mut blocks = blocks.to_vec();
    let (mut i, mut j): (usize, usize) = (0, blocks.len() - 1);
    let mut chsum: u64 = 0;

    while i < j {
        // find blank i
        if blocks[i] != -1 {
            i += 1;
            continue;
        }
        // find non-blank j
        if blocks[j] == -1 {
            j -= 1;
            continue;
        }
        (blocks[i], blocks[j]) = (blocks[j], blocks[i]);
    }

    for i in 0..blocks.len() {
        if blocks[i] == -1 {
            break;
        }
        chsum += blocks[i] as u64 * i as u64;
    }

    return chsum;
}

pub fn run(args: &[String]) -> io::Result<()> {
    if args.len() < 1 {
        eprintln!("Usage day2 <input-file>");
        exit(1);
    }

    let file_name = args[0].as_str();

    let lines = day4::parse_file(file_name)?;
    let blocks = read_blocks(&lines[0]);
    println!(
        "Checksum for the compacted disk is {}",
        checksum_rearranged_blocks(&blocks)
    );

    Ok(())
}
