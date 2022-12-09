mod day_7;
mod read_lines;

fn main() {
    // DAY 7 USED HELP
    let input: Vec<String> = read_lines::get("../inputs/day_7.txt");
    println!("Solution 1 -> {}", day_7::run_a(&input));
    println!("Solution 2 -> {}", day_7::run_b(&input));
}
