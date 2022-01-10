use std::collections::HashMap;

use serde::{Serialize, Deserialize};


pub type Formula = String;
pub type Id = u64;  // Conform to serde_json::Value
pub type Name = String;

#[derive(Serialize, Deserialize, Debug)]
#[derive(PartialEq)]
pub enum IndexedOrLiteral<T> {
    Index(Id),
    Literal(T),
}


#[derive(Serialize, Deserialize, Debug)]
#[derive(PartialEq)]
pub enum HitAmt {
    Constant(i32),
    Formula(String),
}
#[derive(Serialize, Deserialize, Debug)]
#[derive(PartialEq)]
pub struct Hit {
    pub pool: Name,
    pub amount: HitAmt,
}
pub type Hits = Vec::<Hit>;


pub trait Target {
    // Hits should be borrowed by Targets since a single Hit may be
    // applied to multiple Targets (by an Action).
    fn take_hit(&mut self, hit: &Hit) -> i32;
    fn take_condition(&mut self, hit: &Hit) -> bool;
}
