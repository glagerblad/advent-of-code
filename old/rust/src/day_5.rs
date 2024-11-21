pub fn day_five_one(input: &Vec<String>) -> String {
    return input[10..]
        .into_iter()
        .fold(
            vec![
                vec!["V", "C", "D", "R", "Z", "G", "B", "W"],
                vec!["G", "W", "F", "C", "B", "S", "T", "V"],
                vec!["C", "B", "S", "N", "W"],
                vec!["Q", "G", "M", "N", "J", "V", "C", "P"],
                vec!["T", "S", "L", "F", "D", "H", "B"],
                vec!["J", "V", "T", "W", "M", "N"],
                vec!["P", "F", "L", "C", "S", "T", "G"],
                vec!["B", "D", "Z"],
                vec!["M", "N", "Z", "W"],
            ],
            |mut acc: Vec<Vec<&str>>, line: &String| {
                let split_str: Vec<&str> = line.split(" ").collect();
                let amount = split_str[1].parse::<usize>().unwrap();
                let from = split_str[3].parse::<usize>().unwrap() - 1;
                let to = split_str[5].parse::<usize>().unwrap() - 1;
                let new_length = acc[from].len().saturating_sub(amount);
                let mut tail: Vec<&str> = acc[from].split_off(new_length);
                tail.reverse();
                acc[to].append(&mut tail.to_owned());
                return acc;
            },
        )
        .iter()
        .fold("".to_string(), |acc, stack| acc + stack.last().unwrap());
}

pub fn day_five_two(input: &Vec<String>) -> String {
    return input[10..]
        .into_iter()
        .fold(
            vec![
                vec!["V", "C", "D", "R", "Z", "G", "B", "W"],
                vec!["G", "W", "F", "C", "B", "S", "T", "V"],
                vec!["C", "B", "S", "N", "W"],
                vec!["Q", "G", "M", "N", "J", "V", "C", "P"],
                vec!["T", "S", "L", "F", "D", "H", "B"],
                vec!["J", "V", "T", "W", "M", "N"],
                vec!["P", "F", "L", "C", "S", "T", "G"],
                vec!["B", "D", "Z"],
                vec!["M", "N", "Z", "W"],
            ],
            |mut acc: Vec<Vec<&str>>, line: &String| {
                let split_str: Vec<&str> = line.split(" ").collect();
                let amount = split_str[1].parse::<usize>().unwrap();
                let from = split_str[3].parse::<usize>().unwrap() - 1;
                let to = split_str[5].parse::<usize>().unwrap() - 1;
                let new_length = acc[from].len().saturating_sub(amount);
                let tail: Vec<&str> = acc[from].split_off(new_length);
                acc[to].append(&mut tail.to_owned());
                return acc;
            },
        )
        .iter()
        .fold("".to_string(), |acc, stack| acc + stack.last().unwrap());
}
