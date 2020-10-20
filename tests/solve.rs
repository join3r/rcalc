use rcalc::*;
use rust_decimal::prelude::*;

#[test]
fn solve_1_plus_1() {
    let test = "1+1\n";
    let priklad: Priklad<Decimal> = test.parse().unwrap();
    assert_eq!(priklad.solve().unwrap(), Decimal::from_str("2.0").unwrap());
}
#[test]
fn solve_1_times_1() {
    let test = "1*1\n";
    let priklad: Priklad<Decimal> = test.parse().unwrap();
    assert_eq!(priklad.solve().unwrap(), Decimal::from_str("1.0").unwrap());
}
#[test]
fn solve_compl_plus_1() {
    let test = "1+2+3+4+5\n";
    let priklad: Priklad<Decimal> = test.parse().unwrap();
    assert_eq!(priklad.solve().unwrap(), Decimal::from_str("15.0").unwrap());
}
#[test]
fn solve_compl_mult_1() {
    let test = "1*2*3*4*5\n";
    let priklad: Priklad<Decimal> = test.parse().unwrap();
    assert_eq!(
        priklad.solve().unwrap(),
        Decimal::from_str("120.0").unwrap()
    );
}
#[test]
fn solve_compl_mix_1() {
    let test = "1+2*3-4/5\n";
    let priklad: Priklad<Decimal> = test.parse().unwrap();
    assert_eq!(priklad.solve().unwrap(), Decimal::from_str("6.2").unwrap());
}
#[test]
fn solve_neg_1() {
    let test = "-1+1\n";
    let priklad: Priklad<Decimal> = test.parse().unwrap();
    assert_eq!(priklad.solve().unwrap(), Decimal::from_str("0.0").unwrap());
}
#[test]
fn solve_neg_float_1() {
    let test = "-1.23+1.23\n";
    let priklad: Priklad<Decimal> = test.parse().unwrap();
    assert_eq!(priklad.solve().unwrap(), Decimal::from_str("0.0").unwrap());
}
#[test]
fn solve_float_1() {
    let test = "-1.888+1.222\n";
    let priklad: Priklad<Decimal> = test.parse().unwrap();
    assert_eq!(
        priklad.solve().unwrap(),
        Decimal::from_str("-0.666").unwrap()
    );
}
