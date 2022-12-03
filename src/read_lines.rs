use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file: File = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn get(path: &str) -> Vec<String> {
    let mut values: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(line) = line {
                values.push(line)
            }
        }
    }
    return values;
}
