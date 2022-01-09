use std::collections::HashMap;
use std::error::Error;

use serde::{Serialize, Deserialize};

use crate::common::{Id, Name, Formula};
use crate::encyclopedia::read_encyclopedia;


pub type Stat = i64;
pub type BaseStats = HashMap<Name, Stat>;
pub type DerivedStat = Formula;
pub type DerivedStats = HashMap<Name, DerivedStat>;

#[derive(Serialize, Deserialize, Debug)]
pub struct StatBlock {
    base_stats: BaseStats,
    stats: DerivedStats,
}

impl StatBlock {
    pub fn get_stat(&self, name: Name) -> Option<&DerivedStat> {
        self.stats.get(&name)
    }
}

// TODO: this is of course terrible
pub fn generate_stats(id: &Id) -> (BaseStats, DerivedStats) {
    let statblocks = read_encyclopedia::<StatBlock>("data/stats.json");
    let sb = statblocks.get(&id).unwrap();
    let (bs, ds) = (sb.base_stats.clone(), sb.stats.clone());
    (bs, ds)
}

#[cfg(test)]
mod tests {
    use super::*;
}
