use lazy_static::lazy_static;
use regex::{Captures, Match, Regex};

lazy_static! {
    static ref RE_MUL_INSTRUCTION: Regex = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    static ref RE_MUL_DO_DONT: Regex =
        Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)|do\(\)|don't\(\)").unwrap();
}

super::selection!();

enum Part2Match {
    Mul(usize),
    Do,
    Dont,
}

impl Part2Match {
    pub fn do_or_dont(s: &str) -> Self {
        match s {
            "do()" => Self::Do,
            "don't()" => Self::Dont,
            _ => panic!("expected do() or don't() but got {s}"),
        }
    }
}

pub fn part1(file: &str) -> usize {
    RE_MUL_INSTRUCTION
        .captures_iter(file)
        .map(|c| extract_captures_and_multiply(&c))
        .map(|r| r.unwrap_or(0))
        .sum()
}

pub fn part2(file: &str) -> usize {
    let mut flag = true;
    RE_MUL_DO_DONT
        .find_iter(file)
        .map(|m| distinguish_match(&m))
        .map(|r| r.unwrap_or(Part2Match::Mul(0)))
        .map(|i| match i {
            Part2Match::Mul(n) => {
                if flag {
                    n
                } else {
                    0
                }
            }
            Part2Match::Do => {
                flag = true;
                0
            }
            Part2Match::Dont => {
                flag = false;
                0
            }
        })
        .sum()
}

fn distinguish_match(m: &Match) -> Result<Part2Match, std::num::ParseIntError> {
    let extraction = m.as_str();
    match RE_MUL_INSTRUCTION.captures(extraction) {
        None => Ok(Part2Match::do_or_dont(extraction)),
        Some(m) => Ok(Part2Match::Mul(extract_captures_and_multiply(&m)?)),
    }
}

fn extract_captures_and_multiply(c: &Captures) -> Result<usize, std::num::ParseIntError> {
    let (_, [first, second]) = c.extract();
    Ok(first.parse::<usize>()? * second.parse::<usize>()?)
}
