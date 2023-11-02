use tabwriter::TabWriter;
use std::io::Write;
use indexmap::IndexMap;
use crate::balance::Balance;

pub trait Display{
    fn display(&self);
}

impl Display for IndexMap<String, Balance>{
    fn display(&self){
        let mut tw = TabWriter::new(vec![]);

        writeln!(&mut tw, "key\tincome\toutcome\ttotal").unwrap();

        for i in self {
            let key = i.0;
            let balance = i.1;
            // let amount  = if tr.amount < Decimal::ZERO {
            //     format!("{:.2}",tr.amount).red()
            // }else {
            //     format!("{:.2}",tr.amount).green()
            // };
            
            // let tag = if let Some(t) = &tr.tag{
            //     t.to_string()
            // }else {
            //     "".to_string()
            // };

            // let merchant = if let Some(m) = &tr.merchant{
            //     m.to_string()
            // }else {
            //     "".to_string()
            // };

            // let note = if let Some(n) = &tr.note{
            //     format!("note: {n}")
            // }else {
            //     "".to_string()
            // };

            writeln!(&mut tw, "{}\t{:.2}\t{:.2}\t{:.2}",key, balance.income, balance.outcome, balance.total()).unwrap();
            
        }
        let mut total = Balance::default();
        let _ :Vec<_>= self.iter().map(|i| total+=i.1.clone()).collect();

        writeln!(&mut tw,"total\t{:.2}\t{:.2}\t{:.2}",total.income, total.outcome, total.total()).unwrap();

        tw.flush().unwrap();
        let written = String::from_utf8(tw.into_inner().unwrap()).unwrap();
        println!("{}", written);


    }
}