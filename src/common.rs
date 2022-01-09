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
