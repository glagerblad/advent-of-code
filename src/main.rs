mod day_2;
mod read_lines;

fn main() {
    // let input = read_lines::get("./inputs/day-1.txt");
    // println!("{}", day_one::day_one(input))
    let input: Vec<String> = read_lines::get("./inputs/day-2.txt");
    println!("{}", day_2::day_two_part_one(&input));
    println!("{}", day_2::day_two_part_two(&input));
}
