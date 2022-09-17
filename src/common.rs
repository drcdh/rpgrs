use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum PlayerType {
    NPC,
    PC,
}

pub type Formula = String;
pub type Id = u64;  // Conform to serde_json::Value
pub type Name = String;

#[derive(Serialize, Deserialize, Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum IndexedOrLiteral<T> {
    Index(Id),
    Literal(T),
}


#[derive(Serialize, Deserialize, Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum HitAmt {
    Constant(i32),
    Formula(Formula),
//todo    StandardFormula(StandardFormula),
}
#[derive(Serialize, Deserialize, Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub struct Hit {
    pub pool: Name,
    pub amount: HitAmt,
}
pub type Hits = Vec::<Hit>;


pub trait Target {
    // Hits should be borrowed by Targets since a single Hit may be
    // applied to multiple Targets (by an Action).
    // CORRECTION: this isn't correct. An Action may apply the same
    // Effect multiple times, but an Effect needs each of its Hit-s
    // only once.
    // ANOTHER CORRECTION: actually it was correct, but because these
    // methods shouldn't be taking ownership of the Hit objects from
    // the Effect.
    fn hit_pool(&mut self, pool: &Name, amount: i32) -> i32;
//    fn take_condition(&mut self, cond: &Condition) -> bool;
}
