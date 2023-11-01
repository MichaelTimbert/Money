use crate::balance::ComputeBalance;
use crate::{Transaction, balance::Balance};
use crate::filter::Filter;
use std::collections::HashSet;
use indexmap::IndexMap;



pub trait Tag {
    fn list_tag(&self) -> HashSet<String>;
    fn balance_for_tag(&self, tag: &str) -> Balance;
    fn tags_balance(&self) -> IndexMap<String, Balance>;

}


impl Tag for Vec<&Transaction>{
    fn list_tag(&self) -> HashSet<String>{
        let mut tags = HashSet::new();

        for tr in self{
            if let Some(t) = &tr.tag {
                tags.insert(t.clone());
            }
        }

        tags
    }

    fn balance_for_tag(&self, tag: &str) -> Balance{
        self.tag(tag).balance()
    }


    fn tags_balance(&self) -> IndexMap<String, Balance>{
        let mut im = IndexMap::<String, Balance>::new();

        for tag in self.list_tag(){
            let tbalance = self.balance_for_tag(&tag);
            im.insert(tag, tbalance);
        }

        im.sort_by(|k1,v1,k2,v2| v1.total().cmp(&v2.total()));
        im
    }
}