/// Advent of Code utility functions.
/// 
/// This crate provides utility functions to parse Advent of Code inputs.

use std::str::FromStr;

/// Split a string following the given delimiters, parse each elements as T and return a
/// Vec of the elements.
/// 
/// `delimiters` is handle as in [`str::split()`].
pub fn split_parse<T: FromStr>(line: &str, delimiters: &[char]) -> Result<Vec<T>, T::Err>
{
    let mut res = Vec::new();
    
    for i in line.split(delimiters) {
        res.push(i.parse()?);
    }

    Ok(res)
}

/// Split and parse each line of an input string.
/// 
/// See [`split_parse()`]
pub fn split_parse_lines<T: FromStr>(input: &str, delimiters: &[char])
    -> Result<Vec<Vec<T>>, T::Err>
{
    let mut res = Vec::new();
    for l in input.lines() {
        res.push(split_parse(l, delimiters)?);
    }
    Ok(res)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_lib_split_parse_test() {
        let input = "7 85 -957 2/-99554";
        let wrong_input = "789 : 789 -2451 p";

        let res = split_parse::<i64>(input, &[' ', '/']);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res, [7, 85, -957, 2, -99554]);

        assert!(split_parse::<i64>(wrong_input, &[' ', '/']).is_err());
    }

    #[test]
    fn aoc_lib_split_parse_lines_test() {
        let input = "\
            7.56:-8775=986:-78.5\n\
            78.546:21.45=45.5:7879:77897\n\
        ";
        let wrong_input = "\
            7.56:-8775=p:-78.5\n\
            78.546:qkjflkdsj5:7879\n\
        ";

        let res = split_parse_lines::<f64>(input, &[':', '=']);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res[0], [7.56, -8775f64, 986f64, -78.5]);
        assert_eq!(res[1], [78.546, 21.45, 45.5, 7879f64, 77897f64]);

        assert!(split_parse_lines::<f64>(wrong_input, &[':', '=']).is_err());

    }
}
