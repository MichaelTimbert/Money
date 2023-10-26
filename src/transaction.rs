use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize, Deserialize)]
pub struct Transaction {
    // unique id of the depense
    pub id: usize,
    // Date of the transaction
    pub date: NaiveDate,
    // Amount of the transaction
    pub amount: Decimal,
}