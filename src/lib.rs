pub use anyhow::{anyhow, Result};
use std::{ops::{Deref, DerefMut}, str::FromStr};

/// Basic struct that is either operation or number
/// This struct is is in Vec in struct Priklad
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Entry<T> {
    Operation(Operation),
    Number(T),
}

impl<T> Entry<T>
where
    T: std::fmt::Debug + std::marker::Copy,
{
    fn get_num(&self) -> Result<T> {
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
    fn calc<T>(&self, num1: T, num2: T) -> T
    where
        T: std::ops::Add<Output = T>
            + std::ops::Sub<Output = T>
            + std::ops::Mul<Output = T>
            + std::ops::Div<Output = T>
    {
        match self {
            Operation::Plus => num1 + num2,
            Operation::Minus => num1 - num2,
            Operation::Mult => num1 * num2,
            Operation::Div => num1 / num2,
        }
    }
}

/// Struct that holds the math problem
#[derive(Default, Debug, Eq, PartialEq)]
pub struct Priklad<T> {
    pub inner: Vec<Entry<T>>,
    pub idx: usize,
}

impl<T> Priklad<T>
where
    T: std::default::Default + std::fmt::Debug + std::marker::Copy,
{
    pub fn solve(mut self) -> Result<T>
    where
        T: std::ops::Mul<Output = T>
            + std::ops::Div<Output = T>
            + std::ops::Add<Output = T>
            + std::ops::Sub<Output = T>,
    {
        while let Some((num1, op, num2)) = self.next() {
            if op.is_mult() {
                self.replace(op.calc(num1, num2));
            }
        }
        while self.len() != 1 { // FIXME: Why is this needed? Where does the iter break?
            self.reset();
            while let Some((num1, op, num2)) = self.next() {
                self.replace(op.calc(num1, num2));
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
    fn replace(&mut self, solution: T) {
        let range = self.idx - 2..=self.idx;
        let replace_with = vec![Entry::Number(solution)];
        self.splice(range, replace_with);
    }
}

impl<T> Iterator for Priklad<T>
where
    T: std::fmt::Debug + std::marker::Copy,
{
    type Item = (T, Operation, T);

    fn next(&mut self) -> Option<Self::Item> {
        let num1 = self.get(self.idx)?.get_num().unwrap(); // TODO remove unwrap
        self.idx += 1;
        let op = self.get(self.idx)?.get_op().unwrap(); // TODO remove unwrap
        self.idx += 1;
        let num2 = self[self.idx].get_num().unwrap(); // TODO remove unwrap
        Some((num1, op, num2))
    }
}

impl<T> FromStr for Priklad<T>
where
    T: std::default::Default + std::str::FromStr + std::ops::Neg,
    <T as std::str::FromStr>::Err: std::error::Error,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, anyhow::Error> {
        let mut buff = String::new();
        let mut priklad: Priklad<T> = Priklad::default();

        for (n, c) in s.chars().enumerate() {
            if c == ' ' {
                continue;
            };
            if c.is_digit(10) || c == '.' || (n == 0 && Operation::from(&c)? == Operation::Minus) {
                buff.push(c);
            } else {
                if !buff.trim().is_empty() {
                    priklad
                        
                        .push(Entry::Number(buff.trim().parse().unwrap()));
                };
                buff = "".into();
                if c != '\n' {
                    priklad.push(Entry::Operation(Operation::from(&c)?));
                };
            }
        }
        Ok(priklad)
    }
}

impl<T> Deref for Priklad<T> {
    type Target = Vec<Entry<T>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for Priklad<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}