use crate::utils;
use std::collections::HashMap;
// How many points do you score if you follow the strategy guide?
// Score in a round is shape + result

#[derive(Copy, Clone)]
enum PlayerChoice {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Copy, Clone)]
enum GameResult {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

pub fn part_one() {
    let opponent_choices: HashMap<&str, PlayerChoice> = HashMap::from([
        ("A", PlayerChoice::Rock),
        ("B", PlayerChoice::Paper),
        ("C", PlayerChoice::Scissors),
    ]);
    let my_choices: HashMap<&str, PlayerChoice> = HashMap::from([
        ("X", PlayerChoice::Rock),
        ("Y", PlayerChoice::Paper),
        ("Z", PlayerChoice::Scissors),
    ]);
    let contents = utils::get_input("dec_02");
    let mut lines = contents.lines();

    let mut points: u64 = 0;
    // Iterate over all lines
    while let Some(line) = lines.next() {
        // Get the choice from each line
        let mut moves = line.split(" ");
        let opponent_choice = match moves.next() {
            Some(choice) => opponent_choices.get(choice).unwrap(),
            None => panic!("Something messed up"),
        };
        let my_choice = match moves.next() {
            Some(choice) => my_choices.get(choice).unwrap(),
            None => panic!("Something messed up!"),
        };
        // Determine the points, sum them, and move on
        points = points + u64::from(round_points(opponent_choice, my_choice))
    }
    // Print the total
    println!("Total Points: {points}")
}

fn round_points(opponent_choice: &PlayerChoice, my_choice: &PlayerChoice) -> u8 {
    let result_points = match opponent_choice {
        PlayerChoice::Rock => match my_choice {
            PlayerChoice::Rock => result_to_u8(GameResult::Draw),
            PlayerChoice::Paper => result_to_u8(GameResult::Win),
            PlayerChoice::Scissors => result_to_u8(GameResult::Loss),
        },
        PlayerChoice::Paper => match my_choice {
            PlayerChoice::Rock => result_to_u8(GameResult::Loss),
            PlayerChoice::Paper => result_to_u8(GameResult::Draw),
            PlayerChoice::Scissors => result_to_u8(GameResult::Win),
        },
        PlayerChoice::Scissors => match my_choice {
            PlayerChoice::Rock => result_to_u8(GameResult::Win),
            PlayerChoice::Paper => result_to_u8(GameResult::Loss),
            PlayerChoice::Scissors => result_to_u8(GameResult::Draw),
        },
    };

    return result_points + choice_to_u8(*my_choice);
}

fn result_to_u8(result: GameResult) -> u8 {
    *&result as u8
}

fn choice_to_u8(choice: PlayerChoice) -> u8 {
    *&choice as u8
}

// Make the choice that ends in the X, Y, Z result
pub fn part_two() {
    let opponent_choices: HashMap<&str, PlayerChoice> = HashMap::from([
        ("A", PlayerChoice::Rock),
        ("B", PlayerChoice::Paper),
        ("C", PlayerChoice::Scissors),
    ]);
    let my_goals: HashMap<&str, GameResult> = HashMap::from([
        ("X", GameResult::Loss),
        ("Y", GameResult::Draw),
        ("Z", GameResult::Win),
    ]);
    let contents = utils::get_input("dec_02");
    let mut lines = contents.lines();
    let mut points: u64 = 0;
    // Iterate over all lines
    while let Some(line) = lines.next() {
        // Get the choice from each line
        let mut moves = line.split(" ");
        let opponent_choice = match moves.next() {
            Some(choice) => opponent_choices.get(choice).unwrap(),
            None => panic!("Something messed up"),
        };
        let my_goal = match moves.next() {
            Some(choice) => my_goals.get(choice).unwrap(),
            None => panic!("Something messed up!"),
        };
        // Determine the points, sum them, and move on
        points = points + u64::from(points_for_goal(opponent_choice, my_goal))
    }
    // Print the total
    println!("Part Two Total Points: {points}")
}

fn points_for_goal(opponent_choice: &PlayerChoice, my_goal: &GameResult) -> u8 {
    let my_choice_points = match opponent_choice {
        PlayerChoice::Rock => match my_goal {
            GameResult::Draw => choice_to_u8(PlayerChoice::Rock),
            GameResult::Win => choice_to_u8(PlayerChoice::Paper),
            GameResult::Loss => choice_to_u8(PlayerChoice::Scissors),
        },
        PlayerChoice::Paper => match my_goal {
            GameResult::Draw => choice_to_u8(PlayerChoice::Paper),
            GameResult::Win => choice_to_u8(PlayerChoice::Scissors),
            GameResult::Loss => choice_to_u8(PlayerChoice::Rock),
        },
        PlayerChoice::Scissors => match my_goal {
            GameResult::Draw => choice_to_u8(PlayerChoice::Scissors),
            GameResult::Win => choice_to_u8(PlayerChoice::Rock),
            GameResult::Loss => choice_to_u8(PlayerChoice::Paper),
        },
    };

    return my_choice_points + result_to_u8(*my_goal);
}
