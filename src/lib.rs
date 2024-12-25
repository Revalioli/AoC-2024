/// Advent of Code utility functions.
///
/// This crate provides utility functions to parse Advent of Code inputs.

use std::{fmt::Debug, str::FromStr};
use regex::Regex;

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

/// Create a struct instance from the matching result of a regex to a string.
pub trait FromRegex : Sized {

    type Err;

    /// Create a new instance from the match result of a regex on a string.
    ///
    /// This is essentially used with generic Vec of tuples to easily convert
    /// multiple matches and there capture groups to usable tuple instances.
    fn from_regex(r: &Regex, s: &str) -> Result<Self, Self::Err>;

}

/// Type error when parsing a regex capture result to a tuple.
#[derive(Debug)]
pub enum TupleParseRegexError {
    WrongCaptureGroups{ expected: usize, nb_groups: usize },
    TypeParsing(String),
}


macro_rules! vec_tuple_impl_from_regex {
    ( $($T:ident),+ , $count:literal ) => {

        impl< $($T),+ > FromRegex for Vec<( $($T,)+ )>
        where
            $( $T: FromStr<Err: Debug> ),+
        {
            type Err = TupleParseRegexError;

            fn from_regex(r: &Regex, s: &str) -> Result<Self, Self::Err> {
                let mut ret = Vec::new();

                for c in r.captures_iter(s) {
                    if c.len()-1 != $count {
                        return Err(TupleParseRegexError::WrongCaptureGroups{expected: $count, nb_groups: c.len()-1});
                    }

                    let mut captures = c.extract::<$count>().1.into_iter();
                    let to_push = (
                        $(
                            captures.next().unwrap()
                            .parse::<$T>().map_err(|err: $T::Err|
                                TupleParseRegexError::TypeParsing(format!("{err:?}"))
                            )?,
                        )+
                    );

                    ret.push( to_push );
                }

                Ok(ret)
            }
        }

    };
}

vec_tuple_impl_from_regex!{T, 1}
vec_tuple_impl_from_regex!{T, U, 2}
vec_tuple_impl_from_regex!{T, U, V, 3}
vec_tuple_impl_from_regex!{T, U, V, W, 4}
vec_tuple_impl_from_regex!{T, U, V, W, X, 5}
vec_tuple_impl_from_regex!{T, U, V, W, X, Y, 6}
vec_tuple_impl_from_regex!{T, U, V, W, X, Y, Z, 7}


/// Create an instance of `T` for each line based on the result of a regex over a string.
pub fn regex_parse_lines<T: FromRegex>(input: &str, regex: &Regex) -> Result<Vec<T>, T::Err> {
    let mut ret = Vec::new();

    for l in input.lines() {
        ret.push(T::from_regex(&regex, l)?);
    }

    Ok(ret)
}


#[cfg(test)]
#[allow(non_snake_case)]
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


    #[test]
    fn aoc_lib_tuple_2_impl_FromRegex_test() {
        let s = r"5to95.9  mkg jz ¨878fdsljz95.55879 jnlkjhe-778.96to7895to45.2";
        let r = Regex::new( r"(\d+)to(\d+\.\d+)").unwrap();

        let res= Vec::<(u64, f32)>::from_regex(&r, s);
        assert!(res.is_ok(), "{res:?}");
        let res = res.unwrap();
        println!("{res:?}");
        assert_eq!( res, [ (5, 95.9), (7895, 45.2) ] );

        // No match
        let res = Vec::<(u64, f32)>::from_regex(&r, "foobar");
        assert!(res.is_ok());
        assert_eq!(res.unwrap().len(), 0);
    }


    #[test]
    fn aoc_lib_tuple_3_impl_FromRegex_test() {
        let s = r"5to95.9  mkg jz ¨878fdsljz95.55879 jnlkjhe-778.96to7895to45.2,iepgl1556to12.45456foo";
        let r = Regex::new( r"(\d+)to(\d+\.\d+)(.{3})").unwrap();

        let res= Vec::<(u64, f32, String)>::from_regex(&r, s);
        assert!(res.is_ok());
        let res = res.unwrap();
        println!("{res:?}");
        assert_eq!( res, [
            (5, 95.9, String::from("  m")),
            (7895, 45.2, String::from(",ie")),
            (1556, 12.45456, String::from("foo"))
        ]);
    }

    #[test]
    fn aoc_lib_tuple_impl_FromRegex_error_test() {
        let s = r"foo 7889 bar";
        let r_wrong_type = Regex::new(r"(foo)").unwrap();
        let r_wrong_nb_groups = Regex::new(r"(\d*) (ba)r").unwrap();

        let res = Vec::<(u32,)>::from_regex(&r_wrong_type, s);
        assert!(res.is_err());
        assert!(matches!(res.unwrap_err(), TupleParseRegexError::TypeParsing(_)));

        let res = Vec::<(u32,)>::from_regex(&r_wrong_nb_groups, s);
        assert!(res.is_err());
        assert!(matches!(res.unwrap_err(), TupleParseRegexError::WrongCaptureGroups{expected: 1, nb_groups: 2}));
    }
}
