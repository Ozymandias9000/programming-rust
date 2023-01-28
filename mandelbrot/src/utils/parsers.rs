use std::str::FromStr;

use num::Complex;

pub fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

pub fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        _ => None,
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i64>("", ','), None);
    assert_eq!(parse_pair::<i64>("10,20", '.'), None);
    assert_eq!(parse_pair::<i64>("0,0", ','), Some((0, 0)));
    assert_eq!(parse_pair::<i64>("10.20", '.'), Some((10, 20)));
    assert_eq!(parse_pair::<f32>("10.34,20.1", ','), Some((10.34, 20.1)));
}

#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex("asdf"), None);
    assert_eq!(parse_complex(",34.45"), None);
    assert_eq!(parse_complex("34,23"), Some(Complex { re: 34.0, im: 23.0 }));
    assert_eq!(parse_complex("2,1.5"), Some(Complex { re: 2.0, im: 1.5 }));
}
