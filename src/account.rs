use crate::balance::ComputeBalance;
use crate::{Transaction, balance::Balance};
use crate::filter::Filter;
use std::collections::HashSet;
use indexmap::IndexMap;



pub trait Account {
    fn list_account(&self) -> HashSet<String>;
    fn balance_for_account(&self, tag: &str) -> Balance;
    fn accounts_balance(&self) -> IndexMap<String, Balance>;

}


impl Account for Vec<&Transaction>{
    fn list_account(&self) -> HashSet<String>{
        let mut accounts = HashSet::new();

        for tr in self{
            if let Some(a) = &tr.account {
                accounts.insert(a.clone());
            }
        }

        accounts
    }

    fn balance_for_account(&self, account: &str) -> Balance{
        self.account(account).balance()
    }


    fn accounts_balance(&self) -> IndexMap<String, Balance>{
        let mut im = IndexMap::<String, Balance>::new();

        for account in self.list_account(){
            let abalance = self.balance_for_account(&account);
            im.insert(account, abalance);
        }

        im.sort_by(|_k1,v1,_k2,v2| v1.total().cmp(&v2.total()));
        im
    }
}