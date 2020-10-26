use anyhow::{anyhow, Result};
use rust_decimal::prelude::*;
use std::{
    ops::{Deref, DerefMut},
    str::FromStr,
};

/// Basic struct that is either operation or number
/// This struct is is in Vec in struct Priklad
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Entry {
    Operation(Operation),
    Number(Decimal),
}

impl Entry {
    fn get_num(&self) -> Result<Decimal> {
        if let Entry::Number(num) = self {
            Ok(num.to_owned())
        } else {
            Err(anyhow!("Unexpected error. Expected number got {:?}", &self))
        }
    }
    fn get_op(&self) -> Result<Operation> {
        if let Entry::Operation(op) = self {
            Ok(op.to_owned())
        } else {
            Err(anyhow!(
                "Unexpected error. Expected operation got {:?}",
                &self
            ))
        }
    }
}

/// List of supported operations
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Operation {
    Plus,
    Minus,
    Mult,
    Div,
}

impl Operation {
    pub fn from(c: &char) -> Result<Self> {
        match c {
            '+' => Ok(Operation::Plus),
            '-' => Ok(Operation::Minus),
            '*' => Ok(Operation::Mult),
            '/' => Ok(Operation::Div),
            _ => Err(anyhow!("Unexpected char {}", c)),
        }
    }
    fn is_mult(&self) -> bool {
        matches!(self, Operation::Mult) || matches!(self, Operation::Div)
    }
    fn calc(&self, num1: Decimal, num2: Decimal) -> Result<Decimal> {
        match self {
            Operation::Plus => Ok(num1 + num2),
            Operation::Minus => Ok(num1 - num2),
            Operation::Mult => Ok(num1 * num2),
            Operation::Div => {
                if num2 == Decimal::from(0) {
                    Err(anyhow!("Division by 0"))
                } else {
                    Ok(num1 / num2)
                }
            }
        }
    }
}

/// Struct that holds the math problem
#[derive(Default, Debug, Eq, PartialEq)]
pub struct Priklad {
    pub inner: Vec<Entry>,
    pub idx: usize,
}

impl Priklad {
    pub fn solve(mut self) -> Result<Decimal> {
        while let Some(t) = self.next() {
            if t.op.is_mult() {
                self.replace(t.op.calc(t.num1, t.num2)?);
            }
        }
        while self.len() != 1 {
            // FIXME: Why is this needed? Where does the iter break?
            self.reset();
            while let Some(t) = self.next() {
                self.replace(t.op.calc(t.num1, t.num2)?);
            }
        }
        if self.len() != 1 {
            Err(anyhow!("No successful finish\n{:?}", &self))
        } else {
            Ok(self[0].get_num()?)
        }
    }
    fn reset(&mut self) {
        self.idx = 0
    }
    fn replace(&mut self, solution: Decimal) {
        let range = self.idx - 2..=self.idx;
        let replace_with = vec![Entry::Number(solution)];
        self.splice(range, replace_with);
    }
}

impl Iterator for Priklad {
    type Item = Triplet;

    fn next(&mut self) -> Option<Self::Item> {
        let num1 = self.get(self.idx)?.get_num().unwrap(); // TODO remove unwrap
        self.idx += 1;
        let op = self.get(self.idx)?.get_op().unwrap(); // TODO remove unwrap
        self.idx += 1;
        let num2 = self[self.idx].get_num().unwrap(); // TODO remove unwrap
        Some(Triplet {
            num1,
            op,
            num2,
            idx: self.idx,
        })
    }
}

impl FromStr for Priklad {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut buff = String::new();
        let mut priklad: Priklad = Priklad::default();

        for (n, c) in s.chars().enumerate() {
            if c == ' ' {
                continue;
            };
            if c.is_digit(10) || c == '.' || (n == 0 && Operation::from(&c)? == Operation::Minus) {
                buff.push(c);
            } else {
                if let Ok(num) = buff.trim().parse::<Decimal>() {
                    priklad.push(Entry::Number(num));
                } else {
                    return Err(anyhow!("Cannot end with an operation"));
                }
                buff = "".into();
                if c != '\n' {
                    priklad.push(Entry::Operation(Operation::from(&c)?));
                };
            }
        }
        Ok(priklad)
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct Triplet {
    num1: Decimal,
    op: Operation,
    num2: Decimal,
    idx: usize,
}

impl Deref for Priklad {
    type Target = Vec<Entry>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Priklad {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

// needed for Test
impl PartialEq<(&str, Operation, &str)> for Triplet {
    fn eq(&self, other: &(&str, Operation, &str)) -> bool {
        self.num1 == Decimal::from_str(other.0).unwrap()
            && self.op == other.1
            && self.num2 == Decimal::from_str(other.2).unwrap()
    }
}
