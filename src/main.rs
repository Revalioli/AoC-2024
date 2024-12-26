use std::{env, io, fs};

/* Common */
#[macro_export]
macro_rules! run_day {
    ( $d:ident ) => {
        pub fn run() -> io::Result<()> {
            println!("===[ {} ]===", std::stringify!($d));
            let file_name = format!( "inputs/input_{}", std::stringify!($d) );
            let input = read_data( &file_name )?;
            println!("Part 1 : {}", part1(&input) );
            println!("Part 2 : {}", part2(&input) );
            Ok(())
        }
    };
}

#[macro_export]
macro_rules! iter2vars {
    ( $i:ident, [$($e:ident),+] ) => {
        $(
            let $e = $i.next().unwrap();
        )+
    }
}

macro_rules! day_run_vec {
    ( $( $d:ident ),* ) => { [ $( $d::run, )* ] };
}

fn read_data(file_name: &str) -> io::Result<String> {
    let input = fs::read_to_string(file_name)?;
    Ok(input.trim_end().to_string())
}

/* ==== Main ==== */

mod day1;
mod day2;
mod day3;


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let run_days =
        day_run_vec![
            day1, day2, day3
        ];

    let args : Vec<String> = env::args().collect();
    match args.get(1) {
        Some(d) => {
            // Run specific day
            let num : usize = d.parse()?;
            run_days[num-1]()?;
        },
        None => {
            // Run all days
            for day in run_days{
                day()?;
            }
        }
    }
    Ok(())
}


/* Test inputs */

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_day {
        (   $d:ident,
            $test_name_1:ident, $test_name_2:ident,
            $input_1:literal, $result_1:literal,
            $input_2:literal, $result_2:literal
        ) => {
            test_day!{$d, part1, $test_name_1, $input_1, $result_1}
            test_day!{$d, part2, $test_name_2, $input_2, $result_2}
        };
        (   $d:ident,
            $test_name_1:ident, $test_name_2:ident,
            $input:literal, $result_1:literal, $result_2:literal
        ) => {
            test_day!{ $d, $test_name_1, $test_name_2, $input, $result_1, $input, $result_2 }
        };
        ( $d:ident, $part:ident, $test_name:ident, $input:literal, $result:literal ) => {
            #[test]
            fn $test_name() { assert_eq!( $d::$part($input), $result); }
        };
    }

    test_day!{ day1, day1_test1, day1_test2,
        "3   4\n\
        4   3\n\
        2   5\n\
        1   3\n\
        3   9\n\
        3   3", 11, 31
    }

    test_day!{ day2, day2_test1, day2_test2,
        "7 6 4 2 1\n\
        1 2 7 8 9\n\
        9 7 6 2 1\n\
        1 3 2 4 5\n\
        8 6 4 4 1\n\
        1 3 6 7 9", 2, 4
    }

    test_day!{ day3, day3_test1, day3_test2,
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))", 161,
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))", 48
    }

}
