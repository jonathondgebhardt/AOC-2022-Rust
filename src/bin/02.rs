#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Throw {
    Rock,
    Paper,
    Scissors,
}

impl Throw {
    pub fn build(c: char) -> Self {
        match c {
            'A' | 'X' => Throw::Rock,
            'B' | 'Y' => Throw::Paper,
            'C' | 'Z' => Throw::Scissors,
            _ => panic!("unsupported type {}", c),
        }
    }
}

pub enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    pub fn build(c: char) -> Self {
        match c {
            'X' => Outcome::Lose,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => panic!("unsupported type {}", c),
        }
    }
}

pub fn build_throws(s: &str) -> (Throw, Throw) {
    let mut iter = s.chars();
    let opponent = iter.next().unwrap();
    iter.next();
    let me = iter.next().unwrap();

    let opponent = Throw::build(opponent);
    let me = Throw::build(me);
    (opponent, me)
}

pub fn get_score(t1: Throw, t2: Throw) -> u32 {
    let winning_score = match get_winner(t1, t2) {
        Outcome::Win => 6,
        Outcome::Draw => 3,
        Outcome::Lose => 0,
    };

    let throw_score = match t2 {
        Throw::Rock => 1,
        Throw::Paper => 2,
        Throw::Scissors => 3,
    };

    winning_score + throw_score
}

pub fn get_winner(t1: Throw, t2: Throw) -> Outcome {
    if t1 == t2 {
        Outcome::Draw
    } else if t1 == Throw::Rock && t2 == Throw::Paper {
        Outcome::Win
    } else if t1 == Throw::Paper && t2 == Throw::Scissors {
        Outcome::Win
    } else if t1 == Throw::Scissors && t2 == Throw::Rock {
        Outcome::Win
    } else {
        Outcome::Lose
    }
}

pub fn throw_from_outcome(other: Throw, o: Outcome) -> Throw {
    match o {
        Outcome::Win => match other {
            Throw::Rock => Throw::Paper,
            Throw::Paper => Throw::Scissors,
            Throw::Scissors => Throw::Rock,
        },
        Outcome::Draw => other,
        Outcome::Lose => match other {
            Throw::Rock => Throw::Scissors,
            Throw::Paper => Throw::Rock,
            Throw::Scissors => Throw::Paper,
        },
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let score = input
        .lines()
        .map(build_throws)
        .fold(0, |acc, (opp, me)| acc + get_score(opp, me));

    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let score = input
        .lines()
        .map(|line| {
            let mut iter = line.chars();
            let opponent = Throw::build(iter.next().unwrap());
            iter.next();
            let me = throw_from_outcome(opponent, Outcome::build(iter.next().unwrap()));
            (opponent, me)
        })
        .fold(0, |acc, (opp, me)| acc + get_score(opp, me));

    Some(score)
}

fn main() {
    let input = &aoc::read_file("inputs", 2);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    mod day2 {
        use super::super::*;

        #[test]
        fn test_part_one() {
            let input = aoc::read_file("examples", 2);
            assert_eq!(part_one(&input), Some(15));
        }

        #[test]
        fn test_part_two() {
            let input = aoc::read_file("examples", 2);
            assert_eq!(part_two(&input), Some(12));
        }
    }
}
