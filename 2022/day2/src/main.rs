static INPUT: &str = include_str!("./input.txt");

#[derive(Clone)]
enum Move {
    Rock,
    Paper,
    Scissor,
}

impl Move {
    fn value(&self) -> i32 {
        match self {
            Move::Rock => 0,
            Move::Paper => 1,
            Move::Scissor => 2,
        }
    }
    fn from(n: i32) -> Move {
        match n {
            0 => Move::Rock,
            1 => Move::Paper,
            2 => Move::Scissor,
            _ => panic!("Error creating Move instance"),
        }
    }
}

struct Match {
    my_move: Move,
    opponent_move: Move,
}

fn parse_match1(input: &str) -> Match {
    let mut input = input.split_whitespace();
    let opponent_move = match input.next() {
        Some(value) => Move::from((value.as_bytes()[0] - b'A').into()),
        _ => panic!("Error parsing my move"),
    };
    let my_move = match input.next() {
        Some(value) => Move::from((value.as_bytes()[0] - b'X').into()),
        _ => panic!("Error parsing opponent move"),
    };
    Match {
        my_move,
        opponent_move,
    }
}

fn parse_match2(input: &str) -> Match {
    let mut input = input.split_whitespace();
    let opponent_move = match input.next() {
        Some("A") => Move::Rock,
        Some("B") => Move::Paper,
        Some("C") => Move::Scissor,
        _ => panic!("Error parsing opponent move"),
    };
    let my_move = match input.next() {
        Some("X") => match opponent_move {
            Move::Rock => Move::Scissor,
            Move::Paper => Move::Rock,
            Move::Scissor => Move::Paper,
        },
        Some("Y") => opponent_move.clone(),
        Some("Z") => match opponent_move {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissor,
            Move::Scissor => Move::Rock,
        },
        _ => panic!("Error parsing opponent move"),
    };
    Match {
        my_move,
        opponent_move,
    }
}

fn calculate_score_short(m: &Match) -> i32 {
    let mut score = m.my_move.value() + 1;
    let result = (m.my_move.value() - m.opponent_move.value()).rem_euclid(3);
    if result == 1 {
        score += 6;
    } else if result == 2 {
        score += 0;
    } else {
        score += 3;
    }
    score
}

fn main() {
    let all_matches: Vec<Match> = INPUT.trim().split("\n").map(|x| parse_match1(x)).collect();
    let total_score: i32 = all_matches.iter().map(|m| calculate_score_short(m)).sum();
    println!("Total score with strategy 1: {total_score}");

    let all_matches: Vec<Match> = INPUT.trim().split("\n").map(|x| parse_match2(x)).collect();
    let total_score: i32 = all_matches.iter().map(|m| calculate_score_short(m)).sum();
    println!("Total score with strategy 2: {total_score}");
}
