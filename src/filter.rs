use crate::Transaction;


pub trait Filter {    
    fn merchant(&self, merchant: &str) -> Vec<&Transaction>;
}


impl Filter for Vec<&Transaction>{
    
    fn merchant(&self, merchant: &str) -> Vec<&Transaction>{
        self.iter()
        .filter(|tr| tr.merchant == Some(merchant.to_string()))
        .copied()
        .collect()
    }

    
}