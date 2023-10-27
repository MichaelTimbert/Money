use crate::Transaction;
use std::fs::OpenOptions;
use serde::{Deserialize, Serialize};
use itertools::Itertools;

#[derive(Default, Serialize, Deserialize)]
pub struct DataBase {
    transactions: Vec<Transaction>,
}



impl DataBase{
    pub fn load(file: &std::path::PathBuf) -> DataBase{
        //info!("Loading database '{}'", file.display());
        if file.exists() {
            // info!("loading database from {}",file);
            //debug!("Loading from existing file");
            let dbf = OpenOptions::new().read(true).open(file).unwrap();

            let db: DataBase = serde_json::from_reader(dbf).unwrap();

            db
        } else {
            //debug!("Creating new database");
            Default::default()
        }
    }
    
    pub fn store(self, path: &std::path::PathBuf) {
        let dbf = OpenOptions::new()
        .create(true) // create it if doesnt existe
        .truncate(true) // if it alredy existe overwrite it
        .write(true)
        .open(path)
        .unwrap(); // TODO : catch

        // TODO : Catch Result
        let _ = serde_json::to_writer(dbf, &self);
    }

    pub fn last_transaction_id(&self) -> usize{
        let mut last_id = 0;
        self.transactions.iter().for_each(|tr| if tr.id > last_id { last_id=tr.id});
        last_id
    }

    pub fn new_id(&self) -> usize {
        self.last_transaction_id() + 1
    }

    pub fn add_transaction(&mut self, mut tr: Transaction) {
        tr.id = self.new_id();
        self.transactions.push(tr);
    }

    pub fn list_transaction(&self) -> Vec<&Transaction> {
        self.transactions
            .iter()
            .sorted_by(|a, b| a.date.cmp(&b.date))
            .collect()
    }

    pub fn remove_transaction(&mut self, id:usize) -> Option<Transaction>{

        for (pos, tr) in self.transactions.iter().enumerate() {
            if tr.id == id {
                let tr_rm = self.transactions.remove(pos);
                return Some(tr_rm);
            }
        }
        None

    }

}