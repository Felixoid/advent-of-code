use std::io;
use std::process::exit;

use crate::common::parse_chars;

fn find_xmas(lines: &Vec<Vec<char>>) -> u32 {
    let to_find: &Vec<char> = &"XMAS".chars().collect();
    let mut found: u32 = 0;

    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            let i_s: i32 = i as i32 - 1;
            let j_s: i32 = j as i32 - 1;
            let i_e: i32 = i as i32 + 1;
            let j_e: i32 = j as i32 + 1;
            for l in i_s..=i_e {
                for m in j_s..=j_e {
                    let id: i32 = l - i as i32;
                    let jd: i32 = m - j as i32;
                    found += find_next_xmas(lines, i, j, id, jd, to_find);
                }
            }
        }
    }
    found
}

fn find_next_xmas(
    lines: &Vec<Vec<char>>,
    i: usize,
    j: usize,
    id: i32,
    jd: i32,
    to_find: &Vec<char>,
) -> u32 {
    let search = to_find[0];
    let mut found: u32 = 0;
    let current = lines[i][j];
    if current != search {
        return found;
    }
    if to_find.len() == 1 {
        return 1;
    }
    let l: i32 = i as i32 + id;
    let m: i32 = j as i32 + jd;
    if l < 0 || lines.len() <= l as usize {
        return found;
    }
    if m < 0 || lines[i].len() <= m as usize {
        return found;
    }
    found += find_next_xmas(
        lines,
        l as usize,
        m as usize,
        id,
        jd,
        &to_find[1..].to_vec(),
    );
    return found;
}

fn find_x_mas(lines: &Vec<Vec<char>>) -> u32 {
    let mut found: u32 = 0;
    for i in 1..lines.len() - 1 {
        for j in 1..lines[i].len() - 1 {
            if lines[i][j] == 'A' {
                let mut ms: Vec<[usize; 2]> = Vec::new();
                let mut ss: Vec<[usize; 2]> = Vec::new();
                for l in [i - 1, i + 1] {
                    for m in [j - 1, j + 1] {
                        if lines[l][m] == 'M' {
                            ms.push([l, m]);
                        }
                        if lines[l][m] == 'S' {
                            ss.push([l, m]);
                        }
                    }
                }
                if ms.len() == 2 && ss.len() == 2 {
                    if ms[0][0] == ms[1][0] || ms[0][1] == ms[1][1] {
                        found += 1
                    }
                }
            }
        }
    }
    return found;
}

pub fn run(args: &[String]) -> io::Result<()> {
    if args.len() < 1 {
        eprintln!("Usage day2 <input-file>");
        exit(1);
    }

    let file_name = args[0].as_str();

    let lines = parse_chars(file_name)?;

    //println!("Reports are {:?}, len {}", lines, lines.len());

    println!("Found words XMAS {}", find_xmas(&lines));
    println!("Found words X-MAX {}", find_x_mas(&lines));

    Ok(())
}
