use std::{fs, str::FromStr};

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    const GAME_SET_TO_CHECK: GameSet = GameSet {
        red: 12,
        green: 13,
        blue: 14,
    };

    let games = input
        .lines()
        .map(|line| line.parse::<Game>().unwrap())
        .collect::<Vec<_>>();
    let possible_games_id_sum = games
        .iter()
        .filter(|game| game.check(&GAME_SET_TO_CHECK) == GameResult::Possible)
        .fold(0, |acc, game| acc + game.id);
    println!("Possible games part one: {possible_games_id_sum}");

    let power_sum = games.iter().map(|game| game.minimum().power()).sum::<u64>();
    println!("Sum of powers is: {power_sum}");
}

#[derive(Debug, PartialEq, Default)]
struct GameSet {
    red: u8,
    green: u8,
    blue: u8,
}

impl GameSet {
    fn power(&self) -> u64 {
        self.red as u64 * self.green as u64 * self.blue as u64
    }
}

impl FromStr for GameSet {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        #[derive(Debug)]
        enum Cubes {
            Red,
            Green,
            Blue,
        }

        let mut game_set = GameSet::default();

        s.split(',')
            .map(|s| {
                let mut words = s.split_whitespace();
                let num = words.next().unwrap().parse::<u8>().unwrap();
                let cube = match words.next().unwrap() {
                    "red" => Cubes::Red,
                    "green" => Cubes::Green,
                    "blue" => Cubes::Blue,
                    _ => panic!("Missing cube type"),
                };
                (num, cube)
            })
            .for_each(|(num, cube)| match cube {
                Cubes::Red => game_set.red = num,
                Cubes::Green => game_set.green = num,
                Cubes::Blue => game_set.blue = num,
            });

        Ok(game_set)
    }
}

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    sets: Vec<GameSet>,
}

impl Game {
    fn check(&self, to: &GameSet) -> GameResult {
        if self
            .sets
            .iter()
            .any(|set| set.red > to.red || set.blue > to.blue || set.green > to.green)
        {
            GameResult::Impossible
        } else {
            GameResult::Possible
        }
    }

    fn minimum(&self) -> GameSet {
        let mut game_set = GameSet::default();
        self.sets.iter().for_each(|game| {
            game_set.red = game_set.red.max(game.red);
            game_set.green = game_set.green.max(game.green);
            game_set.blue = game_set.blue.max(game.blue);
        });
        game_set
    }
}

impl FromStr for Game {
    type Err = String;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut line = line.split(':');

        let id = line
            .next()
            .unwrap()
            .split_whitespace()
            .find_map(|str| str.parse::<u32>().ok())
            .unwrap();

        let sets = line
            .next()
            .unwrap()
            .split(';')
            .map(|s| s.trim().parse::<GameSet>().unwrap())
            .collect::<Vec<_>>();

        Ok(Game { id, sets })
    }
}

#[derive(Debug, PartialEq)]
enum GameResult {
    Possible,
    Impossible,
}

#[cfg(test)]
mod test {
    mod part_one {
        use crate::{Game, GameResult, GameSet};

        const GAME_SET_TO_CHECK: GameSet = GameSet {
            red: 12,
            green: 13,
            blue: 14,
        };

        #[test]
        fn parse() {
            let row = "Game 11: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
            let game = row.parse::<Game>().unwrap();
            let expected = Game {
                id: 11,
                sets: vec![
                    GameSet {
                        blue: 3,
                        red: 4,
                        green: 0,
                    },
                    GameSet {
                        red: 1,
                        green: 2,
                        blue: 6,
                    },
                    GameSet {
                        green: 2,
                        blue: 0,
                        red: 0,
                    },
                ],
            };
            assert_eq!(game, expected);
        }

        #[test]
        fn t1() {
            let game = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
                .parse::<Game>()
                .unwrap();
            assert_eq!(game.check(&GAME_SET_TO_CHECK), GameResult::Possible)
        }

        #[test]
        fn t2() {
            let game = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"
                .parse::<Game>()
                .unwrap();
            assert_eq!(game.check(&GAME_SET_TO_CHECK), GameResult::Possible)
        }

        #[test]
        fn t3() {
            let game = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
                .parse::<Game>()
                .unwrap();
            assert_eq!(game.check(&GAME_SET_TO_CHECK), GameResult::Impossible)
        }

        #[test]
        fn t4() {
            let game = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
                .parse::<Game>()
                .unwrap();
            assert_eq!(game.check(&GAME_SET_TO_CHECK), GameResult::Impossible)
        }

        #[test]
        fn t5() {
            let game = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
                .parse::<Game>()
                .unwrap();
            assert_eq!(game.check(&GAME_SET_TO_CHECK), GameResult::Possible)
        }

        #[test]
        fn sum() {
            let test_input = "
                Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
                Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
                Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
                Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
                Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
            ";
            let games = test_input
                .lines()
                .filter(|line| !line.trim().is_empty())
                .map(|line| line.parse::<Game>().unwrap())
                .collect::<Vec<_>>();
            let possible_games_id_sum = games
                .iter()
                .filter(|game| game.check(&GAME_SET_TO_CHECK) == GameResult::Possible)
                .fold(0, |acc, game| acc + game.id);
            assert_eq!(possible_games_id_sum, 8)
        }
    }

    mod part_two {
        use crate::{Game, GameSet};

        #[test]
        fn t1() {
            let game = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
                .parse::<Game>()
                .unwrap();
            assert_eq!(
                game.minimum(),
                GameSet {
                    red: 4,
                    green: 2,
                    blue: 6
                }
            )
        }

        #[test]
        fn t2() {
            let game = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"
                .parse::<Game>()
                .unwrap();
            assert_eq!(
                game.minimum(),
                GameSet {
                    red: 1,
                    green: 3,
                    blue: 4
                }
            )
        }

        #[test]
        fn t3() {
            let game = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
                .parse::<Game>()
                .unwrap();
            assert_eq!(
                game.minimum(),
                GameSet {
                    red: 20,
                    green: 13,
                    blue: 6
                }
            )
        }

        #[test]
        fn t4() {
            let game = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
                .parse::<Game>()
                .unwrap();
            assert_eq!(
                game.minimum(),
                GameSet {
                    red: 14,
                    green: 3,
                    blue: 15
                }
            )
        }

        #[test]
        fn t5() {
            let game = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
                .parse::<Game>()
                .unwrap();
            assert_eq!(
                game.minimum(),
                GameSet {
                    red: 6,
                    green: 3,
                    blue: 2
                }
            )
        }

        #[test]
        fn power() {
            let game = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
                .parse::<Game>()
                .unwrap();
            assert_eq!(game.minimum().power(), 1560)
        }
    }
}
