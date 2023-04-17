use std::{collections::HashMap};


pub fn process_part1(input: &str) -> String {

    // A X -> Rock -> 1P
    // B Y -> Paper -> 2P
    // C Z - Scissor -> 3P

    // Win -> 8P
    // Loss -> 0P
    // Draw -> 3P

    // Rock -> Scissors -> Paper ->
    let lookup = HashMap::from([
        ("A X", 4),
        ("A Y", 8),
        ("A Z", 3),
        ("B X", 1),
        ("B Y", 5),
        ("B Z", 9),
        ("C X", 7),
        ("C Y", 2),
        ("C Z", 6)
    ]);
    let answer: u32 = input
                .split("\n")
                .map(|round | {
                    lookup[round]
                })
                .sum();
    answer.to_string()
}

pub fn process_part2(input: &str) -> String {

    #[derive(Copy, Clone)]
    enum State {
        Loss = 0,
        Draw = 3,
        Win = 6
    }

    impl State {
        fn from_str(mov: &str) -> Result<Self, ()> {
            if mov == "X"{ Ok(State::Loss)}
            else if mov == "Y" { Ok(State::Draw)}
            else if mov == "Z" { Ok(State::Win)}
            else{ Err(())}
        }
    }

    #[derive(Copy, Clone, Debug)]
    enum Turn {
        Rock = 1,
        Paper = 2,
        Scissors = 3
    }

    impl Turn {
        fn play(turnstr: &str, statestr: &str) -> Result<u32, ()> {
            let state = State::from_str(statestr).expect("Not a valid value");
            let turn = Turn::from_str(turnstr).unwrap();
            match state {
                State::Loss => {
                    match turn {
                    Turn::Rock => Ok(state as u32 + Turn::Scissors as u32),
                    Turn::Paper => Ok(state as u32 + Turn::Rock as u32),
                    Turn::Scissors => Ok(state as u32 + Turn::Paper as u32)
                    }
                }
                State::Draw => {
                    match turn {
                    Turn::Rock => Ok(state as u32 + Turn::Rock as u32),
                    Turn::Paper => Ok(state as u32 + Turn::Paper as u32),
                    Turn::Scissors => Ok(state as u32 + Turn::Scissors as u32)
                    }
                }
                State::Win => {
                    match turn {
                    Turn::Rock => Ok(state as u32 + Turn::Paper as u32),
                    Turn::Paper => Ok(state as u32 + Turn::Scissors as u32),
                    Turn::Scissors => Ok(state as u32 + Turn::Rock as u32)
                    }
                }
            }
        }

        fn from_str(turn: &str) -> Result<Self, ()> {
            if turn == "A" { Ok(Turn::Rock)}
            else if turn == "B" { Ok(Turn::Paper)}
            else if turn == "C" { Ok(Turn::Scissors)}
            else{ Err(())}
        }
    }

    let answer: u32 = input
                        .lines()
                        .map(|turn| {
                            let (opstr, state) = turn.split_once(" ").unwrap();
                            Turn::play(opstr, state).unwrap()
                        })
                        .sum();

    answer.to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, String::from("15"));
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, String::from("12"));
    }
}
