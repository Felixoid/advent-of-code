use crate::day4;
use std::io;
use std::process::exit;

#[cfg(debug_assertions)]
use crate::common;
#[cfg(debug_assertions)]
use std::{thread, time};

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

fn checksum(blocks: &Vec<i32>) -> u64 {
    let mut chsum: u64 = 0;
    for i in 0..blocks.len() {
        if blocks[i] == -1 {
            continue;
        }
        chsum += blocks[i] as u64 * i as u64;
    }

    return chsum;
}

fn checksum_rearranged_blocks(blocks: &Vec<i32>) -> u64 {
    let mut blocks = blocks.to_vec();
    let (mut i, mut j): (usize, usize) = (0, blocks.len() - 1);

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

    return checksum(&blocks);
}

fn checksum_defrabmented_blocks(blocks: &Vec<i32>) -> u64 {
    let mut blocks = blocks.to_vec();
    #[cfg(debug_assertions)]
    let mut blocks_chars: Vec<char> = blocks
        .iter()
        .map(|n| {
            if *n < 0 {
                '.'
            } else {
                ((n % 10) as u8 + b'0') as char
            }
        })
        .collect();
    let mut id = blocks[blocks.len() - 1];
    while id >= 0 {
        let mut last_block: usize = usize::MAX;
        for i in (0..blocks.len()).rev() {
            if blocks[i] == id {
                last_block = i;
                break;
            }
        }
        let mut block_len: usize = 0;
        for i in (0..last_block).rev() {
            if blocks[i] == id {
                continue;
            }
            block_len = last_block - i;
            break;
        }
        // first free space.
        let mut first_space: usize = usize::MAX;
        for i in 0..blocks.len() {
            if i > last_block {
                break;
            }
            if blocks[i] != -1 {
                first_space = usize::MAX;
                continue;
            }
            if first_space > i {
                first_space = i
            }
            if i + 1 - first_space == block_len {
                #[cfg(debug_assertions)]
                {
                    // debug
                    //common::prnt_lines(&vec![blocks_chars.to_vec()]);
                }
                // we found a proper free space block
                for i in 0..block_len {
                    (blocks[first_space + i], blocks[last_block - i]) =
                        (blocks[last_block - i], blocks[first_space + i]);
                    #[cfg(debug_assertions)]
                    {
                        (blocks_chars[first_space + i], blocks_chars[last_block - i]) =
                            (blocks_chars[last_block - i], blocks_chars[first_space + i]);
                    }
                }
                #[cfg(debug_assertions)]
                {
                    // debug
                    common::prnt_lines(&vec![blocks_chars.to_vec()]);
                    thread::sleep(time::Duration::from_millis(20));
                }
                break;
            }
        }
        id -= 1;
    }

    return checksum(&blocks);
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

    println!(
        "Checksum for the defragmented disk is {}",
        checksum_defrabmented_blocks(&blocks)
    );

    Ok(())
}
