use std::{str::FromStr, time::Instant};

use anyhow::anyhow;


/// The winner of the whole tournament is the player with the highest score.
/// Your total score is the sum of your scores for each round.
/// The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).


fn main() {
    // get input text
    let now = Instant::now();
    let input_txt = include_str!("../input.txt");

    let rounds: Vec<Rounds> = input_txt.split("\n").collect::<Vec<&str>>().iter().map(|&a| Rounds::new(a).unwrap()).collect();

    let total_score = rounds.iter().fold(0, |a, b| a + &b.round_score);

    // for round in &rounds {
    //     println!("Round: {round:?}")
    // }

    println!("total_score = {total_score}");
    println!("ran in {} mc", now.elapsed().as_micros());
}

#[derive(Debug)]
struct Rounds {
    opponent_shape: Shape,
    selected_shape: Shape,
    round_score: i32,
    round_result: RoundResult,
}

impl Rounds {
    pub fn new(round_str: &str) -> anyhow::Result<Self> {
        
        let (opponent, needed_result) = match round_str.split_once(" ") {
            Some((a, x)) => (Shape::from_str(a)?, RoundResult::from_str(x)? ),
            None => return Err(anyhow!("unable to parse string '{round_str}")),
        };

        let selected = Shape::from((&opponent, &needed_result));

        let score = selected.get_score(&needed_result);

        Ok(Self {
            opponent_shape: opponent,
            selected_shape: selected,
            round_score: score,
            round_result: needed_result,
        })
    }
}


#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors
}

impl FromStr for Shape {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Rock),
            "B" => Ok(Self::Paper),
            "C" => Ok(Self::Scissors),
            a => Err(anyhow!("unknown shape identifier '{a}'"))
        }
    }
}

impl ToString for Shape {
    fn to_string(&self) -> String {
        match self {
            Shape::Rock => String::from("Rock"),
            Shape::Paper => String::from("Paper"),
            Shape::Scissors => String::from("Scissors"),
        }
    }
}

impl Into<i32> for &Shape {
    fn into(self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}
impl From<(&Shape, &RoundResult)> for Shape {
    fn from((opponent, expected_result): (&Shape, &RoundResult)) -> Self {
        match (opponent, expected_result) {
            (a, RoundResult::Draw) => a.to_owned(),
            (a, RoundResult::Win) => a.get_win(),
            (a, RoundResult::Loss) => a.get_lose()
        }
    }
}

impl Shape {
    pub fn get_score(&self, round_result: &RoundResult) -> i32 {
        // The score for a single round is the score for the shape you selected 
        // (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the
        // outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).

        
        let result_score = match round_result {
            RoundResult::Win => 6,
            RoundResult::Loss => 0,
            RoundResult::Draw => 3,
        };

        let shape_score: i32 = self.into();

        result_score + shape_score

        
        
    }

    fn get_win(&self) -> Self {
        match self {
            Shape::Rock => Self::Paper,
            Shape::Paper => Self::Scissors,
            Shape::Scissors => Self::Rock,
        }
    }

    fn get_lose(&self) -> Self {
        match self {
            Shape::Rock => Self::Scissors,
            Shape::Paper => Self::Rock,
            Shape::Scissors => Self::Paper,
        }
    }

}


#[derive(Debug)]
enum RoundResult {
    Win,
    Loss,
    Draw
}

impl From<(&Shape, &Shape)> for RoundResult {
    fn from((opponent, selected): (&Shape, &Shape)) -> Self {
        if opponent == selected {
            Self::Draw
        } else {
            match (opponent, selected) {
                (Shape::Paper, Shape::Scissors) | (Shape::Rock, Shape::Paper) | (Shape::Scissors, Shape::Rock) => Self::Win,
             _ => {
                    // println!("{} -- {} assuming loss", a.to_string(), x.to_string());
                    Self::Loss
                }

            }
        }
    }
}

impl FromStr for RoundResult {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Loss),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            a => Err(anyhow!("Unknown or invalid entry '{a}' given"))
        }
    }
}