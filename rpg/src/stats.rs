use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use serde::{Serialize, Deserialize};

use crate::common::{Id, Name, Formula};

pub type Stat = i64;
pub type BaseStats = HashMap<Name, Stat>;
pub type DerivedStat = Formula;
pub type DerivedStats = HashMap<Name, DerivedStat>;

#[derive(Serialize, Deserialize, Debug)]
pub struct StatBlock {
    base_stats: BaseStats,
    stats: DerivedStats,
}

pub type StatBlocks = HashMap<Id, StatBlock>;

// TODO: Compare this with encyclopedia
pub fn get_statblocks(filename: &str) -> Result<StatBlocks, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let en = serde_json::from_reader(reader)?;
    Ok(en)
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
