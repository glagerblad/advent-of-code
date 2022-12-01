use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn values() -> Vec<String> {
    let mut values = Vec::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(line) = line {
                values.push(line)
            }
        }
    }
    return values;
}

// O(N)
fn first(inputs: &Vec<String>) -> i32 {
    let mut first: i32 = 0;
    let mut second: i32 = 0;
    let mut third: i32 = 0;
    let mut elf_snacks: i32 = 0;

    for item_str in inputs {
        if item_str == "" {
            if elf_snacks >= first {
                third = second;
                second = first;
                first = elf_snacks;
            } else if elf_snacks >= second {
                third = second;
                second = elf_snacks;
            } else if elf_snacks > third {
                third = elf_snacks;
            }
            elf_snacks = 0
        } else {
            let item: i32 = item_str.parse::<i32>().unwrap_or_default();
            elf_snacks += item
        }
    }

    println!("#1  -> {}", first);
    println!("#2  -> {}", second);
    println!("#3  -> {}", third);
    return first + second + third;
}

fn main() {
    let inputs: Vec<String> = values();
    println!("{}", first(&inputs))
}
