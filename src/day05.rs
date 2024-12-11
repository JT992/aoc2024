use std::collections::{hash_map::Entry, HashMap};

use lazy_static::lazy_static;
use regex::{Captures, Regex};

lazy_static! {
    static ref RE_ORDERING_RULE: Regex = Regex::new(r"(\d+)\|(\d+)").unwrap();
    static ref RE_PRINT_INSTRUCTION: Regex = Regex::new(r"(?m)^\d+(,\d+)*?,\d+$").unwrap();
}

super::selection!();

// in an Ordering, values that come LATER are stored in the vectors
// for the values that come EARLIER
// (Ordering[earlier] == vec![later])
type Ordering = HashMap<u8, Vec<u8>>;

fn read_ordering_rule(c: &Captures) -> (u8, u8) {
    let (_, [earlier, later]) = c.extract();
    (earlier.parse::<u8>().unwrap(), later.parse::<u8>().unwrap())
}

fn add_to_ordering(earlier: u8, later: u8, ordering: &mut Ordering) {
    match ordering.entry(earlier) {
        Entry::Occupied(mut occupied) => occupied.get_mut().push(later),
        Entry::Vacant(vacant) => {
            vacant.insert(vec![later]);
        }
    }
}

fn check_instruction_validity(instructions: &[u8], ordering: &Ordering) -> bool {
    for (i, later) in instructions[1..].iter().enumerate() {
        for earlier in &instructions[0..=i] {
            if ordering.get(later).is_some_and(|v| v.contains(earlier)) {
                // println!("whoops! {earlier} can't go before {later}!");
                return false;
            }
        }
    }
    true
}

pub fn part1(file: &str) -> usize {
    let mut ordering = Ordering::new();
    RE_ORDERING_RULE
        .captures_iter(file)
        .map(|c| read_ordering_rule(&c))
        .map(|(e, l)| add_to_ordering(e, l, &mut ordering))
        .last();
    RE_PRINT_INSTRUCTION
        .find_iter(file)
        .map(|m| {
            m.as_str()
                .split(',')
                .map(|v| v.parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        })
        .filter(|i| check_instruction_validity(i, &ordering))
        .map(|i| i[i.len() / 2])
        .map(usize::from)
        .sum()
}

pub fn part2(_file: &str) -> usize {
    todo!()
}
