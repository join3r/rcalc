use rcalc::*;
use rust_decimal::prelude::*;

#[test]
fn iter_1_plus_1() {
    let test = "1+1\n";
    let mut priklad: Priklad<Decimal> = test.parse().unwrap();
    assert_eq!(priklad.next().unwrap(), (Decimal::from_str("1.0").unwrap(), Operation::Plus, Decimal::from_str("1.0").unwrap()));
}
#[test]
fn iter_1_times_1() {
    let test = "1*1\n";
    let mut priklad: Priklad<Decimal> = test.parse().unwrap();
    assert_eq!(priklad.next().unwrap(), (Decimal::from_str("1.0").unwrap(), Operation::Mult, Decimal::from_str("1.0").unwrap()));
    
}
#[test]
fn iter_compl_plus_1() {
    let test = "1+2+3+4+5\n";
    let mut priklad: Priklad<Decimal> = test.parse().unwrap();
    assert_eq!(priklad.next().unwrap(), (Decimal::from_str("1.0").unwrap(), Operation::Plus, Decimal::from_str("2.0").unwrap()));
    assert_eq!(priklad.next().unwrap(), (Decimal::from_str("2.0").unwrap(), Operation::Plus, Decimal::from_str("3.0").unwrap()));
    assert_eq!(priklad.next().unwrap(), (Decimal::from_str("3.0").unwrap(), Operation::Plus, Decimal::from_str("4.0").unwrap()));
    assert_eq!(priklad.next().unwrap(), (Decimal::from_str("4.0").unwrap(), Operation::Plus, Decimal::from_str("5.0").unwrap()));
    
}
#[test]
fn iter_compl_mult_1() {
    let test = "1*2*3*4*5\n";
    let mut priklad: Priklad<Decimal> = test.parse().unwrap();
    assert_eq!(priklad.next().unwrap(), (Decimal::from_str("1.0").unwrap(), Operation::Mult, Decimal::from_str("2.0").unwrap()));
    assert_eq!(priklad.next().unwrap(), (Decimal::from_str("2.0").unwrap(), Operation::Mult, Decimal::from_str("3.0").unwrap()));
    assert_eq!(priklad.next().unwrap(), (Decimal::from_str("3.0").unwrap(), Operation::Mult, Decimal::from_str("4.0").unwrap()));
    assert_eq!(priklad.next().unwrap(), (Decimal::from_str("4.0").unwrap(), Operation::Mult, Decimal::from_str("5.0").unwrap()));
    
}
#[test]
fn iter_compl_mix_1() {
    let test = "1+2*3-4/5\n";
    let mut priklad: Priklad<Decimal> = test.parse().unwrap();
    assert_eq!(priklad.next().unwrap(), (Decimal::from_str("1.0").unwrap(), Operation::Plus, Decimal::from_str("2.0").unwrap()));
    assert_eq!(priklad.next().unwrap(), (Decimal::from_str("2.0").unwrap(), Operation::Mult, Decimal::from_str("3.0").unwrap()));
    assert_eq!(priklad.next().unwrap(), (Decimal::from_str("3.0").unwrap(), Operation::Minus, Decimal::from_str("4.0").unwrap()));
    assert_eq!(priklad.next().unwrap(), (Decimal::from_str("4.0").unwrap(), Operation::Div, Decimal::from_str("5.0").unwrap()));
    
}
#[test]
fn iter_neg_1() {
    let test = "-1+1\n";
    let mut priklad: Priklad<Decimal> = test.parse().unwrap();
    assert_eq!(priklad.next().unwrap(), (Decimal::from_str("-1.0").unwrap(), Operation::Plus, Decimal::from_str("1.0").unwrap()));
}

#[test]
fn iter_neg_float_1() {
    let test = "-1.23+1.23\n";
    let mut priklad: Priklad<Decimal> = test.parse().unwrap();
    assert_eq!(priklad.next().unwrap(), (Decimal::from_str("-1.23").unwrap(), Operation::Plus, Decimal::from_str("1.23").unwrap()));
    
}
#[test]
fn iter_float_1() {
    let test = "-1.888+1.222\n";
    let mut priklad: Priklad<Decimal> = test.parse().unwrap();
    assert_eq!(priklad.next().unwrap(), (Decimal::from_str("-1.888").unwrap(), Operation::Plus, Decimal::from_str("1.222").unwrap()));

}