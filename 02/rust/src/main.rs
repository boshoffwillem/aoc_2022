use std::env;
use std::fs;
use std::fmt;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let data = fs::read_to_string(file_path).unwrap();
    let score = parse_data(data.as_str());
    println!("total score: {}", score);
}

fn parse_data(data: &str) -> u32 {
    let splits = data.split('\n');
    let mut score = 0;
    
    for split in splits {
        println!("split: {}", split);

        if split.is_empty() {
            break;
        }

        let (opponent, me) = parse_round(split);
        score += score_round(opponent, me);
    }

    score
}

fn parse_round(round: &str) -> (Weapon, Weapon) {
    let mut split = round.split(' ');

    let opponent: Weapon;
    let weapon = split.next();
    match weapon {
        Some(value) => {
            opponent = parse_oppononet(value);
        },
        None => opponent = Weapon::Rock
    }

    let me: Weapon;
    let weapon = split.next();
    match weapon {
        Some(value) => {
            me = parse_me(value, &opponent);
        }
        None => me = Weapon::Rock
    }

    (opponent, me)
}

fn parse_oppononet(weapon: &str) -> Weapon {
    match weapon {
        "A" => Weapon::Rock,
        "B" => Weapon::Paper,
        "C" => Weapon::Scissors,
        _ => Weapon::Rock
    }
}

fn parse_me(weapon: &str, opponent: &Weapon) -> Weapon {
    match weapon {
        "X" => {
            match opponent {
                Weapon::Rock => Weapon::Scissors,
                Weapon::Paper => Weapon::Rock,
                Weapon::Scissors => Weapon::Paper,
            }
        },
        "Y" => {
            match opponent {
                Weapon::Rock => Weapon::Rock,
                Weapon::Paper => Weapon::Paper,
                Weapon::Scissors => Weapon::Scissors,
            }
        }, 
        "Z" => {
            match opponent {
                Weapon::Rock => Weapon::Paper,
                Weapon::Paper => Weapon::Scissors,
                Weapon::Scissors => Weapon::Rock,
            }
        },
        _ => Weapon::Rock
    }
}

fn score_round(opponent: Weapon, me: Weapon) -> u32 {
    let mut score = 0;
    println!("opponent: {}, me: {}", opponent, me);

    match opponent {
        Weapon::Rock => {
            match me {
                Weapon::Paper => score += 6,
                Weapon::Rock => score += 3,
                _ => {}
            }
        },
        Weapon::Paper => {
            match me {
                Weapon::Scissors => score += 6,
                Weapon::Paper => score += 3,
                _ => {}
            }
        },
        Weapon::Scissors => {
            match me {
                Weapon::Rock => score += 6,
                Weapon::Scissors => score += 3,
                _ => {}
            }
        }
    }

    match me {
        Weapon::Rock => {
            score += 1;
        },
        Weapon::Paper => {
            score += 2;
        },
        Weapon::Scissors => {
            score += 3
        }
    }

    score
}

enum Weapon {
    Rock,
    Paper,
    Scissors
}

impl fmt::Display for Weapon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Weapon::Rock => write!(f, "Rock"),
            Weapon::Paper => write!(f, "Paper"),
            Weapon::Scissors => write!(f, "Scissors")
        }
    }
}

#[cfg(test)]
mod tests {
    // X = Rock = 1
    // Y = Paper = 2
    // Z = Scissors = 3
    // +6 for a win
    //
    // A = Rock
    // B = Paper
    // C = Scissors

    use crate::{parse_round, score_round, parse_data};

    #[test]
    fn should_choose_rock_if_opponent_chooses_rock() {
        let data = "A Y";
        let (opponent, me) = parse_round(data);
        let points = score_round(opponent, me);
        assert_eq!(points, 4); 
    }

    #[test]
    fn should_choose_rock_if_opponent_chooses_paper() {
        let data = "B X";
        let (opponent, me) = parse_round(data);
        let points = score_round(opponent, me);
        assert_eq!(points, 1); 
    }

    #[test]
    fn should_choose_rock_if_opponent_chooses_scissors() {
        let data = "C Z";
        let (opponent, me) = parse_round(data);
        let points = score_round(opponent, me);
        assert_eq!(points, 7); 
    }

    #[test]
    fn should_calculate_total_score() {
        let data = "A Y\nB X\nC Z";
        let score = parse_data(data);
        assert_eq!(score, 12); 
    }
}
