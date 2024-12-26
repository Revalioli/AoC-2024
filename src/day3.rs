use crate::*;
use regex::Regex;
use aoc_2024::FromRegex;

run_day!{day3}


pub fn part1(input: &str) -> u64 {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    Vec::<(u64, u64)>::from_regex(&regex, input).unwrap()
        .iter().map(|t| t.0*t.1)
        .sum()
}

pub fn part2(input: &str) -> u64 {
    let regex_mul = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let regex_instruction = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)").unwrap();

    let mut enabled = true;
    let mut count = 0;

    for m in regex_instruction.find_iter(input) {
        match m.as_str() {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            s if enabled => {
                // Got a mul instruction
                let (_, [left, right]) = regex_mul.captures(s).unwrap().extract();
                count += left.parse::<u64>().unwrap() * right.parse::<u64>().unwrap();
            },
            _ => ()
        }
    }

    count
}
