mod lib;
use lib::{Priklad, Result};
use std::io::{stdin, stdout, Write};
use rust_decimal::prelude::*;

fn main() -> Result<()> {
    loop {
        print!("Priklad: ");
        stdout().flush()?;
        let mut input = String::new();
        stdin().read_line(&mut input)?;
        if input == "\n" {
            break;
        };
        let priklad: Priklad<Decimal> = input.parse()?;
        let vysledok: Decimal = priklad.solve()?;
        println!("{} = {}", input.trim(), vysledok);
    }
    Ok(())
}
