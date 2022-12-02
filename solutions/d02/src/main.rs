use std::fs::read_to_string;

fn main() {
    let input = read_to_string("inputs/d02.txt").unwrap();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

#[derive(PartialEq, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn from_char(c: char) -> Option<Shape> {
        match c {
            'A' | 'X' => Some(Shape::Rock),
            'B' | 'Y' => Some(Shape::Paper),
            'C' | 'Z' => Some(Shape::Scissors),
            _ => None,
        }
    }

    fn choice_score(&self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn versus(&self, opponent: &Shape) -> Outcome {
        if &self.wins_against() == opponent {
            Outcome::Win
        } else if &self.loses_against() == opponent {
            Outcome::Lose
        } else {
            Outcome::Draw
        }
    }

    fn loses_against(&self) -> Shape {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    fn wins_against(&self) -> Shape {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }
}

enum Outcome {
    Win,
    Draw,
    Lose,
}

impl Outcome {
    fn from_char(c: char) -> Option<Outcome> {
        match c {
            'X' => Some(Outcome::Lose),
            'Y' => Some(Outcome::Draw),
            'Z' => Some(Outcome::Win),
            _ => None,
        }
    }

    fn score(&self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }
    }

    fn shape_for_opponent(&self, opponent: &Shape) -> Shape {
        match self {
            Outcome::Win => opponent.loses_against(),
            Outcome::Lose => opponent.wins_against(),
            Outcome::Draw => opponent.clone(),
        }
    }
}

fn part_1(input: &str) -> u32 {
    let mut total_score = 0;
    for game in input.lines() {
        let (opponent_choice, our_choice) = game
            .split_once(' ')
            .map(|(co, c)| {
                (
                    Shape::from_char(co.chars().next().unwrap()).unwrap(),
                    Shape::from_char(c.chars().next().unwrap()).unwrap(),
                )
            })
            .unwrap();
        let our_outcome = our_choice.versus(&opponent_choice);
        total_score += our_choice.choice_score() + our_outcome.score();
    }
    total_score
}

fn part_2(input: &str) -> u32 {
    let mut total_score = 0;
    for game in input.lines() {
        let (opponent_choice, our_outcome) = game
            .split_once(' ')
            .map(|(co, o)| {
                (
                    Shape::from_char(co.chars().next().unwrap()).unwrap(),
                    Outcome::from_char(o.chars().next().unwrap()).unwrap(),
                )
            })
            .unwrap();
        let our_choice = our_outcome.shape_for_opponent(&opponent_choice);
        total_score += our_choice.choice_score() + our_outcome.score();
    }
    total_score
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let test_input = "A Y
B X
C Z";
        assert_eq!(part_1(test_input), 15);
    }

    #[test]
    fn test_part_2() {
        let test_input = "A Y
B X
C Z";
        assert_eq!(part_2(test_input), 12);
    }
}
