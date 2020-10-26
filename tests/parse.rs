use rcalc::*;
use rust_decimal::prelude::*;

#[test]
fn parse_1_plus_1() {
    let test = " 1 + 1 \n";
    let par: Priklad = test.parse().unwrap();
    let comp: Priklad = Priklad {
        inner: vec![
            Entry::Number(Decimal::from_str("1.0").unwrap()),
            Entry::Operation(Operation::Plus),
            Entry::Number(Decimal::from_str("1.0").unwrap()),
        ],
        ..Default::default()
    };
    assert_eq!(par, comp);
}
#[test]
fn parse_1_times_1() {
    let test = "1*1\n";
    let par: Priklad = test.parse().unwrap();
    let comp: Priklad = Priklad {
        inner: vec![
            Entry::Number(Decimal::from_str("1.0").unwrap()),
            Entry::Operation(Operation::Mult),
            Entry::Number(Decimal::from_str("1.0").unwrap()),
        ],
        ..Default::default()
    };
    assert_eq!(par, comp);
}

#[test]
fn parse_compl_plus_1() {
    let test = "1+2+3+4+5\n";
    let par: Priklad = test.parse().unwrap();
    let comp: Priklad = Priklad {
        inner: vec![
            Entry::Number(Decimal::from_str("1.0").unwrap()),
            Entry::Operation(Operation::Plus),
            Entry::Number(Decimal::from_str("2.0").unwrap()),
            Entry::Operation(Operation::Plus),
            Entry::Number(Decimal::from_str("3.0").unwrap()),
            Entry::Operation(Operation::Plus),
            Entry::Number(Decimal::from_str("4.0").unwrap()),
            Entry::Operation(Operation::Plus),
            Entry::Number(Decimal::from_str("5.0").unwrap()),
        ],
        ..Default::default()
    };
    assert_eq!(par, comp);
}
#[test]
fn parse_compl_minus_1() {
    let test = "-1-2-3-4-5\n";
    let par: Priklad = test.parse().unwrap();
    let comp: Priklad = Priklad {
        inner: vec![
            Entry::Number(Decimal::from_str("-1.0").unwrap()),
            Entry::Operation(Operation::Minus),
            Entry::Number(Decimal::from_str("2.0").unwrap()),
            Entry::Operation(Operation::Minus),
            Entry::Number(Decimal::from_str("3.0").unwrap()),
            Entry::Operation(Operation::Minus),
            Entry::Number(Decimal::from_str("4.0").unwrap()),
            Entry::Operation(Operation::Minus),
            Entry::Number(Decimal::from_str("5.0").unwrap()),
        ],
        ..Default::default()
    };
    assert_eq!(par, comp);
}
#[test]
fn parse_compl_mult_1() {
    let test = "1*2*3*4*5\n";
    let par: Priklad = test.parse().unwrap();
    let comp: Priklad = Priklad {
        inner: vec![
            Entry::Number(Decimal::from_str("1.0").unwrap()),
            Entry::Operation(Operation::Mult),
            Entry::Number(Decimal::from_str("2.0").unwrap()),
            Entry::Operation(Operation::Mult),
            Entry::Number(Decimal::from_str("3.0").unwrap()),
            Entry::Operation(Operation::Mult),
            Entry::Number(Decimal::from_str("4.0").unwrap()),
            Entry::Operation(Operation::Mult),
            Entry::Number(Decimal::from_str("5.0").unwrap()),
        ],
        ..Default::default()
    };
    assert_eq!(par, comp);
}

#[test]
fn parse_compl_mix_1() {
    let test = "1+2*3-4/5\n";
    let par: Priklad = test.parse().unwrap();
    let comp: Priklad = Priklad {
        inner: vec![
            Entry::Number(Decimal::from_str("1.0").unwrap()),
            Entry::Operation(Operation::Plus),
            Entry::Number(Decimal::from_str("2.0").unwrap()),
            Entry::Operation(Operation::Mult),
            Entry::Number(Decimal::from_str("3.0").unwrap()),
            Entry::Operation(Operation::Minus),
            Entry::Number(Decimal::from_str("4.0").unwrap()),
            Entry::Operation(Operation::Div),
            Entry::Number(Decimal::from_str("5.0").unwrap()),
        ],
        ..Default::default()
    };
    assert_eq!(par, comp);
}

#[test]
fn parse_neg_1() {
    let test = "-1+1\n";
    let par: Priklad = test.parse().unwrap();
    let comp: Priklad = Priklad {
        inner: vec![
            Entry::Number(Decimal::from_str("-1.0").unwrap()),
            Entry::Operation(Operation::Plus),
            Entry::Number(Decimal::from_str("1.0").unwrap()),
        ],
        ..Default::default()
    };
    assert_eq!(par, comp);
}

#[test]
fn parse_neg_float_1() {
    let test = "-1.23+1.23\n";
    let par: Priklad = test.parse().unwrap();
    let comp: Priklad = Priklad {
        inner: vec![
            Entry::Number(Decimal::from_str("-1.23").unwrap()),
            Entry::Operation(Operation::Plus),
            Entry::Number(Decimal::from_str("1.23").unwrap()),
        ],
        ..Default::default()
    };
    assert_eq!(par, comp);
}

#[test]
fn parse_float_1() {
    let test = "-1.888+1.222\n";
    let par: Priklad = test.parse().unwrap();
    let comp: Priklad = Priklad {
        inner: vec![
            Entry::Number(Decimal::from_str("-1.888").unwrap()),
            Entry::Operation(Operation::Plus),
            Entry::Number(Decimal::from_str("1.222").unwrap()),
        ],
        ..Default::default()
    };
    assert_eq!(par, comp);
}
