use std::collections::HashMap;
use std::error::Error;

use serde::{Serialize, Deserialize};

use crate::common::{Id, Name, Formula};
use crate::encyclopedia::{Encyclopedia, read_encyclopedia};


pub type Stat = i64;
pub type BaseStats = HashMap<Name, Stat>;
pub type DerivedStat = Formula;
pub type DerivedStats = HashMap<Name, DerivedStat>;

#[derive(Serialize, Deserialize, Debug)]
pub struct StatBlock {
    base_stats: BaseStats,
    stats: DerivedStats,
}

pub type StatBlocks = Encyclopedia<StatBlock>;

pub fn get_statblocks(filename: &str) -> Result<StatBlocks, Box<dyn Error>> {
    read_encyclopedia::<StatBlock>(filename)
}

// TODO: this is of course terrible
pub fn generate_stats(id: &Id) -> (BaseStats, DerivedStats) {
    let statblocks = get_statblocks("data/stats.json").unwrap();
    let sb = statblocks.get(&id).unwrap();
    let (bs, ds) = (sb.base_stats.clone(), sb.stats.clone());
    (bs, ds)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn statblocks_test() {
        let filename = "data/stats.json";
        let _ = get_statblocks(filename).expect("Failed to get stack blocks").get(&0).expect("Mssing zero-th statblock");
    }
}
