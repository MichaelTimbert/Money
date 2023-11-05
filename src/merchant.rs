use crate::balance::ComputeBalance;
use crate::{Transaction, balance::Balance};
use crate::filter::Filter;
use std::collections::HashSet;
use indexmap::IndexMap;



pub trait Merchant {
    fn list_merchant(&self) -> HashSet<String>;
    fn balance_for_merchant(&self, merchant: &str) -> Balance;
    fn merchants_balance(&self) -> IndexMap<String, Balance>;

}


impl Merchant for Vec<&Transaction>{
    fn list_merchant(&self) -> HashSet<String>{
        let mut merchants = HashSet::new();

        for tr in self{
            if let Some(m) = &tr.merchant {
                merchants.insert(m.clone());
            }
        }

        merchants
    }

    fn balance_for_merchant(&self, merchant: &str) -> Balance{
        self.merchant(merchant).balance()
    }


    fn merchants_balance(&self) -> IndexMap<String, Balance>{
        let mut im = IndexMap::<String, Balance>::new();

        for merchant in self.list_merchant(){
            let mbalance = self.balance_for_merchant(&merchant);
            im.insert(merchant, mbalance);
        }

        im.sort_by(|_k1,v1,_k2,v2| v1.total().cmp(&v2.total()));
        im
    }
}