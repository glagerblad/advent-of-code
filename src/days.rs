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

#[derive(Copy, Clone, Debug)]
enum OutComes {
    WIN = 6,
    DRAW = 3,
    LOSS = 0,
}

#[derive(Copy, Clone, Debug)]
enum Hand {
    NONE,
    ROCK = 1,
    PAPER = 2,
    SCISSOR = 3,
}

pub fn day_two_part_one(input: Vec<String>) -> i32 {
    let conversion = |played: char| match played {
        'A' => Hand::ROCK,
        'B' => Hand::PAPER,
        'C' => Hand::SCISSOR,
        'X' => Hand::ROCK,
        'Y' => Hand::PAPER,
        'Z' => Hand::SCISSOR,
        _ => Hand::NONE,
    };
    let who_won = |game| match game {
        (Hand::ROCK, Hand::ROCK) => OutComes::DRAW,
        (Hand::ROCK, Hand::PAPER) => OutComes::WIN,
        (Hand::ROCK, Hand::SCISSOR) => OutComes::LOSS,
        (Hand::PAPER, Hand::ROCK) => OutComes::LOSS,
        (Hand::PAPER, Hand::PAPER) => OutComes::DRAW,
        (Hand::PAPER, Hand::SCISSOR) => OutComes::WIN,
        (Hand::SCISSOR, Hand::ROCK) => OutComes::WIN,
        (Hand::SCISSOR, Hand::PAPER) => OutComes::LOSS,
        (Hand::SCISSOR, Hand::SCISSOR) => OutComes::DRAW,
        (_, _) => OutComes::DRAW,
    };
    let mut count: i32 = 0;
    for line in input {
        let chars: Vec<char> = line.chars().collect();
        let p1: Hand = conversion(chars[0]);
        let p2: Hand = conversion(chars[2]);
        let chars: Vec<char> = line.chars().collect();
        let game: OutComes = who_won((p1, p2));
        let score: i32 = game as i32 + conversion(chars[2]) as i32;
        count += score
    }
    return count;
}

pub fn day_two_part_two(input: Vec<String>) -> i32 {
    let conversion_hand = |played: char| match played {
        'A' => Hand::ROCK,
        'B' => Hand::PAPER,
        'C' => Hand::SCISSOR,
        _ => Hand::NONE,
    };
    let conversion_out_come = |played: char| match played {
        'X' => OutComes::LOSS,
        'Y' => OutComes::DRAW,
        'Z' => OutComes::WIN,
        _ => OutComes::DRAW,
    };
    let what_to_play = |game| match game {
        (Hand::ROCK, OutComes::WIN) => Hand::PAPER,
        (Hand::ROCK, OutComes::LOSS) => Hand::SCISSOR,
        (Hand::ROCK, OutComes::DRAW) => Hand::ROCK,
        (Hand::PAPER, OutComes::WIN) => Hand::SCISSOR,
        (Hand::PAPER, OutComes::LOSS) => Hand::ROCK,
        (Hand::PAPER, OutComes::DRAW) => Hand::PAPER,
        (Hand::SCISSOR, OutComes::WIN) => Hand::ROCK,
        (Hand::SCISSOR, OutComes::LOSS) => Hand::PAPER,
        (Hand::SCISSOR, OutComes::DRAW) => Hand::SCISSOR,
        (_, _) => Hand::NONE,
    };
    let mut count: i32 = 0;
    for line in input {
        let chars: Vec<char> = line.chars().collect();
        let p1: Hand = conversion_hand(chars[0]);
        let game_outcome: OutComes = conversion_out_come(chars[2]);
        let hand_to_play: Hand = what_to_play((p1, game_outcome));
        let score: i32 = hand_to_play as i32 + game_outcome as i32;
        println!(
            "Hand to play: {:?}, Game Outcome: {:?} => {:?}",
            hand_to_play, game_outcome, score
        );
        count += score
    }
    return count;
}
