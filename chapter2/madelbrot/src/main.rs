use num::Complex;
use std::str::FromStr;
fn main() {
    println!("Hello, world!");
}

/// Try to determine if 'c' is in the Madelbrot set, using at must 'limit'
/// itarations to decide.
///
/// If 'c' is not a member, return 'Some(i)' where 'i' is the number of
/// iterations it took for 'c' to leave the circle of radius 2 centered on the
/// origin. If 'c' seems to be a member (more precisely, if we reached the iteration
/// limit without being able to prove that 'c' is not a member),
/// return 'None'.
fn ecape_time(c: Complex<f64>, limit: u64) -> Option<u64> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c
    }
    None
}

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.trim().find(separator) {
        None => None,
        Some(idx) => match (T::from_str(&s[..idx]), T::from_str(&s[idx + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

fn parse_complex(s: &str) -> Option<Complex<f64>> {
    parse_pair::<f64>(s, ',').map(|(re, im)| Complex { re, im })
    // match parse_pair(s, ',') {
    //     Some((re, im)) => Some(Complex { re, im }),
    //     None => None,
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_pair_empty() {
        assert_eq!(parse_pair::<i32>(",", ','), None);
    }
    #[test]
    fn test_parse_pair_missing_field2() {
        assert_eq!(parse_pair::<i32>("1,", ','), None);
    }
    #[test]
    fn test_parse_pair_missing_field1() {
        assert_eq!(parse_pair::<i32>(",1", ','), None);
    }
    #[test]
    fn test_parse_pair_wrong_field() {
        assert_eq!(parse_pair::<f64>("1adf,1", ','), None);
    }

    #[test]
    fn test_parse_pair_i32() {
        assert_eq!(parse_pair::<i32>("1,1", ','), Some((1, 1)));
    }
    #[test]
    fn test_parse_pair_f64() {
        assert_eq!(parse_pair::<f64>("1,1", ','), Some((1.0, 1.0)));
    }
    #[test]
    fn test_parse_pair_leading_space() {
        assert_eq!(parse_pair::<f64>(" 1,1", ','), Some((1.0, 1.0)));
    }
    #[test]
    fn test_parse_pair_trailing_space() {
        assert_eq!(parse_pair::<f64>("1,1 ", ','), Some((1.0, 1.0)));
    }
    #[test]
    fn test_parse_pair_complex() {
        assert_eq!(
            parse_complex("1.25,-0.0625"),
            Some(Complex {
                re: 1.25,
                im: -0.0625
            })
        )
    }
    #[test]
    fn test_parse_pair_complex_missing_field1() {
        assert_eq!(parse_complex(", -0.0625"), None)
    }
    #[test]
    fn test_parse_pair_complex_missing_field2() {
        assert_eq!(parse_complex("1.0,"), None)
    }
}
