use rust_decimal::Decimal;
use crate::Transaction;
use std::fmt::Display;
use std::{io::Write};
use tabwriter::TabWriter;
use colored::Colorize;


pub struct Balance {
    pub income: Decimal,
    pub outcome: Decimal,
}

impl Display for Balance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        let income = format!("{:.2}",self.income).green();
        let outcome = format!("{:.2}",self.outcome).red();


        let total = self.income + self.outcome;
        let total = if total < Decimal::ZERO{
            format!("{:.2}",total).red()
        }else {
            format!("{:.2}",total).green()
        };


        let mut tw = TabWriter::new(vec![]);

        writeln!(&mut tw, "Income:\t{}",income).unwrap();
        writeln!(&mut tw, "Outcome:\t{}",outcome).unwrap();
        writeln!(&mut tw, "Total:\t{}",total).unwrap();


        tw.flush().unwrap();
        let written = String::from_utf8(tw.into_inner().unwrap()).unwrap();
        write!(f, "{}", written)?;
        Ok(())

    }
}

pub trait ComputeBalance {
    fn balance(&self) -> Balance;
}

impl ComputeBalance for Vec<&Transaction>{
    fn balance(&self) -> Balance{
        let mut income = Decimal::ZERO;
        let mut outcome = Decimal::ZERO;

        for &tr in self {
            if tr.amount < Decimal::ZERO{
                outcome += tr.amount;
            }else {
                income += tr.amount;
            }
        }

        Balance{income,outcome}
    }
}