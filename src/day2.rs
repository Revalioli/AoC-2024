use crate::*;
use aoc_2024;

run_day!{day2}


fn report_is_safe(r: &[i64], removed_level: Option<usize>) -> bool {
    let mut prev_step = 0;
    let mut prev;
    let init;
    if Some(0) == removed_level {
        prev = r[1];
        init = 2;
    } else {
        prev = r[0];
        if Some(1) == removed_level {
            init = 2;
        } else {
            init = 1;
        }
    };

    for i in init..r.len() {

        if Some(i) == removed_level {
            continue;
        }

        let step = r[i]-prev;
        if step*prev_step < 0 || ! (1..=3).contains(&step.abs()) {
                // Report is not safe
                return match removed_level {
                    Some(_) => false,
                    None => {
                        // Try without element i-1 or element i
                        i > 0 && report_is_safe(r, Some(i-1))
                            || i == 2 && report_is_safe(r, Some(0))
                            || report_is_safe(r, Some(i))
                    }
                };
        } else {
            // Report is still safe
            prev = r[i];
            prev_step = step
        }
    }

    true
}

pub fn part1(input: &str) -> u64{
    let reports: Vec<Vec<i64>> = aoc_2024::split_parse_lines(input, &[' ']).unwrap();
    let mut count = reports.len() as u64;

    for r in &reports {
        if ! report_is_safe(r, Some(usize::MAX)) {
            count -= 1;
        }
    }

    count
}

pub fn part2(input: &str) -> u64{
    let reports: Vec<Vec<i64>> = aoc_2024::split_parse_lines(input, &[' ']).unwrap();
    let mut count = reports.len() as u64;

    for r in &reports {
        if ! report_is_safe(r, None) {
            count -= 1;
        }
    }

    count
}
