use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::common::*;
use crate::effect::{Effect, Traits};
use crate::stats::Stat;

#[derive(Serialize, Deserialize, Debug)]
pub struct Repeat<T> {
    pub rep: T,
    pub period: u16,
    #[serde(default)]
    pub number: Option<u16>,
}
type RepeatEffects = Vec<Repeat<IndexedOrLiteral<Effect>>>;
type RepeatHits = Vec<Repeat<Hit>>;

#[derive(Serialize, Deserialize, Debug)]
enum Visual {
    // https://docs.rs/termion/latest/termion/color/struct.AnsiValue.html
    // https://en.wikipedia.org/wiki/ANSI_escape_code
    AnsiValue(u8),
    //SpriteChange,
    //SpriteOverlay,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Condition {
    name: Name,
    #[serde(default)]
    pub duration: Option<u16>, // see Clocks in mod party
    //stopped_by: Traits, // Conditions?
    #[serde(default)]
    removed_by: Traits, // Effects that have one of these traits will remove the Condition
    //repeat_actions: RepeatActions, // needs targeting
    #[serde(default)]
    pub repeat_effects: RepeatEffects,
    #[serde(default)]
    pub repeat_hits: RepeatHits,
    #[serde(default)]
    mods: HashMap<Name, HashMap<Name, Stat>>, // e.g. {"clock": {"MultMod2": 0}} for K.O.
    //reactions: Map::<Trait, RelativeTargetedEffect>, // e.g. counter, reflect
    #[serde(default)]
    play_override: Option<PlayerType>,
    #[serde(default)]
    visual: Option<Visual>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TargetCondition {
    pub condition_id: Id, // IndexOrLiteral ?
    pub duration: u16,
    pub repeat_effect_countdowns: Vec<u16>,
    pub repeat_hit_countdowns: Vec<u16>,
}
pub type TargetConditions = Vec<TargetCondition>;
