use regex::Regex;

pub fn main(file: &str) -> usize {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    re.captures_iter(file)
        .map(|c| {
            let (_, [first, second]) = c.extract();
            Ok(first.parse::<usize>()? * second.parse::<usize>()?)
        })
        .map(|r: Result<usize, std::num::ParseIntError>| r.unwrap_or(0))
        .sum()
}
