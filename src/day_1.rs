// O(N)
#[warn(dead_code)]
pub fn day_one(input: Vec<String>) -> i32 {
    let mut first: i32 = 0;
    let mut second: i32 = 0;
    let mut third: i32 = 0;
    let mut elf_snacks: i32 = 0;

    for item_str in input {
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
