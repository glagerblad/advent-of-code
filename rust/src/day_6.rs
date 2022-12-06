use std::collections::HashSet;

pub fn run_a(input: &Vec<String>) -> usize {
    let input = input.first().unwrap();
    let xs = input.as_bytes().windows(4).enumerate();
    for (index, bytes) in xs {
        let uniq = bytes.into_iter().collect::<HashSet<_>>();
        if bytes.len() == uniq.len() {
            return index + 4;
        }
    }
    return 0;
}

pub fn run_b(input: &Vec<String>) -> usize {
    let input = input.first().unwrap();
    let xs = input.as_bytes().windows(14).enumerate();
    for (index, bytes) in xs {
        let uniq = bytes.into_iter().collect::<HashSet<_>>();
        if bytes.len() == uniq.len() {
            return index + 14;
        }
    }
    return 0;
}
