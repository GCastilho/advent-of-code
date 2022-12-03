use std::fs;

#[derive(PartialEq, Debug, Clone, Copy)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Hand {
    fn beats(&self) -> Hand {
        match *self {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Victory = 6,
    Failure = 0,
    Draw = 3,
}

#[derive(Debug)]
struct Game {
    opponent: Hand,
    you: Hand,
}

impl Game {
    fn from_string(strategy: &str) -> Option<Game> {
        let hands = strategy
            .split_whitespace()
            .collect::<Vec<&str>>();

        if hands.len() != 2 {
            return None
        }

        let opponent = match hands[0] {
            "A" => Some(Hand::Rock),
            "B" => Some(Hand::Paper),
            "C" => Some(Hand::Scissors),
            _ => None
        };

        let you = match hands[1] {
            "X" => Some(Hand::Rock),
            "Y" => Some(Hand::Paper),
            "Z" => Some(Hand::Scissors),
            _ => None
        };

        match (you, opponent) {
            (Some(you), Some(opponent)) => {
                Some(Game { you, opponent })
            },
            _ => None
        }
    }

    fn play(&self) -> Outcome {
        if self.you.beats() == self.opponent {
            Outcome::Victory
        } else if self.opponent.beats() == self.you {
            Outcome::Failure
        } else {
            Outcome::Draw
        }
    }
}

fn main() {
    let filename = "input.txt";
    // let filename = "example_input.txt";
    let input = fs::read_to_string(filename)
        .unwrap();

    let game_points = input.split_terminator("\n")
        .filter_map(|line| Game::from_string(line))
        .map(|game| {
            let outcome = game.play();
            let result = outcome as u32 + game.you as u32;
            println!("G: {:?} O: {:?}; Result: {}", game, outcome, result);
            result
        })
        .sum::<u32>();

    println!("Points sum: {:?}", game_points);
}
