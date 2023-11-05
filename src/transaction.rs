use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::io::Write;
use tabwriter::TabWriter;
use colored::Colorize;

#[derive(Debug,Serialize, Deserialize)]
pub struct Transaction {
    // unique id of the depense
    pub id: usize,
    // Date of the transaction
    pub date: NaiveDate,
    // Amount of the transaction
    pub amount: Decimal,
    // Optional Note/Description
    pub note: Option<String>,
    // Optional Merchant
    pub merchant: Option<String>,
    // Optional Tag
    pub tag: Option<String>,
    // Optional Account
    pub account: Option<String>,
}

pub trait Operation {
    fn display(&self);
}


impl Operation for Vec<&Transaction> {
    fn display(&self) {
        let mut tw = TabWriter::new(vec![]);

        writeln!(&mut tw, "{}\t{}\t{}\t{}\t{}\t{}","id","date","amount","tag","merchant","note").unwrap();


        for &tr in self {
            let amount  = if tr.amount < Decimal::ZERO {
                format!("{:.2}",tr.amount).red()
            }else {
                format!("{:.2}",tr.amount).green()
            };
            
            let tag = if let Some(t) = &tr.tag{
                t.to_string()
            }else {
                "".to_string()
            };

            let merchant = if let Some(m) = &tr.merchant{
                m.to_string()
            }else {
                "".to_string()
            };

            let note = if let Some(n) = &tr.note{
                format!("note: {n}")
            }else {
                "".to_string()
            };

            writeln!(&mut tw, "{}\t{}\t{}\t{}\t{}\t{}",tr.id,tr.date,amount,tag,merchant,note).unwrap();
        }

        tw.flush().unwrap();
        let written = String::from_utf8(tw.into_inner().unwrap()).unwrap();
        println!("{}", written);
    }

}