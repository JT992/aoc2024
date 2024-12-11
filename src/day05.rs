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

fn ordering_ord(ordering: &Ordering, a: u8, b: u8) -> std::cmp::Ordering {
    // yes. I did try to make `Ordering` a struct and make this a method of it.
    // but it doesn't like you can treat methods as functions with addresses?
    // this is more fun, anyway.
    bool::cmp(
        &ordering.get(&b).is_some_and(|v| v.contains(&a)),
        &ordering.get(&a).is_some_and(|v| v.contains(&b)),
    )
}

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

fn create_ordering(file: &str) -> Ordering {
    let mut ordering = Ordering::default();
    RE_ORDERING_RULE
        .captures_iter(file)
        .map(|c| read_ordering_rule(&c))
        .map(|(e, l)| add_to_ordering(e, l, &mut ordering))
        .last();
    ordering
}

fn collect_instructions(file: &str) -> Vec<Vec<u8>> {
    RE_PRINT_INSTRUCTION
        .find_iter(file)
        .map(|m| {
            m.as_str()
                .split(',')
                .map(|v| v.parse::<u8>().unwrap())
                .collect()
        })
        .collect()
}

pub fn part1(file: &str) -> usize {
    let ordering = create_ordering(file);
    collect_instructions(file)
        .iter()
        .filter(|i| check_instruction_validity(i, &ordering))
        .map(|i| i[i.len() / 2])
        .map(usize::from)
        .sum()
}

pub fn part2(file: &str) -> usize {
    let ordering = create_ordering(file);
    let mut all_instructions = collect_instructions(file);
    let invalid_instructions = all_instructions
        .iter_mut()
        .filter(|i| !check_instruction_validity(i, &ordering));
    let mut sum = 0;
    for i in invalid_instructions {
        i.sort_unstable_by(|a, b| ordering_ord(&ordering, *a, *b));
        sum += usize::from(i[i.len() / 2]);
    }
    sum
}
