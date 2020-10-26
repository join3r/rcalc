use rcalc::*;

#[test]
fn iter_1_plus_1() {
    let test = "1+1\n";
    let mut priklad: Priklad = test.parse().unwrap();
    assert_eq!(priklad.next().unwrap(), ("1.0", Operation::Plus, "1.0"));
}
#[test]
fn iter_1_times_1() {
    let test = "1*1\n";
    let mut priklad: Priklad = test.parse().unwrap();
    assert_eq!(priklad.next().unwrap(), ("1.0", Operation::Mult, "1.0"));
}

#[test]
fn iter_compl_plus_1() {
    let test = "1+2+3+4+5\n";
    let mut priklad: Priklad = test.parse().unwrap();
    assert_eq!(priklad.next().unwrap(), ("1.0", Operation::Plus, "2.0"));
    assert_eq!(priklad.next().unwrap(), ("2.0", Operation::Plus, "3.0"));
    assert_eq!(priklad.next().unwrap(), ("3.0", Operation::Plus, "4.0"));
    assert_eq!(priklad.next().unwrap(), ("4.0", Operation::Plus, "5.0"));
}
#[test]
fn iter_compl_mult_1() {
    let test = "1*2*3*4*5\n";
    let mut priklad: Priklad = test.parse().unwrap();
    assert_eq!(priklad.next().unwrap(), ("1.0", Operation::Mult, "2.0"));
    assert_eq!(priklad.next().unwrap(), ("2.0", Operation::Mult, "3.0"));
    assert_eq!(priklad.next().unwrap(), ("3.0", Operation::Mult, "4.0"));
    assert_eq!(priklad.next().unwrap(), ("4.0", Operation::Mult, "5.0"));
}
#[test]
fn iter_compl_mix_1() {
    let test = "1+2*3-4/5\n";
    let mut priklad: Priklad = test.parse().unwrap();
    assert_eq!(priklad.next().unwrap(), ("1.0", Operation::Plus, "2.0"));
    assert_eq!(priklad.next().unwrap(), ("2.0", Operation::Mult, "3.0"));
    assert_eq!(priklad.next().unwrap(), ("3.0", Operation::Minus, "4.0"));
    assert_eq!(priklad.next().unwrap(), ("4.0", Operation::Div, "5.0"));
}
#[test]
fn iter_neg_1() {
    let test = "-1+1\n";
    let mut priklad: Priklad = test.parse().unwrap();
    assert_eq!(priklad.next().unwrap(), ("-1.0", Operation::Plus, "1.0"));
}

#[test]
fn iter_neg_float_1() {
    let test = "-1.23+1.23\n";
    let mut priklad: Priklad = test.parse().unwrap();
    assert_eq!(priklad.next().unwrap(), ("-1.23", Operation::Plus, "1.23"));
}
#[test]
fn iter_float_1() {
    let test = "-1.888+1.222\n";
    let mut priklad: Priklad = test.parse().unwrap();
    assert_eq!(
        priklad.next().unwrap(),
        ("-1.888", Operation::Plus, "1.222")
    );
}
