fn get_ranges(line: &str) -> (i32, i32, i32, i32) {
    let one_pair: Vec<Vec<i32>> = line
        .split(',')
        .map(|elf| {
            return elf
                .split('-')
                .map(|str_num| str_num.parse::<i32>().unwrap_or_default())
                .collect();
        })
        .collect();
    return (
        one_pair[0][0],
        one_pair[0][1],
        one_pair[1][0],
        one_pair[1][1],
    );
}

pub fn day_four_one(input: &Vec<String>) -> i32 {
    return input.iter().fold(0, |acc, line| {
        let (x1, x2, y1, y2) = get_ranges(line);
        return (x1 <= y1 && x2 >= y2 || y1 <= x1 && y2 >= x2) as i32 + acc;
    });
}

pub fn day_four_two(input: &Vec<String>) -> i32 {
    return input.iter().fold(0, |acc, line| {
        let (x1, x2, y1, y2) = get_ranges(line);
        return (x1 <= y2 && y1 <= x2) as i32 + acc;
    });
}
