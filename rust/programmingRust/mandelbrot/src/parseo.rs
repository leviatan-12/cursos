extern crate num;

use std::str::FromStr;
use num::Complex;

fn parse_pareja<T: FromStr>(s: &str, separator: char) -> Option<(T,T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l,r)),
                _ => None
            }
        }
    }
}

fn parse_complejo(s: &str) -> Option<Complex<f64>> {
    match parse_pareja::<f64>(s,',') {
        None => None,
        Some((re, im)) => Some(Complex {re, im})
    }
}

#[test]
fn prueba_parse_pareja() {
    assert_eq!(parse_pareja::<i32>("",','),None);
    assert_eq!(parse_pareja::<i32>("10,",','),None);
    assert_eq!(parse_pareja::<i32>(",10",','),None);
    assert_eq!(parse_pareja::<i32>("10,20",','),Some((10,20)));
    assert_eq!(parse_pareja::<i32>("10,20lol",','),None);
    assert_eq!(parse_pareja::<f32>("0.5x1.5",'x'),Some((0.5,1.5)));
}

#[test]
fn prueba_parse_complejos() {
    assert_eq!(parse_complejo("1.5,"),None);
    assert_eq!(parse_complejo(",0.111234444"),None);
    assert_eq!(parse_complejo("1.204858,0.111234444"),Some(Complex {re: 1.204858, im: 0.111234444}));

}
