use std::collections::HashMap;

use crate::item;
use crate::item::EquipmentSet;
use crate::item::Item;

type _Stat = i32;
pub type Stat = _Stat;//Option<_Stat>;

pub type BaseStats = HashMap<String, _Stat>;

type DerivedStat = fn(&BaseStats, &EquipmentSet) -> Stat;

pub type DerivedStats = HashMap<String, DerivedStat>;

fn trivial_derived_stat(bs: &BaseStats, key_bs: String) -> Stat {
    *bs.get(&key_bs).unwrap()
}

fn default_derived_stat(
    bs: &BaseStats,
    eq: &EquipmentSet,
    key_bs: String,
    key_eq: String,
    key_ds: String,
) -> Stat {
    let base_stat: Stat = *bs.get(&key_bs).unwrap();
    let mult_mod: Stat = 1;  // todo, use something like Decimal
    let plus_mod: Stat = eq.into_iter().map(|(_n, i)| item::equipment_mod(&i, &key_bs)).sum();
    let eq_power: Stat = match eq.get(&key_eq) {
        Some(slot) => item::equipment_power(slot),
        None => item::equipment_power(&None::<Item>),
    };
    let ds_mult_mod: Stat = 1;  // todo, use something like Decimal
    let ds_plus_mod: Stat = eq.into_iter().map(|(_n, i)| item::equipment_mod(&i, &key_ds)).sum();
    (base_stat*mult_mod + plus_mod + eq_power)*ds_mult_mod + ds_plus_mod
}

/*  Later, this may become a method of an object BaseStatsGenerator
    that is instantiated using external game-specific data.
    Until then, we just hard-code the stat names and default values.
*/
// Similarly, we may later have a DerivedStatsGenerator
pub fn generate_stats() -> (BaseStats, DerivedStats) {
    let mut bs = BaseStats::new();
    bs.insert(String::from("Strength"), 10);
    bs.insert(String::from("Stamina"), 10);

    let mut ds = DerivedStats::new();
    ds.insert(String::from("Offense"), |bs, eq| default_derived_stat(bs, eq, String::from("Strength"), String::from("Weapon"), String::from("Offense")));
    ds.insert(String::from("Defense"), |bs, eq| default_derived_stat(bs, eq, String::from("Stamina"), String::from("Shield"), String::from("Defense")));
    /*
    for (bs_name, _) in &bs {
        ds.insert(bs_name.to_string(), |bs, eq| trivial_derived_stat(bs, bs_name.clone()));
    }
    */
    ds.insert(String::from("Strength"), |bs, _eq| trivial_derived_stat(bs, String::from("Strength")));
    ds.insert(String::from("Stamina"), |bs, _eq| trivial_derived_stat(bs, String::from("Stamina")));
    (bs, ds)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_test() {
        let eq: EquipmentSet = item::generate_equipment_set();
        let (bs, ds) = generate_stats();
        for (bs_name, bs_value) in &bs {
            assert_eq!(ds.get(bs_name).unwrap()(&bs, &eq), *bs_value);
        }
    }
}
