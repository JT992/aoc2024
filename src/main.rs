#![warn(clippy::pedantic)]
#![warn(clippy::perf)]

use std::fs;

use clap::{Parser, ValueEnum};

mod day03;

const TEST_FILE: &str = "test.txt";
const INPUT_FILE: &str = "input.txt";

#[cfg(debug_assertions)]
const ASSUMED_FILE: &str = TEST_FILE;

#[cfg(not(debug_assertions))]
const ASSUMED_FILE: &str = INPUT_FILE;

#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    /// day to execute
    day: usize,

    #[arg(short, long, default_value_t = Mode::Assume)]
    /// run test, puzzle input, or assume from build type
    mode: Mode,
}

#[derive(Debug, Copy, Clone, ValueEnum)]
enum Mode {
    /// assume from build type (debug=test, release=input)
    Assume,
    /// run test
    Test,
    /// run input
    Input,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values will be skipped")
            .get_name()
            .fmt(f)
    }
}

fn main() {
    let args = Args::parse();
    let day = if args.day < 10 {
        format!("0{}", args.day)
    } else {
        args.day.to_string()
    };
    let file_name = match args.mode {
        Mode::Assume => ASSUMED_FILE,
        Mode::Test => TEST_FILE,
        Mode::Input => INPUT_FILE,
    };
    let file = read_file(&day, file_name);
    let funcs = [day03::main];
    let func = funcs
        .get(args.day - 3)
        .expect("expected function to exist. have you updated the funcs list?");
    println!("{}", func(&file));
}

fn read_file(day: &str, file_name: &str) -> String {
    fs::read_to_string(format!("{day}/{file_name}"))
        .expect("expected the file to exist. have you remembered to download the puzzle input?")
}
