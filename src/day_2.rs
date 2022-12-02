#[derive(Copy, Clone, Debug)]
enum OutComes {
    WIN = 6,
    DRAW = 3,
    LOSS = 0,
    NONE = 99,
}

#[derive(Copy, Clone, Debug)]
enum Hand {
    NONE,
    ROCK = 1,
    PAPER = 2,
    SCISSOR = 3,
}

fn conversion_hand(letter: char) -> Hand {
    match letter {
        'A' => Hand::ROCK,
        'B' => Hand::PAPER,
        'C' => Hand::SCISSOR,
        'X' => Hand::ROCK,
        'Y' => Hand::PAPER,
        'Z' => Hand::SCISSOR,
        _ => Hand::NONE,
    }
}

fn who_won(game: (Hand, Hand)) -> OutComes {
    match game {
        (Hand::ROCK, Hand::ROCK) => OutComes::DRAW,
        (Hand::ROCK, Hand::PAPER) => OutComes::WIN,
        (Hand::ROCK, Hand::SCISSOR) => OutComes::LOSS,
        (Hand::PAPER, Hand::ROCK) => OutComes::LOSS,
        (Hand::PAPER, Hand::PAPER) => OutComes::DRAW,
        (Hand::PAPER, Hand::SCISSOR) => OutComes::WIN,
        (Hand::SCISSOR, Hand::ROCK) => OutComes::WIN,
        (Hand::SCISSOR, Hand::PAPER) => OutComes::LOSS,
        (Hand::SCISSOR, Hand::SCISSOR) => OutComes::DRAW,
        (_, _) => OutComes::NONE,
    }
}

pub fn day_two_part_one(input: &Vec<String>) -> i32 {
    let mut count: i32 = 0;
    for line in input {
        let chars: Vec<char> = line.chars().collect();
        let p1: Hand = conversion_hand(chars[0]);
        let p2: Hand = conversion_hand(chars[2]);
        let chars: Vec<char> = line.chars().collect();
        let game: OutComes = who_won((p1, p2));
        let score: i32 = game as i32 + p2 as i32;
        count += score
    }
    return count;
}

fn conversion_out_come(played: char) -> OutComes {
    match played {
        'X' => OutComes::LOSS,
        'Y' => OutComes::DRAW,
        'Z' => OutComes::WIN,
        _ => OutComes::NONE,
    }
}

fn what_to_play(game: (Hand, OutComes)) -> Hand {
    match game {
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
    }
}

pub fn day_two_part_two(input: &Vec<String>) -> i32 {
    let mut count: i32 = 0;
    for line in input {
        let chars: Vec<char> = line.chars().collect();
        let p1: Hand = conversion_hand(chars[0]);
        let game_outcome: OutComes = conversion_out_come(chars[2]);
        let hand_to_play: Hand = what_to_play((p1, game_outcome));
        let score: i32 = hand_to_play as i32 + game_outcome as i32;
        count += score
    }
    return count;
}
