use std::cmp;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process::exit;
use std::result;

fn parse_file(file_name: &str) -> std::io::Result<(Vec<u32>, Vec<u32>)> {
    let mut id1 = Vec::new();
    let mut id2 = Vec::new();

    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let nums: Vec<u32> = line
            .split_whitespace()
            .map(|n| {
                n.parse::<u32>()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
            })
            .collect::<Result<_, _>>()?;
        if nums.len() != 2 {
            eprintln!("Warning: skipping '{}' line", line);
            continue;
        }
        id1.push(nums[0]);
        id2.push(nums[1]);
    }
    Ok((id1, id2))
}

fn count_distant(id1: &Vec<u32>, id2: &Vec<u32>) -> result::Result<u64, &'static str> {
    let id1: Vec<u32> = id1.to_vec();
    let id2: Vec<u32> = id2.to_vec();

    assert!(id1.len() == id2.len());
    let mut distant: u64 = 0;
    let mut i = 0;
    while i < id1.len() {
        distant += u64::from(cmp::max(id1[i], id2[i]) - cmp::min(id1[i], id2[i]));
        i += 1;
    }
    Ok(distant)
}

fn count_similarity(id1: &Vec<u32>, id2: &Vec<u32>) -> result::Result<usize, &'static str> {
    assert!(id1.len() == id2.len());
    let similarity = id1
        .iter()
        .map(|&n| id2.iter().filter(|&m| m == &n).count() * n as usize)
        .sum();
    Ok(similarity)
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage {} <file_name>", args[0]);
        exit(1);
    }

    let file_name = &args[1];

    let (id1, id2) = parse_file(file_name)?;

    let distant =
        count_distant(&id1, &id2).map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

    println!("Distant between the IDs is {}", distant);

    let similarity =
        count_similarity(&id1, &id2).map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

    println!("Similarity between the IDs is {}", similarity);

    Ok(())
}
