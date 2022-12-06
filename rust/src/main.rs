mod day_6;
mod read_lines;

fn main() {
    // let input = read_lines::get("./inputs/day-1.txt");
    // println!("{}", day_one::day_one(input))

    // let input: Vec<String> = read_lines::get("./inputs/day-2.txt");
    // println!("{}", day_2::day_two_part_one(&input));
    // println!("{}", day_2::day_two_part_two(&input));

    // let input: &Vec<String> = &read_lines::get("./inputs/day-3.txt");
    // println!("{}", day_3::day_three_two(input));

    // let input: &Vec<String> = &read_lines::get("./inputs/day_4.txt");
    // println!("{}", day_4::day_four_one(input));
    // println!("{}", day_4::day_four_two(input));

    // let input: Vec<String> = read_lines::get("../inputs/day_5.txt");
    // println!("Solution 1 -> {}", day_5::day_five_one(&input));
    // println!("Solution 2 -> {}", day_5::day_five_two(&input));

    let input: Vec<String> = read_lines::get("../inputs/day_6.txt");
    println!("Solution 1 -> {}", day_6::run_a(&input));
    println!("Solution 2 -> {}", day_6::run_b(&input));
}
