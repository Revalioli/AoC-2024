use std::iter::zip;
use std::collections::HashMap;

use crate::*;

run_day!{day1}

pub fn parsing_columns(input: &str) -> (Vec<i64>, Vec<i64>) {
    input.lines().map( |l| {
        let mut it = l.split_whitespace()
                                                 .map(|i| i.parse::<i64>().unwrap());
        let t1 = it.next();
        let t2 = it.next();

        // (it.next().unwrap(), it.next().unwrap())
        (t1.unwrap(), t2.unwrap())
    }).unzip()
}

pub fn part1(input: &str) -> u64{
    let (mut left, mut right) = parsing_columns(input);

    left.sort();
    right.sort();

    zip(left, right).map(|i| (i.0 - i.1).abs() as u64).sum()
}

pub fn part2(input: &str) -> u64{
    let (left, right) = parsing_columns(input);

    let count = left.len();
    let mut counters: HashMap<i64, u64> = zip(left.clone(), vec![0; count]).collect();

    for i in right {
        if let Some(c) = counters.get_mut(&i) {
            *c += 1
        }
    }

    left.into_iter().map(|i| {
        match counters.get(&i) {
            Some(&c) => c*(i as u64),
            None => 0
        }
    }).sum()

}
