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
fn escape_time(c: Complex<f64>, limit: u64) -> Option<u64> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c
    }
    None
}

/// Parse the string 's' as a coordinate pair, like 600x400 or 1.0,1.25
/// Spceifically 's' should have the form <left><sep><right> where <sep>
/// is the character given by the separator  argument, and 'left' and 'right'
/// are both strings that can be parsed by 'T::from_str', 'separator' must be
/// ASCII character.
///
/// if 's' has the proper form, return 'Some(x,y)'. If it doesn't parse correctly
/// return 'None'.
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.trim().find(separator) {
        None => None,
        Some(idx) => match (T::from_str(&s[..idx]), T::from_str(&s[idx + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

/// Parse pair of floating point numbers separated by a comma as a complex number
fn parse_complex(s: &str) -> Option<Complex<f64>> {
    // Better implementation
    parse_pair::<f64>(s, ',').map(|(re, im)| Complex { re, im })
    // match parse_pair(s, ',') {
    //     Some((re, im)) => Some(Complex { re, im }),
    //     None => None,
    // }
}

/// Given the row and column of a pixel in the output image, return the
/// corresponding point on the complex plane.
/// 'bounds' is a pair giving the width and height of the image in pixels
/// 'pixel' is a (column, row) pair indicating a particular pixel in that image.
/// The 'upper_left' and 'lower_right' parameters are points on the complex plane
/// designating the area our image covers.
fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
    }
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
    #[test]
    fn test_pixel_to_point() {
        assert_eq!(
            pixel_to_point(
                (100, 200),
                (25, 175),
                Complex { re: -1.0, im: 1.0 },
                Complex { re: 1.0, im: -1.0 }
            ),
            Complex {
                re: -0.5,
                im: -0.75
            }
        );
    }
}
