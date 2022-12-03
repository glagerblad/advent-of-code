use std::collections::HashSet;

fn priority(item: &char) -> i32 {
    match item {
        'a'..='z' => *item as i32 - 96,
        'A'..='Z' => *item as i32 - 38,
        _ => 0,
    }
}

pub fn day_three_one(input: &Vec<String>) -> i32 {
    return input.iter().fold(0, |acc: i32, rucksack: &String| {
        let mid: usize = rucksack.len() / 2;
        let (compartment_one, compartment_two) = rucksack.split_at(mid);
        let set_one: HashSet<char> = compartment_one.chars().into_iter().collect();
        let set_two: HashSet<char> = compartment_two.chars().into_iter().collect();
        return acc
            + set_one
                .intersection(&set_two)
                .into_iter()
                .fold(0, |acc: i32, item: &char| acc + priority(item));
    });
}

pub fn day_three_two(input: &Vec<String>) -> i32 {
    return input.chunks(3).fold(0, |acc: i32, group: &[String]| {
        let sets: Vec<HashSet<char>> = group
            .iter()
            .map(|item: &String| item.chars().into_iter().collect())
            .collect();
        return acc
            + sets
                .iter()
                .skip(1)
                .fold(sets[0].clone(), |acc, hs| {
                    acc.intersection(hs).cloned().collect()
                })
                .iter()
                .fold(0, |acc, item| acc + priority(item));
    });
}
