use std::collections::HashSet;

fn pos_of_n_uniq_bytes(n: usize, bytes: &[u8]) -> usize {
    return n + bytes
        .windows(n)
        .enumerate()
        .find(|(_, bytes)| bytes.into_iter().collect::<HashSet<&u8>>().len() == n)
        .unwrap_or_default()
        .0;
}

pub fn run_a(input: &Vec<String>) -> usize {
    let input = input.first().unwrap();
    return pos_of_n_uniq_bytes(4, input.as_bytes());
}

pub fn run_b(input: &Vec<String>) -> usize {
    let input = input.first().unwrap();
    return pos_of_n_uniq_bytes(14, input.as_bytes());
}
