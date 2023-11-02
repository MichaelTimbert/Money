use rust_decimal::Decimal;
use tabwriter::TabWriter;
use std::io::Write;
use indexmap::IndexMap;
use crate::balance::Balance;
use colored::Colorize;

pub trait Display{
    fn display(&self);
}

impl Display for IndexMap<String, Balance>{
    fn display(&self){
        let mut tw = TabWriter::new(vec![]);

        writeln!(&mut tw, "{}\t{}\t{}\t{}","key","income","outcome","total").unwrap();

        for i in self {
            let key = i.0;
            let balance = i.1;

            let income  = if balance.income < Decimal::ZERO{
                format!("{:.2}",balance.income).red()
            }else {
                format!("{:.2}",balance.income).green()
            };

            let outcome  = if balance.outcome < Decimal::ZERO{
                format!("{:.2}",balance.outcome).red()
            }else {
                format!("{:.2}",balance.outcome).green()
            };

            let total  = if balance.total() < Decimal::ZERO{
                format!("{:.2}",balance.total()).red()
            }else {
                format!("{:.2}",balance.total()).green()
            };

            writeln!(&mut tw, "{}\t{}\t{}\t{}",key, income, outcome, total).unwrap();
            
        }
        let mut total = Balance::default();
        let _ :Vec<_>= self.iter().map(|i| total+=i.1.clone()).collect();

        writeln!(&mut tw,"total\t{:.2}\t{:.2}\t{:.2}",total.income, total.outcome, total.total()).unwrap();

        tw.flush().unwrap();
        let written = String::from_utf8(tw.into_inner().unwrap()).unwrap();
        println!("{}", written);


    }
}