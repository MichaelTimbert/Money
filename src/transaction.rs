use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::io::Write;
use tabwriter::TabWriter;

#[derive(Debug,Serialize, Deserialize)]
pub struct Transaction {
    // unique id of the depense
    pub id: usize,
    // Date of the transaction
    pub date: NaiveDate,
    // Amount of the transaction
    pub amount: Decimal,
}

pub trait Operation {
    fn display(&self);
}


impl Operation for Vec<&Transaction> {
    fn display(&self) {
        let mut tw = TabWriter::new(vec![]);

        writeln!(&mut tw, "{}\t{}\t{}","id","date","amount").unwrap();


        for &tr in self {
            writeln!(&mut tw, "{}\t{}\t{}",tr.id,tr.date,tr.amount).unwrap();
        }

        tw.flush().unwrap();
        let written = String::from_utf8(tw.into_inner().unwrap()).unwrap();
        println!("{}", written);
    }

}