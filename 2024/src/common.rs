#[cfg(debug_assertions)]
pub fn prnt_lines(lines: &Vec<Vec<char>>) {
    for line in lines {
        let line: String = line.iter().collect();
        println!("{}", line);
    }
}
